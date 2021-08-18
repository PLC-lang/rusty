use core::panic;

use crate::{
    ast::{DataType, Statement, UserTypeDeclaration},
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
        Some(&StatementAnnotation::ProgramAnnotation {
            qualified_name: "OtherPrg".into()
        }),
        annotations.get_annotation(&statements[0])
    );
    assert_eq!(
        Some(&StatementAnnotation::FunctionAnnotation {
            qualified_name: "OtherFunc".into(),
            return_type: "INT".into()
        }),
        annotations.get_annotation(&statements[1])
    );
    assert_eq!(
        Some(&StatementAnnotation::TypeAnnotation {
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

    if let Statement::Assignment { left, right, .. } = &statements[0] {
        assert_eq!(annotations.get_type_or_void(left, &index).get_name(), "INT");
        assert_eq!(
            annotations.get_type_or_void(right, &index).get_name(),
            "BYTE"
        );
    } else {
        panic!("expected assignment")
    }
    if let Statement::Assignment { left, right, .. } = &statements[1] {
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
        Some(&StatementAnnotation::FunctionAnnotation {
            qualified_name: "foo".into(),
            return_type: "INT".into()
        }),
        foo_annotation
    );

    // AND we expect no type to be associated with the expression
    let associated_type = annotations.get_type(&statements[0], &index);
    assert_eq!(None, associated_type);
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
        Some(&StatementAnnotation::ExpressionAnnotation {
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
        Some(&StatementAnnotation::VariableAnnotation {
            qualified_name: "MyStruct.y".into(),
            resulting_type: "INT".into()
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
    if let Statement::CallStatement {
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
            Some(&StatementAnnotation::FunctionAnnotation {
                qualified_name: "foo".into(),
                return_type: "MyType".into()
            })
        );

        if let Some(Statement::ExpressionList { expressions, .. }) = &**parameters {
            if let Statement::Assignment { left, right, .. } = &expressions[0] {
                assert_eq!(annotations.get_type_or_void(left, &index).get_name(), "INT");
                assert_eq!(
                    annotations.get_type_or_void(right, &index).get_name(),
                    "DINT"
                );
            } else {
                panic!("assignment expected")
            }
            if let Statement::OutputAssignment { left, right, .. } = &expressions[1] {
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
    if let Statement::CallStatement { parameters, .. } = &statements[0] {
        //check the two parameters
        assert_parameter_assignment(parameters, 0, "INT", "DINT", &annotations, &index);
        assert_parameter_assignment(parameters, 1, "BOOL", "REAL", &annotations, &index);

        //check the inner call in the first parameter assignment of the outer call `x := baz(...)`
        if let Statement::Assignment { right, .. } = get_expression_from_list(parameters, 0) {
            if let Statement::CallStatement { parameters, .. } = right.as_ref() {
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

fn get_expression_from_list(stmt: &Option<Statement>, index: usize) -> &Statement {
    if let Some(Statement::ExpressionList { expressions, .. }) = stmt {
        &expressions[index]
    } else {
        panic!("no expression_list, found {:#?}", stmt)
    }
}

fn assert_parameter_assignment(
    parameters: &Option<Statement>,
    param_index: usize,
    left_type: &str,
    right_type: &str,
    annotations: &AnnotationMap,
    index: &Index,
) {
    if let Some(Statement::ExpressionList { expressions, .. }) = parameters {
        if let Statement::Assignment { left, right, .. } = &expressions[param_index] {
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
