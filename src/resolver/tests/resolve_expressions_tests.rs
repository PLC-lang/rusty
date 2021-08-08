use core::panic;

use crate::{
    ast::Statement,
    resolver::{
        tests::{annotate, parse},
        AnnotationMap,
    },
};

#[test]
fn binary_expressions_resolves_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            1 + 2;
            1 + 2000;
            2000 + 1;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BYTE", "UINT", "UINT"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            Some(&expected_types[i].to_string()),
            annotations.type_map.get(&s.get_id()),
            "{:#?}",
            s
        );
    }
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
            Some(&expected_types[i].to_string()),
            annotations.type_map.get(&s.get_id()),
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn pou_expressions_resolve_types() {
    let (unit, index) = parse(
        "
        PROGRAM OtherPrg
        END_PROGRAM   

        FUNCTION OtherFunc
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

    let expected_types = vec!["OtherPrg", "OtherFunc", "OtherFuncBlock"];
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
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

    let nothing = "-".to_string();
    let expected_types = vec![&nothing, &nothing];
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));

    if let Statement::Assignment { left, right, .. } = &statements[0] {
        assert_eq!(
            annotations.type_map.get(&left.get_id()),
            Some(&"INT".to_string())
        );
        assert_eq!(
            annotations.type_map.get(&right.get_id()),
            Some(&"BYTE".to_string())
        );
    } else {
        panic!("expected assignment")
    }
    if let Statement::Assignment { left, right, .. } = &statements[1] {
        assert_eq!(
            annotations.type_map.get(&left.get_id()),
            Some(&"LWORD".to_string())
        );
        assert_eq!(
            annotations.type_map.get(&right.get_id()),
            Some(&"INT".to_string())
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
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
    let nothing = "-".to_string();
    let type_names: Vec<&String> = statements
        .iter()
        .map(|s| annotations.type_map.get(&s.get_id()).unwrap_or(&nothing))
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn function_parameter_assignments_resolve_types() {
    let (unit, index) = parse(
        "
        FUNCTION foo : MyType
            VAR_INPUT
                x : INT;
                y : INT;
            END_VAR
        END_FUNCTION

        PROGRAM PRG
            foo(x := 3, y := 6);
        END_PROGRAM
        
        TYPE MyType: INT; END_TYPE
        ",
    );

    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[1].statements;

    assert_eq!(
        annotations.type_map.get(&statements[0].get_id()),
        Some(&"INT".to_string())
    );
    if let Statement::CallStatement {
        operator,
        parameters,
        ..
    } = &statements[0]
    {
        assert_eq!(
            annotations.type_map.get(&operator.get_id()),
            Some(&"foo".to_string())
        );

        if let Some(Statement::ExpressionList { expressions, .. }) = &**parameters {
            if let Statement::Assignment { left, right, .. } = &expressions[0] {
                assert_eq!(
                    annotations.type_map.get(&left.get_id()),
                    Some(&"INT".to_string())
                );
                assert_eq!(
                    annotations.type_map.get(&right.get_id()),
                    Some(&"BYTE".to_string())
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
        assert_parameter_assignment(parameters, 0, "INT", "DINT", &annotations);
        assert_parameter_assignment(parameters, 1, "BOOL", "REAL", &annotations);

        //check the inner call in the first parameter assignment of the outer call `x := baz(...)`
        if let Statement::Assignment { right, .. } = get_expression_from_list(parameters, 0) {
            if let Statement::CallStatement { parameters, .. } = right.as_ref() {
                // the left side here should be `x` - so lets see if it got mixed up with the outer call's `x`
                assert_parameter_assignment(parameters, 0, "DINT", "BYTE", &annotations);
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
) {
    if let Some(Statement::ExpressionList { expressions, .. }) = parameters {
        if let Statement::Assignment { left, right, .. } = &expressions[param_index] {
            assert_eq!(
                annotations.type_map.get(&left.get_id()),
                Some(&left_type.to_string())
            );
            assert_eq!(
                annotations.type_map.get(&right.get_id()),
                Some(&right_type.to_string())
            );
        } else {
            panic!("assignment expected")
        }
    } else {
        panic!("expression list expected")
    }
}
