use core::panic;

use insta::assert_snapshot;

use crate::{
    ast::{self, flatten_expression_list, AstStatement, DataType, Pou, UserTypeDeclaration},
    index::{Index, VariableType},
    lexer::IdProvider,
    resolver::{AnnotationMap, AnnotationMapImpl, StatementAnnotation},
    test_utils::tests::{annotate_with_ids, codegen, index_with_ids},
    typesystem::{
        DataTypeInformation, BOOL_TYPE, BYTE_TYPE, DINT_TYPE, DWORD_TYPE, INT_TYPE, LINT_TYPE,
        LREAL_TYPE, REAL_TYPE, SINT_TYPE, UINT_TYPE, USINT_TYPE, VOID_TYPE, WORD_TYPE,
    },
};

use crate::TypeAnnotator;

#[macro_export]
macro_rules! assert_type_and_hint {
    ($annotations:expr, $index:expr, $stmt:expr, $expected_type:expr, $expected_type_hint:expr) => {
        assert_eq!(
            (
                $crate::resolver::AnnotationMap::get_type($annotations, $stmt, $index),
                $crate::resolver::AnnotationMap::get_type_hint($annotations, $stmt, $index),
            ),
            (
                $index.get_type($expected_type).ok(),
                $expected_type_hint.and_then(|n| $index.get_type(n).ok())
            )
        );
    };
}
#[test]
fn binary_expressions_resolves_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
            1 + 2;
            1 + 2000;
            2147483648 + 1;
        END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["DINT", "DINT", "LINT"];

    let types: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(expected_types, types);
}

#[test]
fn binary_expressions_resolves_types_for_mixed_signed_ints() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
            VAR a : INT; END_VAR
            a + UINT#7;
        END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;
    if let AstStatement::BinaryExpression { left, right, .. } = &statements[0] {
        assert_type_and_hint!(&annotations, &index, left, INT_TYPE, Some(DINT_TYPE));
        assert_type_and_hint!(&annotations, &index, right, UINT_TYPE, Some(DINT_TYPE));
        assert_type_and_hint!(&annotations, &index, &statements[0], DINT_TYPE, None);
    } else {
        unreachable!()
    }
}

#[test]
#[ignore = "Types on builtin types are not correctly annotated"]
fn expt_binary_expression() {
    fn get_params(stmt: &AstStatement) -> (&AstStatement, &AstStatement) {
        if let AstStatement::CallStatement { parameters, .. } = stmt {
            if let &[left, right] =
                ast::flatten_expression_list(parameters.as_ref().as_ref().unwrap()).as_slice()
            {
                return (left, right);
            }
        }
        panic!("could not deconstruct call")
    }

    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM PRG
            VAR 
                a,b : DINT; 
                c,d : REAL;
                e,f : LREAL;
            END_VAR
            //DINTS
            a ** b; //DINT * DINT -> hint : DINT * DINT result DINT
            a ** d; //DINT * REAL -> hint : REAL * REAL result REAL
            a ** f; //DINT * LREAL -> hint : LREAL * LREAL result LREAL

            // REALS
            c ** b; //REAL * DINT -> hint : REAL * DINT result REAL
            c ** d; //REAL * REAL -> hint : REAL * REAL result REAL
            c ** f; //REAL * LREAL -> hint : LREAL * LREAL result LREAL

            // LREALS
            e ** b; //LREAL * DINT -> hint : REAL * DINT result REAL
            e ** d; //LREAL * REAL -> hint : LREAL * LREAL result LREAL
            e ** f; //LREAL * LREAL -> hint : LREAL * LREAL result LREAL
        END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;
    //DINT
    let (left, right) = get_params(&statements[0]);
    assert_type_and_hint!(&annotations, &index, left, DINT_TYPE, None);
    assert_type_and_hint!(&annotations, &index, right, DINT_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[0], DINT_TYPE, None);
    let (left, right) = get_params(&statements[1]);
    assert_type_and_hint!(&annotations, &index, left, DINT_TYPE, Some(REAL_TYPE));
    assert_type_and_hint!(&annotations, &index, right, REAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[1], REAL_TYPE, None);
    let (left, right) = get_params(&statements[2]);
    assert_type_and_hint!(&annotations, &index, left, DINT_TYPE, Some(LREAL_TYPE));
    assert_type_and_hint!(&annotations, &index, right, LREAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[2], LREAL_TYPE, None);

    //REAL
    let (left, right) = get_params(&statements[3]);
    assert_type_and_hint!(&annotations, &index, left, REAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, right, DINT_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[3], REAL_TYPE, None);
    let (left, right) = get_params(&statements[4]);
    assert_type_and_hint!(&annotations, &index, left, REAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, right, REAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[4], REAL_TYPE, None);
    let (left, right) = get_params(&statements[5]);
    assert_type_and_hint!(&annotations, &index, left, REAL_TYPE, Some(LREAL_TYPE));
    assert_type_and_hint!(&annotations, &index, right, LREAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[5], LREAL_TYPE, None);

    //LREAL
    let (left, right) = get_params(&statements[6]);
    assert_type_and_hint!(&annotations, &index, left, LREAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, right, DINT_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[6], LREAL_TYPE, None);
    let (left, right) = get_params(&statements[7]);
    assert_type_and_hint!(&annotations, &index, left, LREAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, right, REAL_TYPE, Some(LREAL_TYPE));
    assert_type_and_hint!(&annotations, &index, &statements[7], LREAL_TYPE, None);
    let (left, right) = get_params(&statements[8]);
    assert_type_and_hint!(&annotations, &index, left, LREAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, right, LREAL_TYPE, None);
    assert_type_and_hint!(&annotations, &index, &statements[8], LREAL_TYPE, None);
}

#[test]
fn binary_expressions_resolves_types_for_literals_directly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
            VAR a : BYTE; END_VAR
            a := a + 7;
            a := 7;
        END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    if let AstStatement::Assignment {
        right: addition, ..
    } = &statements[0]
    {
        // a + 7 --> DINT (BYTE hint)
        assert_type_and_hint!(&annotations, &index, addition, DINT_TYPE, Some(BYTE_TYPE));
        if let AstStatement::BinaryExpression {
            left: a,
            right: seven,
            ..
        } = addition.as_ref()
        {
            // a --> BYTE (DINT hint)
            assert_type_and_hint!(&annotations, &index, a, BYTE_TYPE, Some(DINT_TYPE));
            // 7 --> DINT (no hint)
            assert_type_and_hint!(&annotations, &index, seven, DINT_TYPE, None);
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }

    if let AstStatement::Assignment { right: seven, .. } = &statements[1] {
        assert_type_and_hint!(&annotations, &index, seven, DINT_TYPE, Some(BYTE_TYPE));
    } else {
        unreachable!()
    }
}

#[test]
fn addition_substraction_expression_with_pointers_resolves_to_pointer_type() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
            VAR a : REF_TO BYTE; b : BYTE; END_VAR
            a := &b + 7;
            a := a + 7 + 1;
            a := 7 + &b;
        END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    if let AstStatement::Assignment {
        right: addition, ..
    } = &statements[0]
    {
        assert_type_and_hint!(
            &annotations,
            &index,
            addition,
            "__POINTER_TO_BYTE",
            Some("__PRG_a")
        );
    }
    if let AstStatement::Assignment {
        right: addition, ..
    } = &statements[1]
    {
        assert_type_and_hint!(&annotations, &index, addition, "__PRG_a", Some("__PRG_a"));
        if let AstStatement::BinaryExpression { left, .. } = &**addition {
            assert_type_and_hint!(&annotations, &index, left, "__PRG_a", None);
        }
    }
    if let AstStatement::Assignment {
        right: addition, ..
    } = &statements[2]
    {
        assert_type_and_hint!(
            &annotations,
            &index,
            addition,
            "__POINTER_TO_BYTE",
            Some("__PRG_a")
        );
    }
}

#[test]
fn equality_with_pointers_is_bool() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
            VAR a : REF_TO BYTE; b : BOOL; END_VAR
            b := a > 7;
            b := 0 = a;
        END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    if let AstStatement::Assignment {
        right: addition, ..
    } = &statements[0]
    {
        assert_type_and_hint!(&annotations, &index, addition, BOOL_TYPE, Some(BOOL_TYPE));
    }
    if let AstStatement::Assignment {
        right: addition, ..
    } = &statements[1]
    {
        assert_type_and_hint!(&annotations, &index, addition, BOOL_TYPE, Some(BOOL_TYPE));
    }
}

#[test]
fn complex_expressions_resolves_types_for_literals_directly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
            VAR 
                a : BYTE; 
                b : SINT; 
                c : INT; 
            END_VAR
            a := ((b + USINT#7) - c);
        END_PROGRAM",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    if let AstStatement::Assignment { right, .. } = &statements[0] {
        // ((b + USINT#7) - c)
        assert_type_and_hint!(&annotations, &index, right, DINT_TYPE, Some(BYTE_TYPE));
        if let AstStatement::BinaryExpression { left, right: c, .. } = right.as_ref() {
            // c
            assert_type_and_hint!(&annotations, &index, c, INT_TYPE, Some(DINT_TYPE));
            // (b + USINT#7)
            assert_type_and_hint!(&annotations, &index, left, DINT_TYPE, None);

            if let AstStatement::BinaryExpression {
                left: b,
                right: seven,
                ..
            } = left.as_ref()
            {
                //b
                assert_type_and_hint!(&annotations, &index, b, SINT_TYPE, Some(DINT_TYPE));
                // USINT#7
                assert_type_and_hint!(&annotations, &index, seven, USINT_TYPE, Some(DINT_TYPE));
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        // 7 --> DINT (BYTE hint)
    } else {
        unreachable!()
    }
}

#[test]
fn unary_expressions_resolves_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
            NOT TRUE;
            -(2+3);
            -0.2;
        END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BOOL", "DINT", "REAL"];

    let types: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(expected_types, types);
}

#[test]
fn binary_expressions_resolves_types_with_floats() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
            1 + 2.2;
            1.1 + 2000;
            2000.0 + 1.0;
        END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
fn binary_expressions_resolves_types_with_float_comparisons() {
    //GIVEN some comparison expressions with floats
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
            VAR a, b : REAL END_VAR
                a < b;
                a = b;
                a >= b;
        END_PROGRAM",
        id_provider.clone(),
    );

    //WHEN I annotate the code
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    //I want the expressions to be of type BOOL, the left and right of type REAL
    for s in statements.iter() {
        assert_type_and_hint!(&annotations, &index, s, BOOL_TYPE, None);

        if let AstStatement::BinaryExpression { left, right, .. } = s {
            assert_type_and_hint!(&annotations, &index, left, REAL_TYPE, None);
            assert_type_and_hint!(&annotations, &index, right, REAL_TYPE, None);
        } else {
            unreachable!()
        }
    }
}

#[test]
fn binary_expressions_resolves_types_of_literals_with_float_comparisons() {
    //GIVEN some comparison expressions with floats
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
            VAR a : REAL END_VAR
                a < 1;
        END_PROGRAM",
        id_provider.clone(),
    );

    //WHEN I annotate the code
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    //I want the '1' to be treated as a real right away (no casting involved)
    for s in statements.iter() {
        assert_type_and_hint!(&annotations, &index, s, BOOL_TYPE, None);

        if let AstStatement::BinaryExpression { left, right, .. } = s {
            assert_type_and_hint!(&annotations, &index, left, REAL_TYPE, None);
            assert_type_and_hint!(&annotations, &index, right, REAL_TYPE, None);
        } else {
            unreachable!()
        }
    }
}

#[test]
fn local_variables_resolves_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "DINT", "DINT", "DINT", "LWORD", "DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "LINT",
        "ULINT",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn necessary_promotions_should_be_type_hinted() {
    // GIVEN  BYTE + DINT, BYTE < DINT
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        VAR_GLOBAL
            b : BYTE;
            di : DINT;
       END_VAR
        
        PROGRAM PRG
            b + di;
            b < di;
        END_PROGRAM",
        id_provider.clone(),
    );

    //WHEN it gets annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    // THEN we want a hint to promote b to DINT, BYTE + DINT should be treated as DINT
    if let AstStatement::BinaryExpression { left, .. } = &statements[0] {
        assert_eq!(
            annotations.get_type(&statements[0], &index),
            index.find_effective_type_by_name("DINT")
        );
        assert_eq!(
            (
                annotations.get_type(left.as_ref(), &index),
                annotations.get_type_hint(left.as_ref(), &index)
            ),
            (
                index.find_effective_type_by_name("BYTE"),
                index.find_effective_type_by_name("DINT")
            )
        );
    } else {
        unreachable!();
    }

    // THEN we want a hint to promote b to DINT, BYTE < DINT should be treated as BOOL
    if let AstStatement::BinaryExpression { left, .. } = &statements[1] {
        assert_eq!(
            annotations.get_type(&statements[1], &index),
            index.find_effective_type_by_name("BOOL")
        );
        assert_eq!(
            (
                annotations.get_type(left.as_ref(), &index),
                annotations.get_type_hint(left.as_ref(), &index)
            ),
            (
                index.find_effective_type_by_name("BYTE"),
                index.find_effective_type_by_name("DINT")
            )
        );
    } else {
        unreachable!();
    }
}

#[test]
fn necessary_promotions_between_real_and_literal_should_be_type_hinted() {
    // GIVEN  REAL > DINT
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        VAR_GLOBAL
            f : REAL;
       END_VAR
        
        PROGRAM PRG
            f > 0;
        END_PROGRAM",
        id_provider.clone(),
    );

    //WHEN it gets annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;

    // THEN we want '0' to be treated as a REAL right away, the result of f > 0 should be type bool
    if let AstStatement::BinaryExpression { right, .. } = &statements[0] {
        assert_eq!(
            annotations.get_type(&statements[0], &index),
            index.find_effective_type_by_name("BOOL")
        );

        assert_type_and_hint!(&annotations, &index, &statements[0], BOOL_TYPE, None);
        assert_type_and_hint!(&annotations, &index, right.as_ref(), REAL_TYPE, None);
    } else {
        unreachable!();
    }
}

#[test]
fn complex_expressions_resolve_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
            VAR
                i : ARRAY[0..10] OF INT;
                y : ARRAY[0..10] OF MyInt;
                a : MyIntArray;
                b : MyAliasArray;
                z : ARRAY[0..10] OF ARRAY[0..5] OF BYTE;
            END_VAR

            i;
            i[2];

            y;
            y[2];

            a;
            a[2];

            b;
            b[2];

            z;
            z[2];
        END_PROGRAM
        
        TYPE MyInt: INT := 7; END_TYPE 
        TYPE MyIntArray: ARRAY[0..10] OF INT := 7; END_TYPE 
        TYPE MyAliasArray: ARRAY[0..10] OF MyInt := 7; END_TYPE 

        ",
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
        "__PRG_z",
        "__PRG_z_",
    ];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn qualified_expressions_resolve_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[1].statements;

    let expected_types = vec!["BYTE", "WORD", "DWORD", "LWORD", "DINT", "DINT", "LWORD"];
    let type_names: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(format!("{:?}", expected_types), format!("{:?}", type_names));
}

#[test]
fn pou_expressions_resolve_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
        annotations.get(&statements[0])
    );
    assert_eq!(
        Some(&StatementAnnotation::Function {
            qualified_name: "OtherFunc".into(),
            return_type: "INT".into(),
            call_name: None,
        }),
        annotations.get(&statements[1])
    );
    assert_eq!(
        Some(&StatementAnnotation::Type {
            type_name: "OtherFuncBlock".into()
        }),
        annotations.get(&statements[2])
    );
}

#[test]
fn assignment_expressions_resolve_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION foo : INT
        foo;
        END_FUNCTION

        PROGRAM PRG
            foo;
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[1].statements;

    // THEN we expect it to be annotated with the function itself
    let foo_annotation = annotations.get(&statements[0]);
    assert_eq!(
        Some(&StatementAnnotation::Function {
            qualified_name: "foo".into(),
            return_type: "INT".into(),
            call_name: None,
        }),
        foo_annotation
    );
    // AND we expect no type to be associated with the expression
    let associated_type = annotations.get_type(&statements[0], &index);
    assert_eq!(None, associated_type);

    let statements = &unit.implementations[0].statements;
    let foo_annotation = annotations.get(&statements[0]);
    assert_eq!(
        Some(&StatementAnnotation::Variable {
            qualified_name: "foo.foo".into(),
            resulting_type: "INT".into(),
            constant: false,
            is_auto_deref: false,
            variable_type: VariableType::Return
        }),
        foo_annotation
    );
}

#[test]
fn function_call_expression_resolves_to_the_function_itself_not_its_return_type() {
    //GIVEN a reference to a function
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION foo : INT
        END_FUNCTION

        PROGRAM PRG
            foo();
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[1].statements;

    // THEN we expect it to be annotated with the function itself
    let foo_annotation = annotations.get(&statements[0]);
    assert_eq!(
        Some(&StatementAnnotation::Value {
            resulting_type: "INT".into()
        }),
        foo_annotation
    );

    // AND we expect no type to be associated with the expression
    let associated_type = annotations.get_type(&statements[0], &index);
    assert_eq!(index.find_effective_type_by_name("INT"), associated_type);
}

#[test]
fn shadowed_function_is_annotated_correctly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        FUNCTION foo : DINT
        END_FUNCTION

        PROGRAM prg 
        foo();
        END_PROGRAM
        ",
        id_provider.clone(),
    );
    //WHEN the AST is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[1].statements;

    // THEN we expect it to be annotated with the function itself
    assert_type_and_hint!(&annotations, &index, &statements[0], "DINT", None);
}

#[test]
fn qualified_expressions_to_aliased_structs_resolve_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    assert_eq!(None, annotations.get(&statements[0]));
    assert_eq!(
        Some(&StatementAnnotation::Variable {
            qualified_name: "MyStruct.y".into(),
            resulting_type: "INT".into(),
            constant: false,
            is_auto_deref: false,
            variable_type: VariableType::Input
        }),
        annotations.get(&statements[1])
    );
}

#[test]
fn function_parameter_assignments_resolve_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[1].statements;

    assert_eq!(
        annotations
            .get_type_or_void(&statements[0], &index)
            .get_name(),
        "INT"
    );
    assert_eq!(
        annotations.get(&statements[0]),
        Some(&StatementAnnotation::value("INT"))
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
            annotations.get(operator),
            Some(&StatementAnnotation::Function {
                qualified_name: "foo".into(),
                return_type: "MyType".into(),
                call_name: None,
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE MyStruct : STRUCT
            x : INT := 20;
            y : BOOL := TRUE;
            z : STRING := 'abc';
        END_STRUCT
        END_TYPE
        ",
        id_provider.clone(),
    );

    let (mut annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    index.import(std::mem::take(&mut annotations.new_index));

    let UserTypeDeclaration { data_type, .. } = &unit.types[0];

    if let DataType::StructType { variables, .. } = data_type {
        assert_eq!(
            Some(&StatementAnnotation::value("DINT")),
            annotations.get(variables[0].initializer.as_ref().unwrap())
        );
        assert_eq!(
            Some(&StatementAnnotation::value("BOOL")),
            annotations.get(variables[1].initializer.as_ref().unwrap())
        );

        let _type_of_z = index.find_member("MyStruct", "z").unwrap().get_type_name();
        assert_eq!(
            Some(&StatementAnnotation::value(
                index
                    .find_effective_type_by_name("__STRING_3")
                    .unwrap()
                    .get_name()
            )),
            annotations.get(variables[2].initializer.as_ref().unwrap())
        );
    } else {
        panic!("no datatype: {:#?}", data_type)
    }
}

#[test]
fn actions_are_resolved() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let foo_reference = &unit.implementations[0].statements[0];
    let annotation = annotations.get(foo_reference);
    assert_eq!(
        Some(&StatementAnnotation::Program {
            qualified_name: "prg.foo".into(),
        }),
        annotation
    );
    let foo_reference = &unit.implementations[0].statements[1];
    let annotation = annotations.get(foo_reference);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let foo_reference = &unit.implementations[0].statements[0];
    let annotation = annotations.get(foo_reference);
    assert_eq!(
        Some(&StatementAnnotation::Variable {
            qualified_name: "cls.foo.foo".into(),
            resulting_type: "INT".into(),
            constant: false,
            is_auto_deref: false,
            variable_type: VariableType::Return
        }),
        annotation
    );
    let method_call = &unit.implementations[1].statements[0];
    if let AstStatement::CallStatement { operator, .. } = method_call {
        assert_eq!(
            Some(&StatementAnnotation::Function {
                return_type: "INT".into(),
                qualified_name: "cls.foo".into(),
                call_name: None,
            }),
            annotations.get(operator)
        );
        assert_eq!(
            Some(&StatementAnnotation::value("INT")),
            annotations.get(method_call)
        );
    } else {
        panic!("Unexpcted statemet : {:?}", method_call);
    }
}

#[test]
fn bitaccess_is_resolved() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    annotations: &AnnotationMapImpl,
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
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
        id_provider.clone(),
    );

    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
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

#[test]
fn global_enums_type_resolving() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "VAR_GLOBAL
            x : (a,b,c);
        END_VAR",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    //check the type-annotation of a,b,c's implicit initializers

    let initalizer_types = index
        .get_global_qualified_enums()
        .values()
        .map(|it| {
            let const_exp = index
                .get_const_expressions()
                .get_constant_statement(it.initial_value.as_ref().unwrap())
                .unwrap();
            annotations
                .get_type(const_exp, &index)
                .map(|it| it.get_name())
        })
        .collect::<Vec<Option<&str>>>();

    assert_eq!(
        vec![Some("DINT"), Some("__global_x"), Some("__global_x")],
        initalizer_types
    );
}

#[test]
fn global_enums_type_resolving2() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        " TYPE MyEnum : BYTE (zero, aa, bb := 7, cc); END_TYPE",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    //check the type-annotation of a,b,c's implicit initializers

    let initalizer_types = index
        .get_global_qualified_enums()
        .values()
        .map(|it| {
            let const_exp = index
                .get_const_expressions()
                .get_constant_statement(it.initial_value.as_ref().unwrap())
                .unwrap();
            (
                annotations
                    .get_type(const_exp, &index)
                    .map(|it| it.get_name()),
                annotations
                    .get_type_hint(const_exp, &index)
                    .map(|it| it.get_name()),
            )
        })
        .collect::<Vec<(Option<&str>, Option<&str>)>>();

    assert_eq!(
        vec![
            (Some("DINT"), Some("MyEnum")),
            (Some("DINT"), Some("MyEnum")),
            (Some("DINT"), Some("MyEnum")),
            (Some("DINT"), Some("MyEnum")),
        ],
        initalizer_types
    );
}

#[test]
fn global_lint_enums_type_resolving() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        " TYPE MyEnum : LINT (zero, aa, bb := 7, cc); END_TYPE",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    //check the type-annotation of a,b,c's implicit initializers

    let initalizer_types = index
        .get_global_qualified_enums()
        .values()
        .map(|it| {
            let const_exp = index
                .get_const_expressions()
                .get_constant_statement(it.initial_value.as_ref().unwrap())
                .unwrap();
            (
                annotations
                    .get_type(const_exp, &index)
                    .map(|it| it.get_name()),
                annotations
                    .get_type_hint(const_exp, &index)
                    .map(|it| it.get_name()),
            )
        })
        .collect::<Vec<(Option<&str>, Option<&str>)>>();

    assert_eq!(
        vec![
            (Some("DINT"), Some("MyEnum")),
            (Some("MyEnum"), Some("MyEnum")),
            (Some("DINT"), Some("MyEnum")),
            (Some("MyEnum"), Some("MyEnum")),
        ],
        initalizer_types
    );
}

#[test]
fn enum_element_initialization_is_annotated_correctly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        " TYPE MyEnum : BYTE (zero, aa, bb := 7, cc); END_TYPE ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let data_type = &unit.types[0].data_type;
    if let DataType::EnumType { elements, .. } = data_type {
        if let AstStatement::Assignment { right, .. } = flatten_expression_list(&elements)[2] {
            assert_type_and_hint!(&annotations, &index, &*right, "DINT", Some("MyEnum"));
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}
#[test]
fn enum_initialization_is_annotated_correctly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        " TYPE MyEnum : BYTE (zero, aa, bb := 7, cc); END_TYPE
        
        PROGRAM PRG
            VAR_TEMP
                x : MyEnum := 1;
                y : MyEnum := bb;
                z : MyEnum := cc;
            END_VAR


            x := aa;
            x := bb;
            x := cc;
        END_PROGRAM
        ",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    let variables = &unit.units[0].variable_blocks[0].variables;

    assert_type_and_hint!(
        &annotations,
        &index,
        variables[0].initializer.as_ref().unwrap(),
        "DINT",
        Some("MyEnum")
    );
    assert_type_and_hint!(
        &annotations,
        &index,
        variables[1].initializer.as_ref().unwrap(),
        "MyEnum",
        Some("MyEnum")
    );
    assert_type_and_hint!(
        &annotations,
        &index,
        variables[2].initializer.as_ref().unwrap(),
        "MyEnum",
        Some("MyEnum")
    );

    let statements = &unit.implementations[0].statements;
    if let AstStatement::Assignment { right, .. } = &statements[0] {
        assert_type_and_hint!(
            &annotations,
            &index,
            right.as_ref(),
            "MyEnum",
            Some("MyEnum")
        );
    } else {
        unreachable!()
    }
    if let AstStatement::Assignment { right, .. } = &statements[1] {
        assert_type_and_hint!(
            &annotations,
            &index,
            right.as_ref(),
            "MyEnum",
            Some("MyEnum")
        );
    } else {
        unreachable!()
    }
    if let AstStatement::Assignment { right, .. } = &statements[2] {
        assert_type_and_hint!(
            &annotations,
            &index,
            right.as_ref(),
            "MyEnum",
            Some("MyEnum")
        );
    } else {
        unreachable!()
    }
}

#[test]
fn struct_members_initializers_type_hint_test() {
    //GIVEN a struct with some initialization
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE MyStruct:
        STRUCT
          i : INT := 7;
          si : SINT := 7;
          b : BOOL := 1;
          r : REAL := 3.1415;
          lr : LREAL := 3.1415;
        END_STRUCT
        END_TYPE
       ",
        id_provider.clone(),
    );

    // WHEN this type is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN the members's initializers have correct type-hints
    if let DataType::StructType { variables, .. } = &unit.types[0].data_type {
        let hints: Vec<&str> = variables
            .iter()
            .map(|v| {
                annotations
                    .get_type_hint(v.initializer.as_ref().unwrap(), &index)
                    .map(crate::typesystem::DataType::get_name)
                    .unwrap()
            })
            .collect();

        assert_eq!(hints, vec!["INT", "SINT", "BOOL", "REAL", "LREAL"]);
    } else {
        unreachable!()
    }
}

#[test]
fn program_members_initializers_type_hint_test() {
    //GIVEN a pou with some initialization
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM prg
      	  VAR_INPUT
            i : INT := 7;
            si : SINT := 7;
            b : BOOL := 1;
            r : REAL := 3.1415;
            lr : LREAL := 3.1415;
          END_VAR
        END_PROGRAM
      ",
        id_provider.clone(),
    );

    // WHEN it is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN the members's initializers have correct type-hints
    let Pou {
        variable_blocks: blocks,
        ..
    } = &unit.units[0];
    let hints: Vec<&str> = blocks[0]
        .variables
        .iter()
        .map(|v| {
            annotations
                .get_type_hint(v.initializer.as_ref().unwrap(), &index)
                .map(crate::typesystem::DataType::get_name)
                .unwrap()
        })
        .collect();

    assert_eq!(hints, vec!["INT", "SINT", "BOOL", "REAL", "LREAL"]);
}

#[test]
fn data_type_initializers_type_hint_test() {
    //GIVEN a struct with some initialization
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE MyArray : ARRAY[0..2] OF INT := [1, 2, 3]; END_TYPE
       ",
        id_provider.clone(),
    );

    // WHEN this type is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN the members's initializers have correct type-hints
    if let Some(initializer) = &unit.types[0].initializer {
        assert_eq!(
            Some(index.get_type("MyArray").unwrap()),
            annotations.get_type_hint(initializer, &index)
        );

        let initializer = index.get_type("MyArray").unwrap().initial_value.unwrap();
        if let AstStatement::LiteralArray {
            elements: Some(exp_list),
            ..
        } = index
            .get_const_expressions()
            .get_constant_statement(&initializer)
            .unwrap()
        {
            if let AstStatement::ExpressionList {
                expressions: elements,
                ..
            } = exp_list.as_ref()
            {
                for ele in elements {
                    assert_eq!(
                        index.get_type("INT").unwrap(),
                        annotations.get_type_hint(ele, &index).unwrap()
                    );
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

#[test]
fn data_type_initializers_multiplied_statement_type_hint_test() {
    //GIVEN a struct with some initialization
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE MyArray : ARRAY[0..2] OF BYTE := [3(7)]; END_TYPE
        VAR_GLOBAL a : ARRAY[0..2] OF BYTE := [3(7)]; END_VAR
       ",
        id_provider.clone(),
    );

    // WHEN this type is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN the members's initializers have correct type-hints
    if let Some(my_array_initializer) = &unit.types[0].initializer {
        let my_array_type = index.get_type("MyArray").unwrap();
        assert_eq!(
            Some(my_array_type),
            annotations.get_type_hint(my_array_initializer, &index)
        );

        let my_array_type_const_initializer = my_array_type.initial_value.unwrap();
        if let AstStatement::LiteralArray {
            elements: Some(multiplied_statement),
            ..
        } = index
            .get_const_expressions()
            .get_constant_statement(&my_array_type_const_initializer)
            .unwrap()
        {
            if let AstStatement::MultipliedStatement {
                element: literal_seven,
                ..
            } = multiplied_statement.as_ref()
            {
                assert_eq!(
                    index.find_effective_type_by_name(BYTE_TYPE),
                    annotations.get_type_hint(literal_seven, &index)
                );
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }

    //same checks for the global a
    if let Some(a_initializer) = &unit.global_vars[0].variables[0].initializer {
        let global = index.find_global_variable("a").unwrap();
        assert_eq!(
            index.find_effective_type_by_name(global.get_type_name()),
            annotations.get_type_hint(a_initializer, &index)
        );

        let global_var_const_initializer = global.initial_value.unwrap();
        if let AstStatement::LiteralArray {
            elements: Some(multiplied_statement),
            ..
        } = index
            .get_const_expressions()
            .get_constant_statement(&global_var_const_initializer)
            .unwrap()
        {
            if let AstStatement::MultipliedStatement {
                element: literal_seven,
                ..
            } = multiplied_statement.as_ref()
            {
                assert_eq!(
                    index.find_effective_type_by_name(BYTE_TYPE),
                    annotations.get_type_hint(literal_seven, &index)
                );
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

#[test]
fn case_conditions_type_hint_test() {
    //GIVEN a Switch-Case statement
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM prg 
        VAR
            x : BYTE;
            y : BYTE;
        END_VAR
        CASE x OF
            1: y := 1;
            2: y := 2;
            3: y := 3;
        ELSE
            y := 0;
        END_CASE
        END_PROGRAM
       ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN we want the case-bocks (1:, 2: , 3:) to have the type hint of the case-selector (x) - in this case BYTE

    //check if 'CASE x' got the type BYTE
    if let AstStatement::CaseStatement {
        selector,
        case_blocks,
        ..
    } = &unit.implementations[0].statements[0]
    {
        let type_of_x = annotations.get_type(selector, &index).unwrap();

        assert_eq!(type_of_x, index.get_type(BYTE_TYPE).unwrap());

        for b in case_blocks {
            let type_hint = annotations
                .get_type_hint(b.condition.as_ref(), &index)
                .unwrap();
            assert_eq!(type_hint, type_of_x);
        }
    } else {
        unreachable!()
    }
}

#[test]
fn range_type_min_max_type_hint_test() {
    //GIVEN a Switch-Case statement
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
            TYPE MyInt: SINT(0..100); END_TYPE
        ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN we want the range-limits (0 and 100) to have proper type-associations
    if let DataType::SubRangeType {
        bounds: Some(AstStatement::RangeStatement { start, end, .. }),
        ..
    } = &unit.types[0].data_type
    {
        //lets see if start and end got their type-annotations
        assert_eq!(
            annotations.get_type(start.as_ref(), &index),
            index.find_effective_type_by_name(DINT_TYPE)
        );
        assert_eq!(
            annotations.get_type(end.as_ref(), &index),
            index.find_effective_type_by_name(DINT_TYPE)
        );

        //lets see if start and end got their type-HINT-annotations
        assert_eq!(
            annotations.get_type_hint(start.as_ref(), &index),
            index.find_effective_type_by_name(SINT_TYPE)
        );
        assert_eq!(
            annotations.get_type_hint(end.as_ref(), &index),
            index.find_effective_type_by_name(SINT_TYPE)
        );
    }
}

#[test]
fn struct_variable_initialization_annotates_initializer() {
    //GIVEN a STRUCT type and global variables of this type
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE MyStruct: STRUCT
          a: DINT; b: DINT;
        END_STRUCT END_TYPE

         VAR_GLOBAL 
           a : MyStruct  := (a:=3, b:=5); 
           b : MyStruct  := (a:=3); 
         END_VAR
         ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN we want the whole initializer to have a type-hint of 'MyStruct'
    {
        let initializer = index
            .find_global_variable("a")
            .unwrap()
            .initial_value
            .and_then(|i| index.get_const_expressions().get_constant_statement(&i))
            .unwrap();

        assert_eq!(
            annotations.get_type_hint(initializer, &index),
            index.find_effective_type_by_name("MyStruct")
        );
    }
    {
        let initializer = index
            .find_global_variable("b")
            .unwrap()
            .initial_value
            .and_then(|i| index.get_const_expressions().get_constant_statement(&i))
            .unwrap();

        assert_eq!(
            annotations.get_type_hint(initializer, &index),
            index.find_effective_type_by_name("MyStruct")
        );
    }
}

#[test]
fn deep_struct_variable_initialization_annotates_initializer() {
    //GIVEN a 2 lvl-STRUCT type and global variables of this type
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        TYPE Point: STRUCT
          a: BYTE; b: SINT;
        END_STRUCT END_TYPE

        Type MyStruct: STRUCT
            v: Point; q: Point;
        END_STRUCT END_TYPE

         VAR_GLOBAL 
           a : MyStruct  := (
               v := (a := 1, b := 2), 
               q := (b := 3)); 
         END_VAR
         ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    // THEN we want the whole initializer to have a type-hint of 'MyStruct'
    let initializer = index
        .find_global_variable("a")
        .unwrap()
        .initial_value
        .and_then(|i| index.get_const_expressions().get_constant_statement(&i))
        .unwrap();

    assert_eq!(
        annotations.get_type_hint(initializer, &index),
        index.find_effective_type_by_name("MyStruct")
    );

    //check the initializer-part
    if let AstStatement::ExpressionList { expressions, .. } = initializer {
        // v := (a := 1, b := 2)
        if let AstStatement::Assignment { left, right, .. } = &expressions[0] {
            assert_eq!(
                annotations.get_type(left, &index),
                index.find_effective_type_by_name("Point")
            );
            assert_eq!(
                annotations.get_type_hint(right, &index),
                index.find_effective_type_by_name("Point")
            );

            // (a := 1, b := 2)
            if let AstStatement::ExpressionList { expressions, .. } = right.as_ref() {
                // a := 1
                if let AstStatement::Assignment { left, right, .. } = &expressions[0] {
                    assert_eq!(
                        annotations.get_type(left.as_ref(), &index),
                        index.find_effective_type_by_name("BYTE")
                    );
                    assert_eq!(
                        annotations.get_type_hint(right.as_ref(), &index),
                        index.find_effective_type_by_name("BYTE")
                    );
                } else {
                    unreachable!()
                }

                // b := 2
                if let AstStatement::Assignment { left, right, .. } = &expressions[1] {
                    assert_eq!(
                        annotations.get_type(left.as_ref(), &index),
                        index.find_effective_type_by_name("SINT")
                    );
                    assert_eq!(
                        annotations.get_type_hint(right.as_ref(), &index),
                        index.find_effective_type_by_name("SINT")
                    );
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

#[test]
fn inouts_should_be_annotated_according_to_auto_deref() {
    //a program with in-out variables that get auto-deref'd
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM foo 
            VAR_IN_OUT
                inout : DINT;
            END_VAR

            inout;
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let inout_ref = &unit.implementations[0].statements[0];

    // then accessing inout should be annotated with DINT, because it is auto-dereferenced
    assert_type_and_hint!(&annotations, &index, inout_ref, DINT_TYPE, None);
}

#[test]
fn action_call_should_be_annotated() {
    //a program with in-out variables that get auto-deref'd
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
        prg.foo();
        END_PROGRAM
        ACTIONS prg
        ACTION foo
            x := 2;
        END_ACTION
        ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let action_call = &unit.implementations[0].statements[0];

    // then accessing inout should be annotated with DINT, because it is auto-dereferenced
    if let AstStatement::CallStatement { operator, .. } = action_call {
        let a = annotations.get(operator);
        assert_eq!(
            Some(&StatementAnnotation::Program {
                qualified_name: "prg.foo".to_string()
            }),
            a
        );
    }
}

#[test]
fn action_body_gets_resolved() {
    //a program with an action in it
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM prg 
            VAR
                x : DINT;
            END_VAR
            prg.foo();
            END_PROGRAM
            ACTIONS prg
            ACTION foo
                x := 2;
            END_ACTION
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let x_assignment = &unit.implementations[1].statements[0];

    // then accessing inout should be annotated with DINT, because it is auto-dereferenced
    if let AstStatement::Assignment { left, right, .. } = x_assignment {
        let a = annotations.get(left);
        assert_eq!(
            Some(&StatementAnnotation::Variable {
                qualified_name: "prg.x".to_string(),
                resulting_type: "DINT".to_string(),
                constant: false,
                is_auto_deref: false,
                variable_type: VariableType::Local
            }),
            a
        );

        let two = annotations.get(right);
        assert_eq!(Some(&StatementAnnotation::value(DINT_TYPE)), two);
    }
}

#[test]
fn class_method_gets_annotated() {
    //a class with a method with class-variables and method-variables
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
    CLASS MyClass
        VAR
            x, y : BYTE;
        END_VAR
    
        METHOD testMethod
            VAR_INPUT myMethodArg : DINT; END_VAR
            VAR myMethodLocalVar : SINT; END_VAR

            x;
            myMethodArg;
            y;
            myMethodLocalVar;
        END_METHOD
    END_CLASS
        ",
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let body = &unit.implementations[0].statements;

    // then accessing inout should be annotated with DINT, because it is auto-dereferenced
    assert_type_and_hint!(&annotations, &index, &body[0], "BYTE", None);
    assert_type_and_hint!(&annotations, &index, &body[1], "DINT", None);
    assert_type_and_hint!(&annotations, &index, &body[2], "BYTE", None);
    assert_type_and_hint!(&annotations, &index, &body[3], "SINT", None);
}

#[test]
fn nested_bitwise_access_resolves_correctly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"PROGRAM prg
        VAR
        a : BOOL;
        x : LWORD;
        END_VAR
        (* Second bit of the second byte of the second word of the second dword of an lword*)
        a := x.%D1.%W1.%B1.%X1;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let assignment = &unit.implementations[0].statements[0];

    if let AstStatement::Assignment { right, .. } = assignment {
        assert_type_and_hint!(&annotations, &index, right, "BOOL", Some("BOOL")); //strange
        if let AstStatement::QualifiedReference { elements, .. } = right.as_ref() {
            assert_type_and_hint!(&annotations, &index, &elements[0], "LWORD", None);
            assert_type_and_hint!(&annotations, &index, &elements[1], "DWORD", None);
            assert_type_and_hint!(&annotations, &index, &elements[2], "WORD", None);
            assert_type_and_hint!(&annotations, &index, &elements[3], "BYTE", None);
            assert_type_and_hint!(&annotations, &index, &elements[4], "BOOL", None);

            if let AstStatement::DirectAccess {
                index: idx_stmt, ..
            } = &elements[4]
            {
                assert_type_and_hint!(&annotations, &index, idx_stmt, "DINT", None);
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!();
    }
}

#[test]
fn literals_passed_to_function_get_annotated() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        FUNCTION foo : STRING
            VAR_INPUT b : BYTE; in : STRING END_VAR

            foo := in;
        END_FUNCTION

        PROGRAM prg
            foo(77, 'abc');
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let call_stmt = &unit.implementations[1].statements[0];

    if let AstStatement::CallStatement { parameters, .. } = call_stmt {
        let parameters = ast::flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[0],
            DINT_TYPE,
            Some(BYTE_TYPE)
        );
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[1],
            "__STRING_3",
            Some("STRING")
        );
    } else {
        unreachable!();
    }
}

#[test]
fn array_accessor_in_struct_array_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        TYPE MyStruct:
        STRUCT
            arr1 : ARRAY[0..3] OF INT;
        END_STRUCT
        END_TYPE

        PROGRAM main
        VAR
            data : MyStruct;
            i : INT;
        END_VAR
        
        data.arr1[i];

        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // WHEN this code is annotated
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let qr = &unit.implementations[0].statements[0];

    if let AstStatement::QualifiedReference { elements, .. } = qr {
        if let AstStatement::ArrayAccess { access, .. } = &elements[1] {
            assert_type_and_hint!(&annotations, &index, access.as_ref(), "INT", None);
        } else {
            unreachable!()
        }
    } else {
        unreachable!();
    }
}

#[test]
fn type_hint_should_not_hint_to_the_effective_type_but_to_the_original() {
    //GIVEN a aliased type to INT and a variable declared as myInt
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        TYPE MyInt: INT(0..100); END_TYPE

        PROGRAM Main
        VAR
            x : MyInt;
        END_VAR 
        x := 7;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    //WHEN we assign to this variable (x := 7)

    // THEN we want the hint for '7' to be MyInt, not INT
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let stmt = &unit.implementations[0].statements[0];

    if let AstStatement::Assignment { left, right, .. } = stmt {
        assert_type_and_hint!(&annotations, &index, left, "MyInt", None);
        assert_type_and_hint!(&annotations, &index, right, "DINT", Some("MyInt"));
    } else {
        unreachable!();
    }
}

#[test]
fn null_statement_should_get_a_valid_type_hint() {
    //GIVEN a NULL assignment to a pointer
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        PROGRAM Main
        VAR
            x : POINTER TO BYTE;
        END_VAR 
        x := NULL;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // THEN we want the hint for 'NULL' to be POINTER TO BYTE
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let stmt = &unit.implementations[0].statements[0];

    let var_x_type = &unit.units[0].variable_blocks[0].variables[0]
        .data_type
        .get_name()
        .unwrap();

    if let AstStatement::Assignment { right, .. } = stmt {
        assert_type_and_hint!(&annotations, &index, right, "VOID", Some(var_x_type));
    } else {
        unreachable!();
    }
}

#[test]
fn resolve_function_with_same_name_as_return_type() {
    //GIVEN a reference to a function with the same name as the return type
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION TIME : TIME
        END_FUNCTION

        PROGRAM PRG
            TIME();
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[1].statements;

    // THEN we expect it to be annotated with the function itself
    let function_annotation = annotations.get(&statements[0]);
    assert_eq!(
        Some(&StatementAnnotation::Value {
            resulting_type: "TIME".into()
        }),
        function_annotation
    );

    // AND we expect no type to be associated with the expression
    let associated_type = annotations.get_type(&statements[0], &index).unwrap();
    let effective_type = index.find_effective_type_by_name("TIME").unwrap();
    assert_eq!(effective_type, associated_type);
    // AND should be Integer
    assert!(matches!(
        effective_type.get_type_information(),
        DataTypeInformation::Integer { .. }
    ))
}

#[test]
fn int_compare_should_resolve_to_bool() {
    //GIVEN a NULL assignment to a pointer
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        PROGRAM Main
        3 = 5;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // THEN we want the hint for 'NULL' to be POINTER TO BYTE
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let a_eq_b = &unit.implementations[0].statements[0];
    assert_eq!(
        Some(&StatementAnnotation::Value {
            resulting_type: "BOOL".to_string(),
        }),
        annotations.get(a_eq_b),
    );
}

#[test]
fn string_compare_should_resolve_to_bool() {
    //GIVEN a NULL assignment to a pointer
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        FUNCTION STRING_EQUAL: BOOL 
        VAR a,b : STRING; END_VAR

        END_FUNCTION;

        PROGRAM Main
        VAR
            a,b: STRING;
        END_VAR 
        a = b;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // THEN we want the hint for 'NULL' to be POINTER TO BYTE
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let a_eq_b = &unit.implementations[1].statements[0];
    assert_eq!(
        Some(&StatementAnnotation::value("BOOL")),
        annotations.get(a_eq_b),
    );
}

#[test]
fn assigning_lword_to_ptr_will_annotate_correctly() {
    //GIVEN a NULL assignment to a pointer
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        PROGRAM Main
        VAR
            a : POINTER TO INT;
            b : DWORD;
        END_VAR 
        b := a;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // THEN we want the hint for 'NULL' to be POINTER TO BYTE
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let a_eq_b = &unit.implementations[0].statements[0];

    let ptr_type = unit.units[0].variable_blocks[0].variables[0]
        .data_type
        .get_name()
        .unwrap();

    if let AstStatement::Assignment { left, right, .. } = a_eq_b {
        assert_type_and_hint!(&annotations, &index, left, DWORD_TYPE, None);
        assert_type_and_hint!(&annotations, &index, right, ptr_type, Some(DWORD_TYPE));
    }
}

#[test]
fn assigning_ptr_to_lword_will_annotate_correctly() {
    //GIVEN a NULL assignment to a pointer
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        PROGRAM Main
        VAR
            a : POINTER TO INT;
            b : DWORD;
        END_VAR 
        a := b;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // THEN we want the hint for 'NULL' to be POINTER TO BYTE
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let a_eq_b = &unit.implementations[0].statements[0];

    let ptr_type = unit.units[0].variable_blocks[0].variables[0]
        .data_type
        .get_name()
        .unwrap();

    if let AstStatement::Assignment { left, right, .. } = a_eq_b {
        assert_type_and_hint!(&annotations, &index, left, ptr_type, None);
        assert_type_and_hint!(&annotations, &index, right, DWORD_TYPE, Some(ptr_type));
    }
}

#[test]
fn assigning_ptr_to_lword_will_annotate_correctly2() {
    //GIVEN a NULL assignment to a pointer
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        PROGRAM Main
        VAR
            a : POINTER TO INT;
            b : DWORD;
        END_VAR 
        b := a^;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // THEN we want the hint for 'NULL' to be POINTER TO BYTE
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let a_eq_b = &unit.implementations[0].statements[0];

    let ptr_type = unit.units[0].variable_blocks[0].variables[0]
        .data_type
        .get_name()
        .unwrap();

    if let AstStatement::Assignment { left, right, .. } = a_eq_b {
        assert_type_and_hint!(&annotations, &index, left, DWORD_TYPE, None);
        assert_type_and_hint!(&annotations, &index, right, INT_TYPE, Some(DWORD_TYPE));

        if let AstStatement::PointerAccess { reference, .. } = right.as_ref() {
            assert_type_and_hint!(&annotations, &index, reference, ptr_type, None);
        }
    } else {
        unreachable!()
    }
}

#[test]
fn address_of_is_annotated_correctly() {
    //GIVEN a NULL assignment to a pointer
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r#"
        PROGRAM Main
        VAR
            b : INT;
        END_VAR 
        &b;
        END_PROGRAM
        "#,
        id_provider.clone(),
    );

    // THEN we want the hint for 'NULL' to be POINTER TO BYTE
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    let s = &unit.implementations[0].statements[0];
    if let Some(&StatementAnnotation::Value { resulting_type }) = annotations.get(s).as_ref() {
        assert_eq!(
            Some(&DataTypeInformation::Pointer {
                auto_deref: false,
                inner_type_name: "INT".to_string(),
                name: "__POINTER_TO_INT".to_string(),
            }),
            index.find_effective_type_info(resulting_type),
        );
    } else {
        unreachable!()
    }
}

#[test]
fn pointer_assignment_with_incompatible_types_hints_correctly() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM PRG
                VAR
                    x : INT;
                    pt : POINTER TO BYTE;
                END_VAR
                pt := &x;
            END_PROGRAM",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let assignment = &unit.implementations[0].statements[0];

    if let AstStatement::Assignment { left, right, .. } = assignment {
        assert_type_and_hint!(&annotations, &index, left, "__PRG_pt", None);
        assert_type_and_hint!(
            &annotations,
            &index,
            right,
            "__POINTER_TO_INT",
            Some("__PRG_pt")
        );
    }
}

#[test]
fn call_on_function_block_array() {
    //GIVEN
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK fb
        END_FUNCTION_BLOCK

        PROGRAM PRG
		VAR
			fbs : ARRAY[1..2] OF fb;
		END_VAR
            fbs[1]();
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    // should be the call statement
    let statements = &unit.implementations[1].statements[0];
    // should contain array access as operator
    let operator = match statements {
        AstStatement::CallStatement { operator, .. } => Some(operator.as_ref()),
        _ => None,
    };
    assert!(matches!(operator, Some(&AstStatement::ArrayAccess { .. })),);

    let annotation = annotations.get(operator.unwrap());
    assert_eq!(
        Some(&StatementAnnotation::Value {
            resulting_type: "fb".into()
        }),
        annotation
    );
}

#[test]
fn and_statement_of_bools_results_in_bool() {
    //GIVEN
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        PROGRAM PRG
		VAR
            a,b : BOOL;
		END_VAR
        
            a AND b;
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let a_and_b = &unit.implementations[0].statements[0];
    // a AND b should be treated as i1
    assert_type_and_hint!(&annotations, &index, a_and_b, BOOL_TYPE, None);
}

#[test]
fn and_statement_of_dints_results_in_dint() {
    //GIVEN
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        PROGRAM PRG
		VAR
            a,b : DINT;
            c,d : INT;
		END_VAR
        
            a AND b;
            c AND d;
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    // a AND b should be treated as DINT
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[0],
        DINT_TYPE,
        None
    );
    // c AND d should be treated as DINT
    assert_type_and_hint!(
        &annotations,
        &index,
        &unit.implementations[0].statements[0],
        DINT_TYPE,
        None
    );
}

#[test]
fn resolve_recursive_function_call() {
    //GIVEN
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION foo : DINT
		VAR_INPUT
			input1 : DINT;
		END_VAR
		VAR_IN_OUT
			inout1 : DINT;
		END_VAR
		VAR_OUTPUT
			output1 : DINT;
		END_VAR
		VAR
			var1, var2, var3 : DINT;
		END_VAR
			foo(input1 := var1, inout1 := var2, output1 => var3, );
			foo := var1;
		END_FUNCTION
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let type_map = annotations.type_map;
    let annotated_types = format!("{:#?}", type_map);

    insta::assert_snapshot!(annotated_types);
}

#[test]
fn resolve_recursive_program_call() {
    //GIVEN
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        PROGRAM mainProg
		VAR_INPUT
			input1 : DINT;
		END_VAR
		VAR_IN_OUT
			inout1 : DINT;
		END_VAR
		VAR_OUTPUT
			output1 : DINT;
		END_VAR
		VAR
			var1, var2, var3 : DINT;
		END_VAR
			mainProg(input1 := var1, inout1 := var2, output1 => var3, );
		END_PROGRAM
        ",
        id_provider.clone(),
    );

    //WHEN the AST is annotated
    let (annotations, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let type_map = annotations.type_map;
    let annotated_types = format!("{:#?}", type_map);

    insta::assert_snapshot!(annotated_types);
}

#[test]
fn function_block_initialization_test() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
            FUNCTION_BLOCK TON
            VAR_INPUT
              PT: TIME;
            END_VAR
            END_FUNCTION_BLOCK


            PROGRAM main 
            VAR
                timer : TON := (PT := T#0s); 
            END_VAR
            END_PROGRAM
            ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    //PT will be a TIME variable, qualified name will be TON.PT
    let statement = unit.units[1].variable_blocks[0].variables[0]
        .initializer
        .as_ref()
        .unwrap();
    if let AstStatement::Assignment { left, .. } = statement {
        let left = left.as_ref();
        let annotation = annotations.get(left).unwrap();
        assert_eq!(
            annotation,
            &StatementAnnotation::Variable {
                resulting_type: "TIME".into(),
                qualified_name: "TON.PT".into(),
                constant: false,
                variable_type: VariableType::Input,
                is_auto_deref: false
            }
        )
    } else {
        unreachable!("Should be an assignment")
    }
}

#[test]
fn undeclared_varargs_type_hint_promoted_correctly() {
    // GIVEN a variadic function without type declarations
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
            FUNCTION variadic : BOOL
            VAR_INPUT
                args: ...;
            END_VAR
            END_FUNCTION

            PROGRAM main 
            VAR
                float: REAL := 3.0;
                double: LREAL := 4.0;
                u1: BOOL;
                u8: USINT := 255;
                short: INT := -3;
                long: DINT := 2_000_000_000;
                longlong: LINT := 16_000_000_000;
            END_VAR
                variadic(float, double, u1, u8, short, long, longlong, 'hello');
            END_PROGRAM
            ",
        id_provider.clone(),
    );

    // WHEN called with numerical types
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let call_stmt = &unit.implementations[1].statements[0];
    // THEN types smaller than LREAL/DINT get promoted while booleans and other types stay untouched.
    if let AstStatement::CallStatement { parameters, .. } = call_stmt {
        let parameters = ast::flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[0],
            REAL_TYPE,
            Some(LREAL_TYPE)
        );
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[1],
            LREAL_TYPE,
            Some(LREAL_TYPE)
        );
        assert_type_and_hint!(&annotations, &index, parameters[2], BOOL_TYPE, None);
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[3],
            USINT_TYPE,
            Some(DINT_TYPE)
        );
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[4],
            INT_TYPE,
            Some(DINT_TYPE)
        );
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[5],
            DINT_TYPE,
            Some(DINT_TYPE)
        );
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[6],
            LINT_TYPE,
            Some(LINT_TYPE)
        );
        assert_type_and_hint!(&annotations, &index, parameters[7], "__STRING_5", None);
    } else {
        unreachable!();
    }
}

#[test]
fn passing_a_function_as_param_correctly_resolves_as_variable() {
    let id_provider = IdProvider::default();
    // GIVEN a function
    let (unit, mut index) = index_with_ids(
        r#"
        {external}
        FUNCTION printf : DINT
        VAR_IN_OUT
            format : STRING;
        END_VAR
        VAR_INPUT
            args: ...;
        END_VAR
        END_FUNCTION

        FUNCTION main : DINT
            printf('Value %d, %d, %d', main, main * 10, main * 100);
        END_FUNCTION
    "#,
        id_provider.clone(),
    );

    // WHEN calling another function with itself as parameter
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let call_stmt = &unit.implementations[1].statements[0];
    // THEN the type of the parameter resolves to the original function type
    if let AstStatement::CallStatement { parameters, .. } = call_stmt {
        let parameters = ast::flatten_expression_list(parameters.as_ref().as_ref().unwrap());
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[1],
            DINT_TYPE,
            Some(DINT_TYPE)
        );
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[2],
            DINT_TYPE,
            Some(DINT_TYPE)
        );
        assert_type_and_hint!(
            &annotations,
            &index,
            parameters[3],
            DINT_TYPE,
            Some(DINT_TYPE)
        );
    } else {
        unreachable!()
    }
}

#[test]
fn resolve_return_variable_in_nested_call() {
    let id_provider = IdProvider::default();
    // GIVEN a call statement where we take the adr of the return-variable
    let src = "
        FUNCTION main : DINT
        VAR
            x1, x2 : DINT;
        END_VAR
        x1 := SMC_Read(
                    ValAddr := ADR(main));
        END_FUNCTION
        FUNCTION SMC_Read : DINT
        VAR_INPUT
            ValAddr : LWORD;
        END_VAR
        END_FUNCTION
          ";
    let (unit, mut index) = index_with_ids(src, id_provider.clone());

    // THEN we check if the adr(main) really resolved to the return-variable
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let ass = &unit.implementations[0].statements[0];

    if let AstStatement::Assignment { right, .. } = ass {
        if let AstStatement::CallStatement { parameters, .. } = right.as_ref() {
            let inner_ass = ast::flatten_expression_list(parameters.as_ref().as_ref().unwrap())[0];
            if let AstStatement::Assignment { right, .. } = inner_ass {
                if let AstStatement::CallStatement { parameters, .. } = right.as_ref() {
                    let main =
                        ast::flatten_expression_list(parameters.as_ref().as_ref().unwrap())[0];
                    let a = annotations.get(main).unwrap();
                    assert_eq!(
                        a,
                        &StatementAnnotation::Variable {
                            resulting_type: "DINT".to_string(),
                            qualified_name: "main.main".to_string(),
                            constant: false,
                            variable_type: VariableType::Return,
                            is_auto_deref: false
                        }
                    )
                }
            }
        }
    }

    // AND we want a call passing the return-variable as apointer (actually the adress as a LWORD)
    assert_snapshot!(codegen(src));
}

#[test]
fn hardware_access_types_annotated() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "PROGRAM prg
        VAR
          x1,x2 : BYTE;
          y1,y2 : INT;
        END_VAR
          x1 := %IB1.2;
          x2 := %QW1.2;
          y1 := %MD1.2;
          y2 := %GX1.2;
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    if let AstStatement::Assignment { right, .. } = &unit.implementations[0].statements[0] {
        assert_type_and_hint!(&annotations, &index, &*right, BYTE_TYPE, Some(BYTE_TYPE));
    } else {
        unreachable!("Must be assignment")
    }
    if let AstStatement::Assignment { right, .. } = &unit.implementations[0].statements[1] {
        assert_type_and_hint!(&annotations, &index, &*right, WORD_TYPE, Some(BYTE_TYPE));
    } else {
        unreachable!("Must be assignment")
    }
    if let AstStatement::Assignment { right, .. } = &unit.implementations[0].statements[2] {
        assert_type_and_hint!(&annotations, &index, &*right, DWORD_TYPE, Some(INT_TYPE));
    } else {
        unreachable!("Must be assignment")
    }
    if let AstStatement::Assignment { right, .. } = &unit.implementations[0].statements[3] {
        assert_type_and_hint!(&annotations, &index, &*right, BOOL_TYPE, Some(INT_TYPE));
    } else {
        unreachable!("Must be assignment")
    }
}

#[test]
fn multiple_pointer_referencing_annotates_correctly() {
    let id_provider = IdProvider::default();
    // GIVEN a variable which is referenced multiple times with consecutive address operators
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM PRG
        VAR 
            a : BYTE; 
        END_VAR
            &&a;
            &&&a;
        END_PROGRAM",
        id_provider.clone(),
    );
    let mut annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;
    index.import(std::mem::take(&mut annotations.new_index));

    // THEN it is correctly annotated with nested pointers
    assert_type_and_hint!(
        &annotations,
        &index,
        &statements[0],
        "__POINTER_TO___POINTER_TO_BYTE",
        None
    );

    assert_type_and_hint!(
        &annotations,
        &index,
        &statements[1],
        "__POINTER_TO___POINTER_TO___POINTER_TO_BYTE",
        None
    );
}

#[test]
fn multiple_pointer_with_dereference_annotates_and_nests_correctly() {
    let id_provider = IdProvider::default();
    // GIVEN a parenthesized, double-pointer
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM PRG
        VAR 
            a : BYTE;
        END_VAR
            (&&a)^;
        END_PROGRAM",
        id_provider.clone(),
    );
    // WHEN it is dereferenced once
    let mut annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statement = &unit.implementations[0].statements[0];
    index.import(std::mem::take(&mut annotations.new_index));
    // THEN the expressions are nested and annotated correctly
    if let AstStatement::PointerAccess { reference, .. } = &statement {
        assert_type_and_hint!(
            &annotations,
            &index,
            reference,
            "__POINTER_TO___POINTER_TO_BYTE",
            None
        );

        if let AstStatement::UnaryExpression { value, .. } = &reference.as_ref() {
            assert_type_and_hint!(&annotations, &index, value, "__POINTER_TO_BYTE", None);

            if let AstStatement::UnaryExpression { value, .. } = &value.as_ref() {
                assert_type_and_hint!(&annotations, &index, value, "BYTE", None);
            }
        }
    } else {
        panic!("Not a pointer")
    }
    // AND the overall type of the statement is annotated correctly
    assert_type_and_hint!(&annotations, &index, statement, "__POINTER_TO_BYTE", None);
}

#[test]
fn multiple_negative_annotates_correctly() {
    let id_provider = IdProvider::default();
    // GIVEN a variable which is prefixed with two minus signs
    let (unit, mut index) = index_with_ids(
        "
        PROGRAM PRG
        VAR 
            a : DINT; 
        END_VAR
            --a;
            -(-a);
        END_PROGRAM",
        id_provider.clone(),
    );

    let mut annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[0].statements;
    index.import(std::mem::take(&mut annotations.new_index));

    // THEN it is correctly annotated
    if let AstStatement::UnaryExpression { value, .. } = &statements[0] {
        assert_type_and_hint!(&annotations, &index, value, DINT_TYPE, None);

        if let AstStatement::UnaryExpression { value, .. } = &value.as_ref() {
            assert_type_and_hint!(&annotations, &index, value, DINT_TYPE, None);
        }
    }

    if let AstStatement::UnaryExpression { value, .. } = &statements[1] {
        assert_type_and_hint!(&annotations, &index, value, DINT_TYPE, None);

        if let AstStatement::UnaryExpression { value, .. } = &value.as_ref() {
            assert_type_and_hint!(&annotations, &index, value, DINT_TYPE, None);
        }
    }
}

#[test]
fn array_of_struct_with_inital_values_annotated_correctly() {
    let id_provider = IdProvider::default();
    // GIVEN
    let (unit, mut index) = index_with_ids(
        "
        TYPE myStruct : STRUCT 
            	a,b : DINT; 
        	END_STRUCT
		END_TYPE

		PROGRAM main
		VAR
			arr : ARRAY[0..1] OF myStruct := ((a:= 10, b:= 20), (a:= 30, b:= 40));
		END_VAR
		END_PROGRAM",
        id_provider.clone(),
    );

    let mut annotations = annotate_with_ids(&unit, &mut index, id_provider);
    index.import(std::mem::take(&mut annotations.new_index));

    let container_name = &unit.implementations[0].name; // main
    let members = index.get_container_members(container_name);
    // there is only one member => arr
    assert_eq!(1, members.len());

    if let Some(AstStatement::ExpressionList { expressions, .. }) = index
        .get_const_expressions()
        .maybe_get_constant_statement(&members[0].initial_value)
    {
        // we initialized the array with 2 structs
        assert_eq!(2, expressions.len());
        let target_type = index
            .find_effective_type_by_name("myStruct")
            .expect("at this point we should have the type");
        for e in expressions {
            let type_hint = annotations
                .get_type_hint(e, &index)
                .expect("we should have a type hint");
            assert_eq!(target_type, type_hint);
        }
    } else {
        panic!("No initial value, initial value should be an expression list")
    }
}
