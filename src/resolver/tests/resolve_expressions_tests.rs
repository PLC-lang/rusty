use core::panic;

use crate::{
    ast::{AstStatement, DataType, UserTypeDeclaration},
    index::Index,
    resolver::{
        tests::{annotate, parse},
        AnnotationMap, StatementAnnotation,
    },
    typesystem::VOID_TYPE,
};

#[test]
fn binary_expressions_resolves_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            1 + 2;
            1 + 2000;
            2147483648 + 1;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["DINT", "DINT", "LINT"];

    let types: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(expected_types, types);
}

#[test]
fn unary_expressions_resolves_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            NOT TRUE;
            -(2+3);
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BOOL", "DINT"];

    let types: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(expected_types, types);
}

#[test]
fn binary_expressions_resolves_types_with_floats() {
    let (unit, index) = parse(
        "PROGRAM PRG
            1 + 2.2;
            1.1 + 2000;
            2000.0 + 1.0;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["REAL", "REAL", "REAL"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            expected_types[i],
            annotations.get_type_or_void(s, &index).get_name(),
            "{:#?}",
            s
        );
    }
}

#[test]
fn local_variables_resolves_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            VAR
                b : BYTE;
                w : WORD;
                dw : DWORD;
                lw : LWORD;
                si : SINT;
                usi : USINT;
                i : INT;
                ui : UINT;
                di : DINT;
                udi : UDINT;
                li : LINT;
                uli : ULINT;
            END_VAR

            b;
            w;
            dw;
            lw;
            si;
            usi;
            i;
            ui;
            di;
            udi;
            li;
            uli;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "BYTE", "WORD", "DWORD", "LWORD", "SINT", "USINT", "INT", "UINT", "DINT", "UDINT", "LINT",
        "ULINT",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn global_resolves_types() {
    let (unit, index) = parse(
        "
        VAR_GLOBAL
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
            si : SINT;
            usi : USINT;
            i : INT;
            ui : UINT;
            di : DINT;
            udi : UDINT;
            li : LINT;
            uli : ULINT;
        END_VAR
        
        PROGRAM PRG
            b;
            w;
            dw;
            lw;
            si;
            usi;
            i;
            ui;
            di;
            udi;
            li;
            uli;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "BYTE", "WORD", "DWORD", "LWORD", "SINT", "USINT", "INT", "UINT", "DINT", "UDINT", "LINT",
        "ULINT",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn global_initializers_resolves_types() {
    let (unit, index) = parse(
        "
        VAR_GLOBAL
            b : BYTE := 0;
            w : WORD := 0;
            dw : DWORD := 0;
            lw : LWORD := 0;
            si : SINT := 0;
            usi : USINT := 0;
            i : INT := 0;
            ui : UINT := 0;
            di : DINT := 0;
            udi : UDINT := 0;
            li : LINT := 0;
            uli : ULINT := 0;
        END_VAR
        ",
    );
    let annotations = annotate(&unit, &index);
    let statements: Vec<&AstStatement> = unit.global_vars[0]
        .variables
        .iter()
        .map(|it| it.initializer.as_ref().unwrap())
        .collect();

    let expected_types = vec![
        "DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "DINT",
        "DINT",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn resolve_binary_expressions() {
    let (unit, index) = parse(
        "
        VAR_GLOBAL
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
            si : SINT;
            usi : USINT;
            i : INT;
            ui : UINT;
            di : DINT;
            udi : UDINT;
            li : LINT;
            uli : ULINT;
        END_VAR
        
        PROGRAM PRG
            b + b;
            b + w;
            b + dw;
            b + lw;
            b + si;
            b + usi;
            b + i;
            b + ui;
            b + di;
            b + udi;
            b + li;
            b + uli;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "BYTE", "WORD", "DWORD", "LWORD", "SINT", "BYTE", "INT", "UINT", "DINT", "UDINT", "LINT",
        "ULINT",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn complex_expressions_resolve_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            VAR
                b : BYTE;
                w : WORD;
                dw : DWORD;
                lw : LWORD;
                si : SINT;
                usi : USINT;
                i : INT;
                ui : UINT;
                di : DINT;
                udi : UDINT;
                li : LINT;
                uli : ULINT;
                r : REAL;
            END_VAR

            b + w * di + li;
            b + w + di;
            b + w * di + r;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["LINT", "DINT", "REAL"];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn pointer_expressions_resolve_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            VAR
                i : REF_TO INT;
                y : REF_TO MyInt;
                a : MyIntRef;
                b : MyAliasRef;
            END_VAR

            i;
            i^;

            y;
            y^;

            a;
            a^;

            b;
            b^;
        END_PROGRAM
        
        TYPE MyInt: INT := 7; END_TYPE 
        TYPE MyIntRef: REF_TO INT; END_TYPE 
        TYPE MyAliasRef: REF_TO MyInt; END_TYPE 

        ",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "__PRG_i",
        "INT",
        "__PRG_y",
        "INT",
        "MyIntRef",
        "INT",
        "MyAliasRef",
        "INT",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}
#[test]
fn array_expressions_resolve_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            VAR
                i : ARRAY[0..10] OF INT;
                y : ARRAY[0..10] OF MyInt;
                a : MyIntArray;
                b : MyAliasArray;
            END_VAR

            i;
            i[2];

            y;
            y[2];

            a;
            a[2];

            b;
            b[2];
        END_PROGRAM
        
        TYPE MyInt: INT := 7; END_TYPE 
        TYPE MyIntArray: ARRAY[0..10] OF INT := 7; END_TYPE 
        TYPE MyAliasArray: ARRAY[0..10] OF MyInt := 7; END_TYPE 

        ",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "__PRG_i",
        "INT",
        "__PRG_y",
        "INT",
        "MyIntArray",
        "INT",
        "MyAliasArray",
        "INT",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn qualified_expressions_resolve_types() {
    let (unit, index) = parse(
        "
         PROGRAM Other
            VAR_INPUT
                b : BYTE;
                w : WORD;
                dw : DWORD;
                lw : LWORD;
            END_VAR
        END_PROGRAM   

        PROGRAM PRG
            Other.b;
            Other.w;
            Other.dw;
            Other.lw;
            Other.b + Other.w;
            Other.b + Other.w + Other.dw;
            Other.b + Other.w + Other.dw + Other.lw;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[1].statements;

    let expected_types = vec!["BYTE", "WORD", "DWORD", "LWORD", "WORD", "DWORD", "LWORD"];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn pou_expressions_resolve_types() {
    let (unit, index) = parse(
        "
        PROGRAM OtherPrg
        END_PROGRAM   

        FUNCTION OtherFunc : INT
        END_FUNCTION

        FUNCTION_BLOCK OtherFuncBlock
        END_FUNCTION_BLOCK

        PROGRAM PRG
            OtherPrg;
            OtherFunc;
            OtherFuncBlock;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[3].statements;

    //none of these pou's should really resolve to a type
    let expected_types = vec![VOID_TYPE, VOID_TYPE, VOID_TYPE];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();
    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));

    assert_eq!(
        Some(&StatementAnnotation::Program {
            qualified_name: "OtherPrg".into()
        }),
        annotations.get_annotation(&statements[0])
    );
    assert_eq!(
        Some(&StatementAnnotation::Function {
            qualified_name: "OtherFunc".into(),
            return_type: "INT".into()
        }),
        annotations.get_annotation(&statements[1])
    );
    assert_eq!(
        Some(&StatementAnnotation::Type {
            type_name: "OtherFuncBlock".into()
        }),
        annotations.get_annotation(&statements[2])
    );
}

#[test]
fn assignment_expressions_resolve_types() {
    let (unit, index) = parse(
        "
        PROGRAM PRG
            VAR
                x : INT;
                y : BYTE;
                z : LWORD;
            END_VAR

            x := y;
            z := x;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![VOID_TYPE, VOID_TYPE];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));

    if let AstStatement::Assignment { left, right, .. } = &statements[0] {
        assert_eq!(annotations.get_type_or_void(left, &index).get_name(), "INT");
        assert_eq!(
            annotations.get_type_or_void(right, &index).get_name(),
            "BYTE"
        );
    } else {
        panic!("expected assignment")
    }
    if let AstStatement::Assignment { left, right, .. } = &statements[1] {
        assert_eq!(
            annotations.get_type_or_void(left, &index).get_name(),
            "LWORD"
        );
        assert_eq!(
            annotations.get_type_or_void(right, &index).get_name(),
            "INT"
        );
    } else {
        panic!("expected assignment")
    }
}

#[test]
fn qualified_expressions_to_structs_resolve_types() {
    let (unit, index) = parse(
        "
        TYPE NextStruct: STRUCT
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
        END_STRUCT
        END_TYPE
 
        TYPE MyStruct: STRUCT
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
            next : NextStruct;
        END_STRUCT
        END_TYPE

        PROGRAM PRG
            VAR 
                mys : MyStruct;
            END_VAR
            mys;
            mys.b;
            mys.w;
            mys.dw;
            mys.lw;
            mys.next;
            mys.next.b;
            mys.next.w;
            mys.next.dw;
            mys.next.lw;
        END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "MyStruct",
        "BYTE",
        "WORD",
        "DWORD",
        "LWORD",
        "NextStruct",
        "BYTE",
        "WORD",
        "DWORD",
        "LWORD",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn qualified_expressions_to_inlined_structs_resolve_types() {
    let (unit, index) = parse(
        "
        PROGRAM PRG
            VAR 
                mys : STRUCT
                    b : BYTE;
                    w : WORD;
                    dw : DWORD;
                    lw : LWORD;
                END_STRUCT;
            END_VAR
            mys;
            mys.b;
            mys.w;
            mys.dw;
            mys.lw;
        END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["__PRG_mys", "BYTE", "WORD", "DWORD", "LWORD"];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn function_expression_resolves_to_the_function_itself_not_its_return_type() {
    //GIVEN a reference to a function
    let (unit, index) = parse(
        "
        FUNCTION foo : INT
        foo;
        END_FUNCTION

        PROGRAM PRG
            foo;
        END_PROGRAM
        ",
    );

    //WHEN the AST is annotated
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[1].statements;

    // THEN we expect it to be annotated with the function itself
    let foo_annotation = annotations.get_annotation(&statements[0]);
    assert_eq!(
        Some(&StatementAnnotation::Function {
            qualified_name: "foo".into(),
            return_type: "INT".into()
        }),
        foo_annotation
    );
    // AND we expect no type to be associated with the expression
    let associated_type = annotations.get_type(&statements[0], &index);
    assert_eq!(None, associated_type);

    let statements = &unit.implementations[0].statements;
    let foo_annotation = annotations.get_annotation(&statements[0]);
    assert_eq!(
        Some(&StatementAnnotation::Variable {
            qualified_name: "foo.foo".into(),
            resulting_type: "INT".into(),
            constant: false
        }),
        foo_annotation
    );
}

#[test]
fn function_call_expression_resolves_to_the_function_itself_not_its_return_type() {
    //GIVEN a reference to a function
    let (unit, index) = parse(
        "
        FUNCTION foo : INT
        END_FUNCTION

        PROGRAM PRG
            foo();
        END_PROGRAM
        ",
    );

    //WHEN the AST is annotated
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[1].statements;

    // THEN we expect it to be annotated with the function itself
    let foo_annotation = annotations.get_annotation(&statements[0]);
    assert_eq!(
        Some(&StatementAnnotation::Value {
            resulting_type: "INT".into()
        }),
        foo_annotation
    );

    // AND we expect no type to be associated with the expression
    let associated_type = annotations.get_type(&statements[0], &index);
    assert_eq!(index.find_type("INT"), associated_type);
}

#[test]
fn qualified_expressions_to_aliased_structs_resolve_types() {
    let (unit, index) = parse(
        "
        TYPE NextStruct: STRUCT
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
        END_STRUCT
        END_TYPE
 
        TYPE MyStruct: STRUCT
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
            next : AliasedNextStruct;
        END_STRUCT
        END_TYPE

        TYPE AliasedMyStruct : MyStruct; END_TYPE
        TYPE AliasedNextStruct : NextStruct; END_TYPE

        PROGRAM PRG
            VAR 
                mys : AliasedMyStruct;
            END_VAR
            mys;
            mys.b;
            mys.w;
            mys.dw;
            mys.lw;
            mys.next;
            mys.next.b;
            mys.next.w;
            mys.next.dw;
            mys.next.lw;
        END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "MyStruct",
        "BYTE",
        "WORD",
        "DWORD",
        "LWORD",
        "NextStruct",
        "BYTE",
        "WORD",
        "DWORD",
        "LWORD",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn qualified_expressions_to_fbs_resolve_types() {
    let (unit, index) = parse(
        "
        FUNCTION_BLOCK MyFb
            VAR_INPUT
                fb_b : SINT;
                fb_i : INT;
                fb_d : DINT;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM PRG
            VAR 
                fb : MyFb;
            END_VAR
            fb;
            fb.fb_b;
            fb.fb_i;
            fb.fb_d;
       END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[1].statements;

    let expected_types = vec!["MyFb", "SINT", "INT", "DINT"];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn qualified_expressions_dont_fallback_to_globals() {
    let (unit, index) = parse(
        "
        VAR_GLOBAL
            x : DINT;
        END_VAR 

        TYPE MyStruct: STRUCT
            y : INT;
        END_STRUCT
        END_TYPE

        PROGRAM PRG
            VAR P : MyStruct; END_VAR
            P.x;
            P.y;
        END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    assert_eq!(None, annotations.get_annotation(&statements[0]));
    assert_eq!(
        Some(&StatementAnnotation::Variable {
            qualified_name: "MyStruct.y".into(),
            resulting_type: "INT".into(),
            constant: false
        }),
        annotations.get_annotation(&statements[1])
    );
}

#[test]
fn function_parameter_assignments_resolve_types() {
    let (unit, index) = parse(
        "
        FUNCTION foo : MyType
            VAR_INPUT
                x : INT;
            END_VAR
            VAR_OUTPUT
                y : SINT;
            END_VAR
        END_FUNCTION

        PROGRAM PRG
            foo(x := 3, y => 6);
        END_PROGRAM
        
        TYPE MyType: INT; END_TYPE
        ",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[1].statements;

    assert_eq!(
        annotations
            .get_type_or_void(&statements[0], &index)
            .get_name(),
        "INT"
    );
    assert_eq!(
        annotations.get_annotation(&statements[0]),
        Some(&StatementAnnotation::expression("INT"))
    );
    if let AstStatement::CallStatement {
        operator,
        parameters,
        ..
    } = &statements[0]
    {
        //make sure the call's operator resolved correctly
        assert_eq!(
            annotations.get_type_or_void(operator, &index).get_name(),
            VOID_TYPE
        );
        assert_eq!(
            annotations.get_annotation(operator),
            Some(&StatementAnnotation::Function {
                qualified_name: "foo".into(),
                return_type: "MyType".into()
            })
        );

        if let Some(AstStatement::ExpressionList { expressions, .. }) = &**parameters {
            if let AstStatement::Assignment { left, right, .. } = &expressions[0] {
                assert_eq!(annotations.get_type_or_void(left, &index).get_name(), "INT");
                assert_eq!(
                    annotations.get_type_or_void(right, &index).get_name(),
                    "DINT"
                );
            } else {
                panic!("assignment expected")
            }
            if let AstStatement::OutputAssignment { left, right, .. } = &expressions[1] {
                assert_eq!(
                    annotations.get_type_or_void(left, &index).get_name(),
                    "SINT"
                );
                assert_eq!(
                    annotations.get_type_or_void(right, &index).get_name(),
                    "DINT"
                );
            } else {
                panic!("assignment expected")
            }
        } else {
            panic!("expression list expected")
        }
    } else {
        panic!("call statement");
    }
}

#[test]
fn nested_function_parameter_assignments_resolve_types() {
    let (unit, index) = parse(
        "
        FUNCTION foo : INT
            VAR_INPUT
                x : INT;
                y : BOOL;
            END_VAR
        END_FUNCTION

        FUNCTION baz : DINT
            VAR_INPUT
                x : DINT;
                y : DINT;
            END_VAR
        END_FUNCTION


        PROGRAM PRG
            VAR r: REAL; END_VAR
            foo(x := baz(x := 200, y := FALSE), y := baz(x := 200, y := TRUE) + r);
        END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[2].statements;
    if let AstStatement::CallStatement { parameters, .. } = &statements[0] {
        //check the two parameters
        assert_parameter_assignment(parameters, 0, "INT", "DINT", &annotations, &index);
        assert_parameter_assignment(parameters, 1, "BOOL", "REAL", &annotations, &index);

        //check the inner call in the first parameter assignment of the outer call `x := baz(...)`
        if let AstStatement::Assignment { right, .. } = get_expression_from_list(parameters, 0) {
            if let AstStatement::CallStatement { parameters, .. } = right.as_ref() {
                // the left side here should be `x` - so lets see if it got mixed up with the outer call's `x`
                assert_parameter_assignment(parameters, 0, "DINT", "DINT", &annotations, &index);
            } else {
                panic!("inner call")
            }
        } else {
            panic!("assignment");
        }
    } else {
        panic!("call statement")
    }
}

#[test]
fn type_initial_values_are_resolved() {
    let (unit, index) = parse(
        "
        TYPE MyStruct : STRUCT
            x : INT := 20;
            y : BOOL := TRUE;
            z : STRING := 'abc';
        END_STRUCT
        END_TYPE
        ",
    );

    let annotations = annotate(&unit, &index);
    let UserTypeDeclaration { data_type, .. } = &unit.types[0];

    if let DataType::StructType { variables, .. } = data_type {
        assert_eq!(
            Some(&StatementAnnotation::expression("DINT")),
            annotations.get(variables[0].initializer.as_ref().unwrap())
        );
        assert_eq!(
            Some(&StatementAnnotation::expression("BOOL")),
            annotations.get(variables[1].initializer.as_ref().unwrap())
        );
        assert_eq!(
            Some(&StatementAnnotation::expression("STRING")),
            annotations.get(variables[2].initializer.as_ref().unwrap())
        );
    } else {
        panic!("no datatype: {:#?}", data_type)
    }
}

#[test]
fn actions_are_resolved() {
    let (unit, index) = parse(
        "
        PROGRAM prg
            foo;
            prg.foo;
        END_PROGRAM
        ACTIONS prg
        ACTION foo 
        END_ACTION
        END_ACTIONS

        FUNCTION buz : INT
        prg.foo();
        prg.foo;
        END_FUNCTION
        ",
    );

    let annotations = annotate(&unit, &index);
    let foo_reference = &unit.implementations[0].statements[0];
    let annotation = annotations.get_annotation(foo_reference);
    assert_eq!(
        Some(&StatementAnnotation::Program {
            qualified_name: "prg.foo".into(),
        }),
        annotation
    );
    let foo_reference = &unit.implementations[0].statements[1];
    let annotation = annotations.get_annotation(foo_reference);
    assert_eq!(
        Some(&StatementAnnotation::Program {
            qualified_name: "prg.foo".into(),
        }),
        annotation
    );
    let method_call = &unit.implementations[2].statements[0];
    if let AstStatement::CallStatement { operator, .. } = method_call {
        assert_eq!(
            Some(&StatementAnnotation::Program {
                qualified_name: "prg.foo".into(),
            }),
            annotations.get(operator)
        );
        assert_eq!(None, annotations.get(method_call));
    } else {
        panic!("Unexpcted statemet : {:?}", method_call);
    }
}
#[test]
fn method_references_are_resolved() {
    let (unit, index) = parse(
        "
        CLASS cls
        METHOD foo : INT
            foo;
        END_METHOD
        END_CLASS

        FUNCTION buz : INT
        VAR cl : cls; END_VAR
        cl.foo();
        END_FUNCTION
        ",
    );

    let annotations = annotate(&unit, &index);
    let foo_reference = &unit.implementations[0].statements[0];
    let annotation = annotations.get_annotation(foo_reference);
    assert_eq!(
        Some(&StatementAnnotation::Variable {
            qualified_name: "cls.foo.foo".into(),
            resulting_type: "INT".into(),
            constant: false
        }),
        annotation
    );
    let method_call = &unit.implementations[1].statements[0];
    if let AstStatement::CallStatement { operator, .. } = method_call {
        assert_eq!(
            Some(&StatementAnnotation::Function {
                return_type: "INT".into(),
                qualified_name: "cls.foo".into(),
            }),
            annotations.get(operator)
        );
        assert_eq!(
            Some(&StatementAnnotation::expression("INT")),
            annotations.get(method_call)
        );
    } else {
        panic!("Unexpcted statemet : {:?}", method_call);
    }
}

#[test]
fn bitaccess_is_resolved() {
    let (unit, index) = parse(
        r"
    PROGRAM prg
        VAR
            a,b,c,d,e : INT;
        END_VAR
        a.0;
        b.%X1;
        c.%B2;
        d.%W3;
        e.%D4;
    END_PROGRAM
    ",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BOOL", "BOOL", "BYTE", "WORD", "DWORD"];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn variable_direct_access_type_resolved() {
    let (unit, index) = parse(
        r"
    PROGRAM prg
        VAR
            a : INT;
            b : REAL;
            c : LREAL;
        END_VAR
        a.%Xa;
        a.%Xb;
        a.%Xc;
    END_PROGRAM
    ",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["INT", "REAL", "LREAL"];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| {
            if let AstStatement::QualifiedReference { elements, .. } = s {
                if let AstStatement::DirectAccess { index, .. } = elements.last().unwrap() {
                    return index;
                }
            }
            panic!("Wrong type {:?}", s);
        })
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

fn get_expression_from_list(stmt: &Option<AstStatement>, index: usize) -> &AstStatement {
    if let Some(AstStatement::ExpressionList { expressions, .. }) = stmt {
        &expressions[index]
    } else {
        panic!("no expression_list, found {:#?}", stmt)
    }
}

fn assert_parameter_assignment(
    parameters: &Option<AstStatement>,
    param_index: usize,
    left_type: &str,
    right_type: &str,
    annotations: &AnnotationMap,
    index: &Index,
) {
    if let Some(AstStatement::ExpressionList { expressions, .. }) = parameters {
        if let AstStatement::Assignment { left, right, .. } = &expressions[param_index] {
            assert_eq!(
                annotations.get_type_or_void(left, index).get_name(),
                left_type
            );
            assert_eq!(
                annotations.get_type_or_void(right, index).get_name(),
                right_type
            );
        } else {
            panic!("assignment expected")
        }
    } else {
        panic!("expression list expected")
    }
}

#[test]
fn const_flag_is_calculated_when_resolving_simple_references() {
    let (unit, index) = parse(
        "
        VAR_GLOBAL CONSTANT
            cg : INT := 1;
        END_VAR
        
        VAR_GLOBAL
            g : INT := 1;
        END_VAR

        PROGRAM PRG
            VAR CONSTANT
                cl : INT;
            END_VAR

            VAR 
                l : INT;
            END_VAR

            cg;
            g;
            cl;
            l;
       END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_consts = vec![true, false, true, false];
    let actual_consts: Vec<bool> = statements
        .iter()
        .map(|s| {
            if let Some(StatementAnnotation::Variable { constant, .. }) = annotations.get(s) {
                *constant
            } else {
                unreachable!()
            }
        })
        .collect();

    assert_eq!(
        format!("{:?}", expected_consts),
        format!("{:?}", actual_consts)
    );
}

#[test]
fn const_flag_is_calculated_when_resolving_qualified_variables() {
    let (unit, index) = parse(
        "
        TYPE NextStruct: STRUCT
            b : BYTE;
        END_STRUCT
        END_TYPE
 
        TYPE MyStruct: STRUCT
            b : BYTE;
            next : NextStruct;
        END_STRUCT
        END_TYPE

        PROGRAM PRG
            VAR 
                mys : MyStruct;
            END_VAR
            VAR CONSTANT 
                cmys : MyStruct;
            END_VAR

            cmys.b;
            mys.b;
            cmys.next.b;
            mys.next.b;
        END_PROGRAM",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_consts = vec![true, false, true, false];
    let actual_consts: Vec<bool> = statements
        .iter()
        .map(|s| {
            if let Some(StatementAnnotation::Variable { constant, .. }) = annotations.get(s) {
                *constant
            } else {
                unreachable!()
            }
        })
        .collect();

    assert_eq!(
        format!("{:?}", expected_consts),
        format!("{:?}", actual_consts)
    );
}

#[test]
fn const_flag_is_calculated_when_resolving_qualified_variables_over_prgs() {
    let (unit, index) = parse(
        "
        TYPE NextStruct: STRUCT
            b : BYTE;
        END_STRUCT
        END_TYPE
 
        TYPE MyStruct: STRUCT
            b : BYTE;
            next : NextStruct;
        END_STRUCT
        END_TYPE

        PROGRAM PRG
            other.mys.next.b;
            other.cmys.next.b;
        END_PROGRAM
        
        PROGRAM other
            VAR 
                mys : MyStruct;
            END_VAR
            VAR CONSTANT 
                cmys : MyStruct;
            END_VAR

        END_PROGRAM
        ",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_consts = vec![false, true];
    let actual_consts: Vec<bool> = statements
        .iter()
        .map(|s| {
            if let Some(StatementAnnotation::Variable { constant, .. }) = annotations.get(s) {
                *constant
            } else {
                unreachable!()
            }
        })
        .collect();

    assert_eq!(
        format!("{:?}", expected_consts),
        format!("{:?}", actual_consts)
    );
}

#[test]
fn const_flag_is_calculated_when_resolving_enum_literals() {
    let (unit, index) = parse(
        "
        TYPE Color: (red, green, yellow);
        END_TYPE
                
        PROGRAM other
            VAR 
                state: (OPEN, CLOSE);
            END_VAR
            red;
            green;
            OPEN;
            state;
        END_PROGRAM
        ",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_consts = vec![true, true, true, false];
    let actual_consts: Vec<bool> = statements
        .iter()
        .map(|s| {
            if let Some(StatementAnnotation::Variable { constant, .. }) = annotations.get(s) {
                *constant
            } else {
                unreachable!()
            }
        })
        .collect();

    assert_eq!(
        format!("{:?}", expected_consts),
        format!("{:?}", actual_consts)
    );
}
