use crate::{
    ast::AstStatement,
    index::VariableType,
    lexer::IdProvider,
    resolver::{AnnotationMap, StatementAnnotation},
    test_utils::tests::{annotate_with_ids, index_with_ids},
};

use super::util_macros::{
    annotate, deconstruct_assignment, deconstruct_binary_expression, deconstruct_call_statement,
    deconstruct_qualified_reference,
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
        b := 4;
    END_PROGRAM

    VAR_GLOBAL constant
        gX : INT := 7;
    END_VAR
    "#
    );

    // lets look at a := gX;
    let (a, g_x) = deconstruct_assignment!(&statements[0]);
    // the left side is a reference to the Variable prg.a
    assert_eq!(
        annotations.get(a).unwrap(),
        &StatementAnnotation::Variable {
            resulting_type: "SINT".into(),
            qualified_name: "prg.a".into(),
            constant: false,
            variable_type: VariableType::Local,
            is_auto_deref: false
        }
    );

    // the right side is a reference to the global variable gX
    assert_eq!(
        annotations.get(g_x).unwrap(),
        &StatementAnnotation::Variable {
            resulting_type: "INT".into(),
            qualified_name: "gX".into(),
            constant: true,
            variable_type: VariableType::Global,
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

    // now lets take a look at several annotations that got placed:
    // p.x resolves into a variable, a Variable annotation indicates an LValue
    assert_eq!(
        annotations.get(&statements[0]),
        Some(&StatementAnnotation::Variable {
            qualified_name: "POINT.x".into(),   // the qualified name of the target element
            resulting_type: "SINT".into(),      // the variable's type
            constant: false,                    // whether this variable is a constant or not
            is_auto_deref: false, // whether this pointerType should be automatically dereferenced
            variable_type: VariableType::Input  // the type of declaration
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

    // p.y + foo() resolves to an INT (bigger of the two)
    assert_eq!(
        annotations.get(&statements[2]),
        Some(&StatementAnnotation::Value { resulting_type: "DINT".into() })
    );

    // Main.in
    let segments = deconstruct_qualified_reference!(&statements[3]);
    assert_eq!(
        annotations.get(&segments[0]), //Main resolves to a Program
        Some(&StatementAnnotation::Program { qualified_name: "Main".into() })
    );
    assert_eq!(
        annotations.get(&segments[1]), //in resolves to the variable in
        Some(&StatementAnnotation::Variable {
            qualified_name: "Main.in".into(),
            resulting_type: "INT".into(),
            constant: false,
            is_auto_deref: false,
            variable_type: VariableType::Input
        })
    );
    // the qualified statement gets the annotation of the last segment
    assert_eq!(annotations.get(&statements[3]), annotations.get(&segments[1]));
}

/// The resolver (the component that annotates the AST) not only annnotates the real datatype
/// of expressions, it also annotates a hint on what datatype needs to be generated for an
/// expression. This means that the resolver not only analyzes the real type of a declared element,
/// it only determines the type of an evaluated expression, and even hints necessary casts
/// (e.g. when assigning an INT to a DINT-variable). The TypeAnnotations offers a helper function
/// `get_type(&AstStatement)` that extracts the resulting-type from the statement's annotation.
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
/// - Type-Annotation: the real type of this expression. These Type-Hints work for all kinds of
/// type necessary casts or promotions, including:
///  - casting a variable due to an assignment to a differently-typed variable (e.g. `myInt := mySInt;`)
///  - casting a variable when passing it to a function that expects a different type (e.g. `foo(mySInt);` mySInt
///         gets the type-hint of the 1st foo-parameter)
///  - automatic upscaling expressions in binary-expressions (e.g. `myInt + mySInt;` mySInt will receive a
///         typehint of "INT" so the addition can be performed as `INT + INT`)
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

    // k := d
    let (_, parameters) = deconstruct_call_statement!(&statements[2]);
    // left has no special type-hint
    assert_eq!(annotations.get_hint(parameters[0]), Some(&StatementAnnotation::value("DINT"))); //numeric literals are treated as DINTs
    assert_eq!(
        annotations.get_hint(parameters[0]),
        Some(&StatementAnnotation::Value { resulting_type: "INT".into() }) // the parameter's hint is "INT" because it needs to be casted when passed
    );
}
