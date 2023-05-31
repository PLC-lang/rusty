use plc_ast::ast::{AstStatement, ReferenceAccess, ReferenceExpr};

use crate::{
    index::{ArgumentType, VariableType},
    resolver::{AnnotationMap, StatementAnnotation},
    test_utils::tests::{annotate_with_ids, index_with_ids},
};

use super::util_macros::{
    annotate, deconstruct_assignment, deconstruct_binary_expression, deconstruct_call_statement,
};

/// # Architecture Design Record: Annotated AST
///
/// The rusty parser creates an AST. After everything was indexed, the AST is revisited and
/// `annotated`. While the AST itself only represents the source code from a syntactical point
/// of view, the annotations add semantics to single AST-elements. The semantic information
/// allows for much easier validation and code-generation.
///
/// With the help of annotations we can easily find out the target of references,
/// and their types.
#[test]
fn references_to_variables_are_annotated() {
    // parse and index
    let (statements, annotations, ..) = annotate!(
        r#"
    PROGRAM prg 
        VAR
            a : SINT;
            b : SINT;
        END_VAR

        a := gX;
    END_PROGRAM

    VAR_GLOBAL constant
        gX : INT := 7;
    END_VAR
    "#
    );

    // lets look at a := gX;
    let (a, g_x) = deconstruct_assignment!(&statements[0]);
    // the left side is a reference to the Variable prg.a of type SINT
    assert_eq!(
        annotations.get(a).unwrap(),
        &StatementAnnotation::Variable {
            resulting_type: "SINT".into(),
            qualified_name: "prg.a".into(),
            constant: false,
            argument_type: ArgumentType::ByVal(VariableType::Local),
            is_auto_deref: false
        }
    );

    // the right side is a reference to the global variable gX of type INT
    assert_eq!(
        annotations.get(g_x).unwrap(),
        &StatementAnnotation::Variable {
            resulting_type: "INT".into(),
            qualified_name: "gX".into(),
            constant: true,
            argument_type: ArgumentType::ByVal(VariableType::Global),
            is_auto_deref: false
        }
    );
}

/// The whole body of a POU is annotated. Annotations tell you whether a reference resolves to a
/// variable, a POU, a value or a call. They can be used to better decide what code to generate,
/// or to easily validate different scenarios.
#[test]
fn different_types_of_annotations() {
    // parse and annotate the following program
    let (statements, annotations, ..) = annotate!(
        r#"
        PROGRAM prg 
            VAR p : POINT END_VAR
 
            p.x;            // to a variable (POINT.x)
            foo();          // resolves to a call
            p.y + foo();    // resolves to a value
            Main.in;        // resolves to a variable (Main.in)
        END_PROGRAM

        FUNCTION foo : DINT END_FUNCTION

        PROGRAM Main 
            VAR_INPUT in : INT; END_VAR
        END_FUNCTION

        TYPE POINT:
            STRUCT
                x,y : SINT;
            END_STRUCT
        END_POINT
    "#
    );

    // p.x resolves to a variable, a Variable annotation indicates an LValue
    assert_eq!(
        annotations.get(&statements[0]),
        Some(&StatementAnnotation::Variable {
            qualified_name: "POINT.x".into(), // the qualified name of the target element
            resulting_type: "SINT".into(),    // the variable's type
            constant: false,                  // whether this variable is a constant or not
            is_auto_deref: false,             // whether this pointerType should be automatically dereferenced
            argument_type: ArgumentType::ByVal(VariableType::Input), // the type of declaration
        })
    );

    // foo()
    let (operator, ..) = deconstruct_call_statement!(&statements[1]);
    // the call's operator 'foo' resolves to the function
    assert_eq!(
        annotations.get(operator.as_ref()),
        Some(&StatementAnnotation::Function {
            return_type: "DINT".into(),
            qualified_name: "foo".into(),
            call_name: None
        })
    );
    // the whole call resolves to a value, a value indicates an RValue
    assert_eq!(
        annotations.get(&statements[1]),
        Some(&StatementAnnotation::Value { resulting_type: "DINT".into() })
    );

    // p.y + foo() resolves to a value of type INT (the bigger type of the two)
    assert_eq!(
        annotations.get(&statements[2]),
        Some(&StatementAnnotation::Value { resulting_type: "DINT".into() })
    );

    // Main.in
    let qualified_reference = &statements[3];
    let AstStatement::ReferenceExpr(ReferenceExpr {
        access: ReferenceAccess::Member(member),
        base: Some(qualifier),
    }) = qualified_reference.get_stmt()
    else {
        unreachable!()
    };
    // // Main resolves to a Program
    assert_eq!(
        annotations.get(qualifier),
        Some(&StatementAnnotation::Program { qualified_name: "Main".into() })
    );
    // in resolves to the member variable
    assert_eq!(
        annotations.get(member),
        Some(&StatementAnnotation::Variable {
            qualified_name: "Main.in".into(),
            resulting_type: "INT".into(),
            constant: false,
            is_auto_deref: false,
            argument_type: ArgumentType::ByVal(VariableType::Input),
        })
    );

    // the whole Main.in also resolves to the member variable
    assert_eq!(
        annotations.get(qualified_reference),
        Some(&StatementAnnotation::Variable {
            qualified_name: "Main.in".into(),
            resulting_type: "INT".into(),
            constant: false,
            is_auto_deref: false,
            argument_type: ArgumentType::ByVal(VariableType::Input),
        })
    );
}

/// The resolver (the component that annotates the AST) not only annnotates the real datatype
/// of expressions, it also annotates a hint on what datatype needs to be generated for an
/// expression. This means that the resolver not only analyzes the real type of a declared element,
/// it also determines the type of an evaluated expression and even hints necessary casts
/// (e.g. when assigning an INT to a DINT-variable). The TypeAnnotations offers a helper function
/// `get_type(&AstStatement)` that extracts the resulting-type from the statement's annotation -
/// whatever type of annotation it is.
#[test]
fn type_annotations_reflect_the_evaluation_result() {
    let (statements, annotations, idx) = annotate!(
        r#"
        PROGRAM prg 
            VAR 
                i : INT; 
                d : DINT;
                l : LINT;
            END_VAR
        
            i + d;
            i + 3;
            l - d;
        END_PROGRAM
    "#
    );

    // i + d evaluates to DINT (the bigger of the two)
    assert_eq!(annotations.get_type(&statements[0], &idx), idx.find_effective_type_by_name("DINT"));
    // i + 3 evaluates to DINT (because the literal 3 is treated as a DINT)
    assert_eq!(annotations.get_type(&statements[1], &idx), idx.find_effective_type_by_name("DINT"));
    // l - d evaluates to LINT (the bigger of the two)
    assert_eq!(annotations.get_type(&statements[2], &idx), idx.find_effective_type_by_name("LINT"));
}

/// An expression gets a type-annotation and a type-hint annotation. The meaning is the following:
/// - Type-Annotation: the real type of this expression.
/// - Type Hint: the type that should be used to treat this expression.
///
/// If the Type-Hint differs from the Type-Annotation the resolver indicates that we need a cast
/// from the Type to the Type-hint.
///
/// These Type-Hints work for all kinds of necessary casts or promotions, including:
///  - casting a variable due to an assignment to a differently-typed variable (e.g. `myInt := mySInt;`)
///  - casting a variable when passing it to a function that expects a different type (e.g. `foo(mySInt);` mySInt
///         gets the type-hint of the 1st foo-parameter)

#[test]
fn type_annotations_indicates_necessary_casts() {
    let (statements, annotations, _) = annotate!(
        r#"
        PROGRAM prg 
            VAR 
                i : INT; 
                d : DINT;
            END_VAR
        
            i := d;
            d := i;
            foo(3);
        END_PROGRAM

        FUNCTION foo : INT
            VAR_INPUT in : INT END_VAR
        END_VAR
    "#
    );

    // i := d
    let (left, right) = deconstruct_assignment!(&statements[0]);
    // left has no special type-hint
    assert_eq!(annotations.get_hint(left), None);
    // right is hinted as "INT" because it needs to be casted to an INT to be assigned to the variable i
    assert_eq!(
        annotations.get_hint(right),
        Some(&StatementAnnotation::Value { resulting_type: "INT".into() })
    );

    // d := i
    let (left, right) = deconstruct_assignment!(&statements[1]);
    // left has no special type-hint
    assert_eq!(annotations.get_hint(left), None);
    // right is hinted as "DINT" because it needs to be casted to a DINT to be assigned to the variable d
    assert_eq!(
        annotations.get_hint(right),
        Some(&StatementAnnotation::Value { resulting_type: "DINT".into() })
    );

    // foo(3)
    let (_, parameters) = deconstruct_call_statement!(&statements[2]);
    // 3 is a DINT but hinted as an INT because foo expects an INT as first parameter
    assert_eq!(annotations.get_hint(parameters[0]), Some(&StatementAnnotation::value("INT")));
}

/// The resolver also hints necessary upscaling of operands in binary-expressions (e.g. `myInt + mySInt;`
/// mySInt will receive a typehint of "INT" so the addition can be performed as `INT + INT`);
/// It also correctly resolves the result of such expressions
#[test]
fn type_annotations_for_binary_expression() {
    let (statements, annotations, _) = annotate!(
        r#"
        PROGRAM prg 
            VAR 
                i : INT; 
                d : DINT;
                r : LREAL;
            END_VAR
        
            i + d;
            r / d;
        END_PROGRAM
    "#
    );

    // i + d
    let (left, right) = deconstruct_binary_expression!(&statements[0]);
    // left needs upscale to match the right type-size
    assert_eq!(annotations.get_hint(left), Some(&StatementAnnotation::value("DINT")));
    // right is the bigger type and needs no hint
    assert_eq!(annotations.get_hint(right), None);
    // the whole expression (i + d) results in an DINT
    assert_eq!(annotations.get(&statements[0]), Some(&StatementAnnotation::value("DINT")));

    // r / d
    let (left, right) = deconstruct_binary_expression!(&statements[1]);
    // left is the bigger type and needs no hint
    assert_eq!(annotations.get_hint(left), None);
    // right needs upscale to match the left type-size
    assert_eq!(annotations.get_hint(right), Some(&StatementAnnotation::value("LREAL")));
    // the whole expression (i / d) results in an LREAL
    assert_eq!(annotations.get(&statements[1]), Some(&StatementAnnotation::value("LREAL")));
}

/// In general one can say, that an expression's type can be derived like this:
///  1. get the expression's type-hint-annotation,
///  2. if there is no type-hint-annotation check the type-annotation
#[test]
fn useful_type_annotation_method() {
    let (statements, annotations, idx) = annotate!(
        r#"
        PROGRAM prg 
            VAR 
                i : INT; 
                d : DINT;
            END_VAR
        
            i + d;
        END_PROGRAM
    "#
    );

    // the get_type
    assert_eq!(
        annotations.get_type_or_void(&statements[0], &idx),
        idx.get_effective_type_or_void_by_name("DINT")
    );
}
