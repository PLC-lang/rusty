use crate::index::const_expressions::ConstExpression;
use crate::index::Index;
use crate::resolver::const_evaluator_new::{
    evaluate_constants_new, evaluate_expression, UnresolvableConstant,
};
use crate::test_utils::tests::{annotate_with_ids, codegen_with_new_constant_evaluator, index_with_ids};
use num::complex::ComplexFloat;
use num::integer::Roots;
use plc_ast::ast::AstStatement;
use plc_ast::ast::{AstFactory, AstNode};
use plc_ast::control_statements::{AstControlStatement, CaseStatement};
use plc_ast::literals::{Array, AstLiteral};
use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocation;
use plc_util::filtered_assert_snapshot;

const EMPTY: Vec<UnresolvableConstant> = vec![];

/// locally overwrite assert_eq to assert the Debug-Equality
macro_rules! debug_assert_eq {
    ($left:expr, $right:expr) => {
        assert_eq!(format!("{:#?}", $left), format!("{:#?}", $right))
    };
}

fn eval_constants(src: &str) -> (Index, Vec<UnresolvableConstant>) {
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(src, id_provider.clone());
    let mut annotations = annotate_with_ids(&parse_result, &mut index, id_provider);
    evaluate_constants_new(index, &mut annotations)
}

fn find_member_value<'a>(index: &'a Index, pou: &str, reference: &str) -> Option<&'a AstNode> {
    index
        .find_member(pou, reference)
        .and_then(|it| index.get_const_expressions().maybe_get_constant_statement(&it.initial_value))
}

fn find_constant_value<'a>(index: &'a Index, reference: &str) -> Option<&'a AstNode> {
    index
        .find_global_variable(reference)
        .and_then(|it| index.get_const_expressions().maybe_get_constant_statement(&it.initial_value))
}
fn create_int_literal(v: i128) -> AstNode {
    AstFactory::create_literal(AstLiteral::new_integer(v), SourceLocation::internal(), 0)
}

fn create_real_literal(v: f64) -> AstNode {
    AstFactory::create_literal(AstLiteral::new_real(format!("{v:}")), SourceLocation::internal(), 0)
}
fn create_bool_literal(v: bool) -> AstNode {
    AstFactory::create_literal(AstLiteral::new_bool(v), SourceLocation::internal(), 0)
}

fn create_string_literal(v: &str, wide: bool) -> AstNode {
    AstFactory::create_literal(AstLiteral::new_string(v.to_string(), wide), SourceLocation::internal(), 0)
}

fn create_null_literal() -> AstNode {
    AstFactory::create_literal(AstLiteral::new_null(), SourceLocation::internal(), 0)
}

fn create_array_literal(elements: Vec<AstNode>) -> AstNode {
    let loc = SourceLocation::internal();
    let list_node = AstFactory::create_expression_list(elements, loc.clone(), 0);
    AstFactory::create_literal(AstLiteral::new_array(Some(Box::new(list_node))), loc, 0)
}

fn extract_struct_as_strings(node: &AstNode) -> Vec<String> {
    let inner = match node.get_stmt() {
        AstStatement::ExpressionList(_) => node,
        AstStatement::ParenExpression(inner) => inner,
        AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) => elements,
        _ => panic!("Expected ExpressionList or ParenExpression, got: {:?}", node.get_stmt()),
    };
    let AstStatement::ExpressionList(elements) = inner.get_stmt() else {
        panic!("Not an ExpressionList: {:?}", inner.get_stmt());
    };

    fn extract_array_values(array_node: &AstNode) -> String {
        match array_node.get_stmt() {
            AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) => {
                let arr = extract_struct_as_strings(elements);
                format!("[{}]", arr.join(", "))
            }
            AstStatement::ExpressionList(inner) => {
                let arr = inner
                    .iter()
                    .map(|el| match el.get_stmt() {
                        AstStatement::Literal(AstLiteral::Integer(v)) => v.to_string(),
                        AstStatement::Literal(AstLiteral::Real(v)) => v.to_string(),
                        AstStatement::Literal(AstLiteral::Bool(v)) => v.to_string(),
                        AstStatement::Literal(AstLiteral::String(v)) => v.value.to_string(),
                        _ => format!("{:?}", el.get_stmt()),
                    })
                    .collect::<Vec<_>>();
                format!("[{}]", arr.join(", "))
            }
            _ => format!("{:?}", array_node.get_stmt()),
        }
    }

    elements
        .iter()
        .map(|child| {
            let value_node = match child.get_stmt() {
                AstStatement::Assignment(assignment) => &assignment.right,
                _ => child,
            };

            match value_node.get_stmt() {
                AstStatement::Literal(lit) => match lit {
                    AstLiteral::Integer(v) => v.to_string(),
                    AstLiteral::Real(v) => v.to_string(),
                    AstLiteral::Bool(v) => v.to_string(),
                    AstLiteral::String(v) => v.value.to_string(),
                    AstLiteral::Array(_) => extract_array_values(value_node),
                    _ => format!("{:?}", lit),
                },
                AstStatement::ExpressionList(_) => extract_array_values(value_node),
                _ => panic!("Expected Literal or ExpressionList, got: {:?}", value_node.get_stmt()),
            }
        })
        .collect()
}

fn get_expression_list_nodes(node: &AstNode) -> Vec<&AstNode> {
    match node.get_stmt() {
        AstStatement::Literal(AstLiteral::Array(Array { elements })) => {
            match elements {
                Some(inner_node) => match inner_node.get_stmt() {
                    // Fall: [1, 2, 3]
                    AstStatement::ExpressionList(inner) => inner.iter().collect(),
                    // Fall: [3(10)]
                    _ => vec![inner_node.as_ref()],
                },
                None => Vec::new(),
            }
        }
        AstStatement::ExpressionList(inner) => inner.iter().collect(),
        _ => vec![node],
    }
}

fn first_case_condition(unit: &plc_ast::ast::CompilationUnit) -> &AstNode {
    let AstStatement::ControlStatement(AstControlStatement::Case(CaseStatement { case_blocks, .. })) =
        unit.implementations[0].statements[0].get_stmt()
    else {
        panic!("Expected first implementation statement to be a CASE statement");
    };

    case_blocks[0].condition.as_ref()
}

fn assert_int_literal(node: &AstNode, expected: i128) {
    match node.get_stmt() {
        AstStatement::Literal(AstLiteral::Integer(v)) => assert_eq!(*v, expected),
        _ => panic!("Expected integer literal in array, got: {:?}", node.get_stmt()),
    }
}

fn assert_multiplied_integer(node: &AstNode, multiplier: u32, expected_val: i128) {
    match node.get_stmt() {
        AstStatement::MultipliedStatement(m) => {
            assert_eq!(m.multiplier, multiplier);
            match m.element.get_stmt() {
                AstStatement::Literal(AstLiteral::Integer(v)) => assert_eq!(*v, expected_val),
                _ => panic!(
                    "Expected integer literal inside multiplied statement, got: {:?}",
                    m.element.get_stmt()
                ),
            }
        }
        _ => panic!("Expected MultipliedStatement, got {:?}", node.get_stmt()),
    }
}

#[test]
fn evaluate_expression_resolves_case_condition_with_constants() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        r#"
        VAR_GLOBAL CONSTANT
            TWO : INT := 2;
            THREE : INT := 3;
        END_VAR

        PROGRAM main
            VAR
                selector : INT;
            END_VAR

            CASE selector OF
                TWO + THREE * 4:
                    selector := 1;
            END_CASE
        END_PROGRAM
        "#,
        id_provider,
    );
    let condition = first_case_condition(&unit);

    let evaluated = evaluate_expression(condition, Some("main"), &index).unwrap().unwrap();

    debug_assert_eq!(&evaluated, &create_int_literal(14));
}

#[test]
fn evaluate_expression_preserves_original_node_id_and_location() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        r#"
        PROGRAM main
            VAR
                selector : INT;
            END_VAR

            CASE selector OF
                1 + 2:
                    selector := 1;
            END_CASE
        END_PROGRAM
        "#,
        id_provider,
    );
    let condition = first_case_condition(&unit);

    let evaluated = evaluate_expression(condition, Some("main"), &index).unwrap().unwrap();

    assert_eq!(evaluated.get_id(), condition.get_id());
    assert_eq!(evaluated.get_location(), condition.get_location());
    debug_assert_eq!(&evaluated, &create_int_literal(3));
}

#[test]
fn evaluate_expression_reports_missing_type_info() {
    let expression = create_int_literal(1);

    let err = evaluate_expression(&expression, None, &Index::default()).unwrap_err();

    assert_eq!(err.get_reason(), "Type info for INT not found");
}

#[test]
fn const_int_arithmetic_resolves() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 4 + 6 * 2 - 3;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(13)));
}

#[test]
fn const_real_and_mixed_arithmetic_resolves() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            r1 : LREAL := 1.5 + 2;
            r2 : REAL  := 3;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "r1"), Some(create_real_literal(3.5)));
    debug_assert_eq!(find_constant_value(&index, "r2"), Some(create_real_literal(3.0)));
}

#[test]
fn two_const_real_and_mixed_arithmetic_resolves() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            iX : INT := 40;
            rX : LREAL := 40.2;
            iY : INT := iX;
            rY : LREAL := iX;
            iZ : INT := iY / 7;
            rZ : LREAL := rY / 7.7;
        END_VAR

        VAR_GLOBAL CONSTANT
            a : INT := iX;
            b : INT := iY;
            c : INT := iZ;
            d : LREAL := rX;
            e : LREAL := rY;
            f : LREAL := rZ;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(40), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(40), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(5), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_real_literal(40.2), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_real_literal(40.0), find_constant_value(&index, "e").unwrap());
    debug_assert_eq!(&create_real_literal(40_f64 / 7.7), find_constant_value(&index, "f").unwrap());
}

#[test]
fn const_references_int_float_type_behavior_evaluation() {
    // GIVEN constants of various types
    let (index, _) = eval_constants(
        "VAR_GLOBAL CONSTANT
            // INT - INT
            int_plus_int : INT := 3 + 1;
            int_minus_int : INT := 3 - 1;
            int_mul_int : INT := 3 * 2;
            int_div_int : INT := 5 / 2;
            int_mod_int : INT := 5 MOD 2;
            int_eq_int : INT := 5 = 5;
            int_neq_int : INT := 5 <> 5;
            int_g_int : INT := 5 > 5;
            int_ge_int : INT := 5 >= 5;
            int_l_int : INT := 5 < 5;
            int_le_int : INT := 5 <= 5;
            // INT - REAL
            int_plus_real : REAL := 3 + 1.1;
            int_minus_real : REAL := 3 - 1.1;
            int_mul_real : REAL := 3 * 1.1;
            int_div_real : REAL := 5 / 2.1;
            int_mod_real : REAL := 5 MOD 2.1;
            int_eq_real : BOOL := 5 = 2.1;
            int_neq_real : BOOL := 5 <> 2.1;
            int_g_real : BOOL := 5 > 5.0;
            int_ge_real : BOOL := 5 >= 5.0;
            int_l_real : BOOL := 5 < 5.0;
            int_le_real : BOOL := 5 <= 5.0;

            // REAL - INT
            real_plus_int : REAL := 3.3 + 1;
            real_minus_int : REAL := 3.3 - 1;
            real_mul_int : REAL := 3.3 * 2;
            real_div_int : REAL := 5.2 / 2;
            real_mod_int : REAL := 5.2 MOD 2;
            real_eq_int : BOOL := 5.2 = 2;
            real_neq_int : BOOL := 5.2 <> 2;
            real_g_int : BOOL := 5.0 > 5;
            real_ge_int : BOOL := 5.0 >= 5;
            real_l_int : BOOL := 5.0 < 5;
            real_le_int : BOOL := 5.0 <= 5;

            // REAL - REAL
            real_plus_real : REAL := 3.3 + 1.1;
            real_minus_real : REAL := 3.3 - 1.1;
            real_mul_real : REAL := 3.3 * 1.1;
            real_div_real : REAL := 5.3 / 2.1;
            real_mod_real : REAL := 5.3 MOD 2.1;
            real_eq_real : BOOL := 5.3 = 2.1;
            real_neq_real : BOOL := 5.3 <> 2.1;
            real_g_real : BOOL := 5.0 > 5.0;
            real_ge_real : BOOL := 5.0 >= 5.0;
            real_l_real : BOOL := 5.0 < 5.0;
            real_le_real : BOOL := 5.0 <= 5.0;

            // BOOL - BOOL
            _true_ : BOOL := TRUE;
            _false_ : BOOL := FALSE;
            bool_and_bool : BOOL := _true_ AND _true_;
            bool_or_bool : BOOL := _true_ OR _false_;
            bool_xor_bool : BOOL := _true_ XOR _true_;
            not_bool : BOOL := NOT _true_;
        END_VAR
        ",
    );

    // THEN all invalid type combinations should appear in unresolvable
    let mut expected_unresolvable = vec![
        "int_eq_int",
        "int_neq_int",
        "int_g_int",
        "int_ge_int",
        "int_l_int",
        "int_le_int",
        "int_eq_real",
        "int_neq_real",
        "int_g_real",
        "int_ge_real",
        "int_l_real",
        "int_le_real",
        "real_eq_int",
        "real_neq_int",
        "real_g_int",
        "real_ge_int",
        "real_l_int",
        "real_le_int",
        "real_eq_real",
        "real_neq_real",
        "real_g_real",
        "real_ge_real",
        "real_l_real",
        "real_le_real",
    ];
    expected_unresolvable.sort_unstable();

    let mut unresolvables: Vec<&str> = index
        .get_globals()
        .values()
        .filter(|it| {
            let const_expr =
                index.get_const_expressions().find_const_expression(it.initial_value.as_ref().unwrap());
            matches!(const_expr, Some(ConstExpression::Unresolvable { .. }))
        })
        .map(|it| it.get_qualified_name())
        .collect();
    unresolvables.sort_unstable();

    debug_assert_eq!(expected_unresolvable, unresolvables);

    // INT - INT
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "int_plus_int").unwrap());
    debug_assert_eq!(&create_int_literal(2), find_constant_value(&index, "int_minus_int").unwrap());
    debug_assert_eq!(&create_int_literal(6), find_constant_value(&index, "int_mul_int").unwrap());
    debug_assert_eq!(&create_int_literal(2), find_constant_value(&index, "int_div_int").unwrap());
    debug_assert_eq!(&create_int_literal(1), find_constant_value(&index, "int_mod_int").unwrap());

    // INT ↔ REAL
    debug_assert_eq!(&create_real_literal(4.1), find_constant_value(&index, "int_plus_real").unwrap());
    debug_assert_eq!(&create_real_literal(3.0 - 1.1), find_constant_value(&index, "int_minus_real").unwrap());
    debug_assert_eq!(&create_real_literal(3.0 * 1.1), find_constant_value(&index, "int_mul_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.0 / 2.1), find_constant_value(&index, "int_div_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.0 % 2.1), find_constant_value(&index, "int_mod_real").unwrap());

    // REAL ↔ INT
    debug_assert_eq!(&create_real_literal(4.3), find_constant_value(&index, "real_plus_int").unwrap());
    debug_assert_eq!(&create_real_literal(2.3), find_constant_value(&index, "real_minus_int").unwrap());
    debug_assert_eq!(&create_real_literal(6.6), find_constant_value(&index, "real_mul_int").unwrap());
    debug_assert_eq!(&create_real_literal(5.2 / 2.0), find_constant_value(&index, "real_div_int").unwrap());
    debug_assert_eq!(&create_real_literal(5.2 % 2.0), find_constant_value(&index, "real_mod_int").unwrap());

    // REAL ↔ REAL
    debug_assert_eq!(&create_real_literal(4.4), find_constant_value(&index, "real_plus_real").unwrap());
    debug_assert_eq!(
        &create_real_literal(3.3 - 1.1),
        find_constant_value(&index, "real_minus_real").unwrap()
    );
    debug_assert_eq!(&create_real_literal(3.3 * 1.1), find_constant_value(&index, "real_mul_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.3 / 2.1), find_constant_value(&index, "real_div_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.3 % 2.1), find_constant_value(&index, "real_mod_real").unwrap());

    // BOOL ↔ BOOL
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "bool_and_bool").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "bool_or_bool").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "bool_xor_bool").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "not_bool").unwrap());
}
#[test]
fn const_reference_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 4;
            b : INT := a + 1;
            c : INT := b + 1;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(4)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(5)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_int_literal(6)));
}

#[test]
fn array_size_from_constant() {
    // GIVEN some an array with const-expr -dimensions
    let (_, unresolvable) = eval_constants(
        r#"
        PROGRAM aaa
            VAR CONSTANT
                a : INT := 3;
                b : INT := 7;
            END_VAR

            VAR
                arr : ARRAY[a..b] OF BYTE;
            END_VAR
        END_PROGRAM
       "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
}
#[test]
fn array_members_from_constant() {
    // GIVEN some an array with const-expr -dimensions
    let (_index, unresolvable) = eval_constants(
        r#"
        PROGRAM aaa
            VAR CONSTANT
                a : INT := 3;
                b : INT := 7;
            END_VAR

            VAR
                arr : ARRAY[0..5] OF INT := [a,4,5,6,b];
            END_VAR
        END_PROGRAM
       "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
}

#[test]
fn const_enum_and_cast_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyEnum : (INIT := 5, RUNNING, STOPPED := 20); END_TYPE

        VAR_GLOBAL CONSTANT
            e : MyEnum := MyEnum.RUNNING; // Sollte 6 sein
            r : REAL := REAL#10;          // Expliziter Cast
            combined : REAL := REAL#MyEnum.INIT + 0.5; // Cast von Enum-Member
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "e"), Some(create_int_literal(6)));
    debug_assert_eq!(find_constant_value(&index, "r"), Some(create_real_literal(10.0)));
    debug_assert_eq!(find_constant_value(&index, "combined"), Some(create_real_literal(5.5)));
}

#[test]
fn const_complex_expressions_resolve() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            x : INT := +((5 + 3) * 2 - 4 / 2);
            y : REAL := +(-(1.5) + (2.0 * 3.0) / 2.0);
            a : INT := 5 AND 3 OR 8 XOR 2;
            b : INT := NOT(6) + 4;
            z : REAL := +((2 + 3) * 4 - 1.5) / 2.0;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "x"), Some(create_int_literal(14)));
    debug_assert_eq!(find_constant_value(&index, "y"), Some(create_real_literal(1.5)));
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(11)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(-3)));
    debug_assert_eq!(find_constant_value(&index, "z"), Some(create_real_literal(9.25)));
}

#[test]
fn const_paren_and_unary_plus_resolves() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := +(10 - (2 * 3));
            b : REAL := +(+1.5);
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(4)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_real_literal(1.5)));
}
#[test]
fn const_multiplied_statement_isolated_resolves() {
    let (_index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..2] OF INT := 3(3(3 + 5));
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
}

#[test]
fn test_simple_array() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..2] OF INT := [1,2,3];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let arr = create_array_literal(vec![create_int_literal(1), create_int_literal(2), create_int_literal(3)]);
    debug_assert_eq!(find_constant_value(&index, "arr").unwrap(), arr);
}

#[test]
fn const_in_other_scope() {
    let (index, unresolvable) = eval_constants(
        r#"
                VAR_GLOBAL CONSTANT
                    a : INT := 20;
                END_VAR
                PROGRAM prg
                    VAR CONSTANT
                        a : INT := 7;
                        arr : ARRAY[0..a] OF INT;
                    END_VAR
                    VAR
                        y : INT := a + 1;
                    END_VAR
                END_PROGRAM
                "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(20)));
    debug_assert_eq!(find_member_value(&index, "prg", "a"), Some(create_int_literal(7)));
    debug_assert_eq!(find_member_value(&index, "prg", "y"), Some(create_int_literal(8)));
}

#[test]
fn constants_with_array_sizes_and_shadowing() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            max_size : INT := 5;
        END_VAR

       PROGRAM prog
            VAR CONSTANT
                max_size : INT := 3;
                arr : ARRAY[0..max_size] OF INT;
            END_VAR

            VAR
                sum : INT := arr[0];
            END_VAR
        END_PROGRAM
        "#,
    );

    debug_assert_eq!(unresolvable.len(), 1);

    debug_assert_eq!(find_constant_value(&index, "max_size"), Some(create_int_literal(5)));

    debug_assert_eq!(find_member_value(&index, "prog", "max_size"), Some(create_int_literal(3)));

    assert!(unresolvable[0].get_reason().unwrap().contains("default"));
}

#[test]
fn local_const_reference_resolves() {
    let (index, unresolvable) = eval_constants(
        "PROGRAM prg
            VAR CONSTANT
                x : INT := 4;
            END_VAR
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            y : INT := prg.x + 2;
            c : INT := y + prg.x;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "y"), Some(create_int_literal(6)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_int_literal(10)));
}

#[test]
fn unary_not_resolves_bool() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            b : BOOL := NOT FALSE;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_bool_literal(true)));
}

#[test]
fn test_bool_with_numbers() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : BOOL := true;
            b : BOOL := false;
            c : BOOL := TRUE;
            d : BOOL := FALSE;
            e : BOOL := 1;
            f : BOOL := 0;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_bool_literal(false)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "d"), Some(create_bool_literal(false)));
    debug_assert_eq!(find_constant_value(&index, "e"), Some(create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "f"), Some(create_bool_literal(false)));
}

#[test]
fn comparisons_resolve_to_bool() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            b1 : BOOL := 5 > 3;
            b2 : BOOL := 3 <= 3;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "b1"), Some(create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "b2"), Some(create_bool_literal(true)));
}

#[test]
fn comparisons_with_reals_are_unresolvable() {
    let (index, _unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            i_ok : BOOL := 5 > 3;
            r_eq : BOOL := 3.5 = 3.5;
            r_lt : BOOL := 3.5 < 4.0;
            i_r_cmp : BOOL := 3 < 3.5;
        END_VAR",
    );

    let mut unresolvables: Vec<&str> = index
        .get_globals()
        .values()
        .filter(|it| {
            let const_expr =
                index.get_const_expressions().find_const_expression(it.initial_value.as_ref().unwrap());
            matches!(const_expr, Some(ConstExpression::Unresolvable { .. }))
        })
        .map(|it| it.get_qualified_name())
        .collect();
    unresolvables.sort_unstable();

    debug_assert_eq!(unresolvables, vec!["i_r_cmp", "r_eq", "r_lt"]);
    debug_assert_eq!(find_constant_value(&index, "i_ok"), Some(create_bool_literal(true)));
}

#[test]
fn detect_cycle_int_declaration() {
    let (_index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := b;
            b : INT := c;
            c : INT := d;
            d : INT := e;
            e : INT := a;
            good : INT := 20;
        END_VAR",
    );

    debug_assert_eq!(unresolvable.len(), 5);
}

#[test]
fn bitwise_int_works() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 6 AND 3;
            b : INT := 6 XOR 3;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(2)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(5)));
}

#[test]
fn no_forward_declaration() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := b;
            b : INT := c;
            c : INT := d;
            d : INT := 10;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(10)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(10)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_int_literal(10)));
    debug_assert_eq!(find_constant_value(&index, "d"), Some(create_int_literal(10)));
}

#[test]
fn division_by_zero_is_unresolvable() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 10 / 0;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("divide by zero"));
}

#[test]
fn modulo_by_zero_is_unresolvable() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 10 MOD 0;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("remainder"));
}

#[test]
fn bool_used_in_binary_expression_is_unresolvable() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            b : BOOL := true;
            a : INT := 5 + b;
        END_VAR",
    );
    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0]
        .get_reason()
        .unwrap()
        .contains("Unsupported binary operation in const_evaluator"));
}

#[test]
fn bool_in_binary_is_unresolvable() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : BOOL := TRUE AND FALSE;
            b : BOOL := TRUE AND TRUE;
            c : BOOL := FALSE AND FALSE;
        END_VAR",
    );

    assert_eq!(unresolvable, EMPTY);
    debug_assert_eq!(create_bool_literal(false), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(create_bool_literal(true), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(create_bool_literal(false), find_constant_value(&index, "c").unwrap());
}

#[test]
fn bool_to_int_cast_is_unresolvable() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := TRUE;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("Cannot convert value of type BOOL to INT"));
}

#[test]
fn boolean_mega_test_is_resolvable() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            // 1. Einfache Casts & NOT
            a : BOOL := 1;
            b : BOOL := 0;

            e : BOOL := 0 AND 1;
            h : BOOL := 1 AND 1;
            i : BOOL := 0 AND 0;

            j : BOOL := 1 OR 0;
            k : BOOL := 1 XOR 1;
            l : BOOL := 0 XOR 1;

            m : BOOL := 10 > 5;
            n : BOOL := 10 = 10;
            o : BOOL := 10 <> 10;

            q : BOOL := 0 OR 1 AND 0;
            r : BOOL := NOT 0 AND 1;
        END_VAR",
    );

    assert_eq!(unresolvable, EMPTY);

    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "e").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "h").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "i").unwrap());

    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "j").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "k").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "l").unwrap());

    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "m").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "n").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "o").unwrap());

    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "q").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "r").unwrap());
}

#[test]
fn real_to_bool_cast_is_unresolvable() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : BOOL := true;
            b : BOOL := 0.0;
            c: BOOL := 1.0;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 2);
    assert!(unresolvable[0].get_reason().unwrap().contains("Cannot convert value of type REAL to BOOL"));
    assert!(unresolvable[1].get_reason().unwrap().contains("Cannot convert value of type REAL to BOOL"));
}

#[test]
fn bool_to_real_cast_is_unresolvable() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            r : REAL := TRUE;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("Cannot convert value of type BOOL to REAL"));
}

#[test]
fn int_target_overflow_is_detected() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : SINT := 120 + 20;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("This will overflow"));
}

#[test]
fn unsigned_negative_overflow_is_detected() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : USINT := -1;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("This will overflow"));
}

#[test]
fn real_to_int_overflow_is_detected() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 1.0e20;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("This will overflow"));
}

#[test]
fn real_target_overflow_is_detected() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : REAL := 1e39;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("This will overflow"));
}

#[test]
fn lreal_non_finite_overflow_is_detected() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : LREAL := 1.0e309;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("This will overflow"));
}

#[test]
fn real_to_int_cast_rounds() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 1.6;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(1)));
}

#[test]
fn real_to_int_cast_rounds_negative() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := -1.5;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(-1)));
}

#[test]
fn non_const_reference_is_unresolvable() {
    let (_, unresolvable) = eval_constants(
        "VAR_GLOBAL
            a : INT := 3;
        END_VAR

        VAR_GLOBAL CONSTANT
            b : INT := a + 1;
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("no const reference"));
}

#[test]
fn local_member_value_is_resolved() {
    let (index, unresolvable) = eval_constants(
        "PROGRAM prg
            VAR CONSTANT
                x : INT := 7;
            END_VAR
            VAR
                y : INT := x + 1;
            END_VAR
        END_PROGRAM",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_member_value(&index, "prg", "y"), Some(create_int_literal(8)));
}

#[test]
fn const_struct_default_initializer_is_resolved() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyStruct:
             STRUCT
                a : INT;
                b : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            myStruct : MyStruct := (a := 42, b := 100);
            myStruct2 : MyStruct := (666, 14500);
            a : INT := s.a;
            b : INT := s.b;
        END_VAR
        "#,
    );

    debug_assert_eq!(2, unresolvable.len());

    let act_my_struct = extract_struct_as_strings(find_constant_value(&index, "myStruct").unwrap());
    debug_assert_eq!(act_my_struct[0], "42");
    debug_assert_eq!(act_my_struct[1], "100");

    let act_my_struct = extract_struct_as_strings(find_constant_value(&index, "myStruct2").unwrap());
    debug_assert_eq!(act_my_struct[0], "666");
    debug_assert_eq!(act_my_struct[1], "14500");

    unresolvable[0].get_reason().unwrap().contains("Struct field access is not supported in a Constant!");
    unresolvable[1].get_reason().unwrap().contains("Struct field access is not supported in a Constant!");
}

#[test]
fn const_array_of_struct_initializer_is_resolved() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Sensor : STRUCT
            id : INT;
            value : REAL;
            active : BOOL;
        END_STRUCT END_TYPE

        VAR_GLOBAL CONSTANT
            sensors : ARRAY[0..1] OF Sensor := [
                (id := 10, value := 1.1, active := TRUE),
                (id := 11, value := 2.2, active := FALSE)
            ];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);

    let sensors_ast = find_constant_value(&index, "sensors").expect("sensors const should exist");
    assert!(matches!(sensors_ast.get_stmt(), AstStatement::Literal(AstLiteral::Array(_))));
}

#[test]
fn const_tyoe_default_initializer_is_resolved() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE OneInt : INT := 1; END_TYPE

        VAR_GLOBAL CONSTANT
            MAX_SIZE : INT := 99;
            MIN_LEN : INT;
            counter : OneInt;
        END_VAR

        PROGRAM PLC_PRG
            VAR CONSTANT
                DEFAULT_INPUT : BOOL;
            END_VAR
        END_PROGRAM
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(99), find_constant_value(&index, "MAX_SIZE").unwrap());
    debug_assert_eq!(&create_int_literal(0), find_constant_value(&index, "MIN_LEN").unwrap());
    debug_assert_eq!(&create_int_literal(1), find_constant_value(&index, "counter").unwrap());
    debug_assert_eq!(
        &create_bool_literal(false),
        find_member_value(&index, "PLC_PRG", "DEFAULT_INPUT").unwrap()
    );
}

#[test]
fn const_variable_with_default_initializer() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            a : INT;
            b : INT := a + 1;
            c : INT := 120 / 0;
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable.len(), 1);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(0)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(1)));
    assert!(unresolvable[0].get_reason().unwrap().contains("Attempt to divide by zero"));
}

#[test]
fn const_array_of_struct_default_initializer_is_resolved() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE MyStruct : STRUCT
            a : INT;
        END_STRUCT END_TYPE

        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..1] OF MyStruct;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
}
#[test]
fn test_with_array_and_values() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            a : REAL := 40958309485309485;
            c : REAL := -a;
            d : REAL := 3.33;
            arr : ARRAY[0..4] OF INT := [b, 2, d, 4];
            b : INT := 20;
        END_VAR
        "#,
    );
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_real_literal(40958309485309485.0), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_real_literal(-40958309485309485.0), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_real_literal(3.33), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_int_literal(20), find_constant_value(&index, "b").unwrap());
    let arr = create_array_literal(vec![
        create_int_literal(20),
        create_int_literal(2),
        create_int_literal(3),
        create_int_literal(4),
    ]);
    debug_assert_eq!(&arr, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn test_const_array_heavy_integers_and_hex() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            MAX_LINT : LINT := 16#7FFFFFFFFFFFFFFF;
            OFFSET   : LINT := 16#0000000000000001;
            arr : ARRAY[0..2] OF LINT := [MAX_LINT, MAX_LINT - OFFSET, 16#FF_FF_FF_FF];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);

    let expected_arr = create_array_literal(vec![
        create_int_literal(9223372036854775807),
        create_int_literal(9223372036854775806),
        create_int_literal(4294967295),
    ]);
    debug_assert_eq!(&expected_arr, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn test_const_array_heavy_reals() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            PI : LREAL := 3.141592653589793;
            EPSILON : LREAL := 1.0E-10;
            arr : ARRAY[0..1] OF LREAL := [PI * 2.0, 1.0 + EPSILON];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);

    let expected_arr =
        create_array_literal(vec![create_real_literal(6.283185307179586), create_real_literal(1.0000000001)]);
    debug_assert_eq!(&expected_arr, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn test_const_array_sint_overflow_fails() {
    let (_, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            TOO_BIG : INT := 500;
            myArr : ARRAY[0..0] OF SINT := 0;
            myArr1 : ARRAY[0..0] OF SINT := 0.0;
            myArr2 : ARRAY[0..0] OF SINT := 250.0;
            myArr3 : ARRAY[0..0] OF SINT := [TOO_BIG];
            myArr4 : ARRAY[0..1] OF SINT := [TOO_BIG, TOO_BIG];
        END_VAR
        "#,
    );

    //codegen will handle the invalid statements!
    debug_assert_eq!(unresolvable.len(), 2);
    debug_assert_eq!(unresolvable[0].get_reason().expect("REASON").contains("overflow"), true);
    debug_assert_eq!(unresolvable[1].get_reason().expect("REASON").contains("overflow"), true);
}

#[test]
fn test_const_array_deep_nesting() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            FACTOR : INT := 3;
            arr : ARRAY[0..0] OF INT := [((100 + 50) * FACTOR) / (10 - 7)];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let expected_arr = create_array_literal(vec![create_int_literal(150)]);
    debug_assert_eq!(&expected_arr, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn nested_array_literals_multiplied_statement_type_resolving() {
    // GIVEN a multi-nested Array Type with an initializer
    let (_index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            TWO : INT := 2;
            THREE : INT := 3;
            a : ARRAY[0..1] OF ARRAY[0..1] OF INT  := [[2(TWO)],[2(THREE)]];
            b : ARRAY[0..1] OF ARRAY[0..1] OF INT  := [[2]];
        END_VAR
       "#,
    );
    debug_assert_eq!(unresolvable, EMPTY);
}

#[test]
fn test_const_resolved_array_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr3 : ARRAY[0..4] OF LINT := [1, a, 3, b];
            calcArray : ARRAY[0..1] OF INT := [(1 + 1) * 100, 5 * 2];
            a : LINT := 229384;
            b : LINT := a / 2;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(229384), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(229384 / 2), find_constant_value(&index, "b").unwrap());
    let expected_calc_array = create_array_literal(vec![create_int_literal(200), create_int_literal(10)]);
    debug_assert_eq!(&expected_calc_array, find_constant_value(&index, "calcArray").unwrap());

    let expected_arr3 = create_array_literal(vec![
        create_int_literal(1),
        create_int_literal(229384),
        create_int_literal(3),
        create_int_literal(114692),
    ]);
    debug_assert_eq!(&expected_arr3, find_constant_value(&index, "arr3").unwrap());
}

#[test]
fn test_multiplied_simple_repeat() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..2] OF INT := [3(10)];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let arr_node = find_constant_value(&index, "arr").unwrap();
    let elements = get_expression_list_nodes(arr_node);
    assert_eq!(elements.len(), 1);
    assert_multiplied_integer(elements[0], 3, 10);
}

#[test]
fn test_multiplied_mixed_with_literals() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..3] OF INT := [1, 2(0), 5];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let arr_node = find_constant_value(&index, "arr").unwrap();
    let elements = get_expression_list_nodes(arr_node);
    assert_eq!(elements.len(), 3);
    assert_int_literal(elements[0], 1);
    assert_multiplied_integer(elements[1], 2, 0);
    assert_int_literal(elements[2], 5);
}

#[test]
fn test_multiplied_with_expression_element() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..1] OF INT := [2(5 + 5)];
            arrN : ARRAY[0..5] OF INT := [2(1, 2), 2(0)];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);

    let arr_node = find_constant_value(&index, "arr").unwrap();
    let arr_elements = get_expression_list_nodes(arr_node);
    assert_eq!(arr_elements.len(), 1);
    assert_multiplied_integer(arr_elements[0], 2, 10);

    let arr_n_node = find_constant_value(&index, "arrN").unwrap();
    let arr_n_elements = get_expression_list_nodes(arr_n_node);
    assert_eq!(arr_n_elements.len(), 2);

    match arr_n_elements[0].get_stmt() {
        AstStatement::MultipliedStatement(mult) => {
            assert_eq!(mult.multiplier, 2);
            let inner = &mult.element;
            let inner_values = get_expression_list_nodes(inner);
            assert_eq!(inner_values.len(), 2);
            assert_int_literal(inner_values[0], 1);
            assert_int_literal(inner_values[1], 2);
        }
        _ => panic!("Expected top-level multiplied statement at arrN[0]"),
    }

    assert_multiplied_integer(arr_n_elements[1], 2, 0);
}

#[test]
fn test_multiplied_with_var_ref() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            ele : INT := 7;
            arr : ARRAY[0..3] OF INT := [2(2(ele - (2 - 2)))];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(create_int_literal(7), find_constant_value(&index, "ele").unwrap());

    let arr_node = find_constant_value(&index, "arr").unwrap();
    let arr_elements = get_expression_list_nodes(arr_node);
    assert_eq!(arr_elements.len(), 1);

    match arr_elements[0].get_stmt() {
        AstStatement::MultipliedStatement(outer) => {
            assert_eq!(outer.multiplier, 2);
            match outer.element.get_stmt() {
                AstStatement::MultipliedStatement(inner) => {
                    assert_eq!(inner.multiplier, 2);
                    assert_int_literal(&inner.element, 7);
                }
                _ => panic!("Expected nested MultipliedStatement, got: {:?}", outer.element.get_stmt()),
            }
        }
        _ => panic!("Expected top-level MultipliedStatement, got: {:?}", arr_elements[0].get_stmt()),
    }
}

#[test]
fn test_string_literal_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            s : STRING := 'hello';
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "s"), Some(create_string_literal("hello", false)));
}

#[test]
fn test_wstring_literal_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            ws : WSTRING := "hello";
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "ws"), Some(create_string_literal("hello", true)));
}

#[test]
fn test_string_reference_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            a : STRING := 'alpha';
            b : STRING := a;
            c : STRING := b;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_string_literal("alpha", false)));
}

#[test]
fn test_wstring_reference_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            a : WSTRING := "beta";
            b : WSTRING := a;
            c : WSTRING := b;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_string_literal("beta", true)));
}

#[test]
fn test_string_wstring_cross_cast_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            s : STRING := 'Hello';
            w : WSTRING := s;
            s2 : STRING := w;
            s3 : STRING := s + w;
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable.len(), 1);
    debug_assert_eq!(find_constant_value(&index, "w"), Some(create_string_literal("Hello", true)));
    debug_assert_eq!(find_constant_value(&index, "s2"), Some(create_string_literal("Hello", false)));
    assert!(unresolvable[0].get_reason().unwrap().contains("ot supported for STRING/WSTRING types"));
}

#[test]
fn test_string_array_literal_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..2] OF STRING := ['a', 'b', 'c'];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let expected = create_array_literal(vec![
        create_string_literal("a", false),
        create_string_literal("b", false),
        create_string_literal("c", false),
    ]);
    debug_assert_eq!(&expected, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn test_wstring_array_literal_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..1] OF WSTRING := ["x", "y"];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let expected =
        create_array_literal(vec![create_string_literal("x", true), create_string_literal("y", true)]);
    debug_assert_eq!(&expected, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn test_string_default_value_resolves_empty() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            s : STRING;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "s"), Some(create_string_literal("", false)));
}

#[test]
fn test_wstring_default_value_resolves_empty() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            ws : WSTRING;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "ws"), Some(create_string_literal("", true)));
}

#[test]
fn test_string_type_alias_default_initializer_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE AliasStr : STRING := 'seed'; END_TYPE

        VAR_GLOBAL CONSTANT
            a : AliasStr;
            b : STRING := a;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_string_literal("seed", false)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_string_literal("seed", false)));
}

#[test]
fn test_byte_literal_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            b : BYTE := 42;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(42)));
}

#[test]
fn test_byte_arithmetic_expression_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            b : BYTE := (10 + 5) * 3 - 20;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(25)));
}

#[test]
fn test_byte_bitwise_expression_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            a : BYTE := 6 AND 3;
            b : BYTE := 6 OR 3;
            c : BYTE := 6 XOR 3;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(2)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(7)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_int_literal(5)));
}

#[test]
fn test_byte_reference_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            b0 : BYTE := 7;
            b1 : BYTE := b0 + 1;
            b2 : BYTE := b1 * 2;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "b2"), Some(create_int_literal(16)));
}

#[test]
fn test_byte_division_and_modulo_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            q : BYTE := 100 / 6;
            r : BYTE := 100 MOD 6;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "q"), Some(create_int_literal(16)));
    debug_assert_eq!(find_constant_value(&index, "r"), Some(create_int_literal(4)));
}

#[test]
fn test_byte_array_literal_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..3] OF BYTE := [1, 2, 3, 4];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let expected = create_array_literal(vec![
        create_int_literal(1),
        create_int_literal(2),
        create_int_literal(3),
        create_int_literal(4),
    ]);
    debug_assert_eq!(&expected, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn test_byte_array_with_expressions_and_refs_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            x : BYTE := 3;
            y : BYTE := 10;
            arr : ARRAY[0..3] OF BYTE := [x, y, x + y, y MOD x];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let expected = create_array_literal(vec![
        create_int_literal(3),
        create_int_literal(10),
        create_int_literal(13),
        create_int_literal(1),
    ]);
    debug_assert_eq!(&expected, find_constant_value(&index, "arr").unwrap());
}

#[test]
fn test_byte_multiplied_array_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..5] OF BYTE := [2(1), 2(2), 2(3)];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let arr_node = find_constant_value(&index, "arr").unwrap();
    let elements = get_expression_list_nodes(arr_node);
    assert_eq!(elements.len(), 3);
    assert_multiplied_integer(elements[0], 2, 1);
    assert_multiplied_integer(elements[1], 2, 2);
    assert_multiplied_integer(elements[2], 2, 3);
}

#[test]
fn test_byte_nested_multiplied_array_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..7] OF BYTE := [2(2(1,2))];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let arr_node = find_constant_value(&index, "arr").unwrap();
    let elements = get_expression_list_nodes(arr_node);
    assert_eq!(elements.len(), 1);

    match elements[0].get_stmt() {
        AstStatement::MultipliedStatement(outer) => {
            assert_eq!(outer.multiplier, 2);
            match outer.element.get_stmt() {
                AstStatement::MultipliedStatement(inner) => {
                    assert_eq!(inner.multiplier, 2);

                    let inner_node = match inner.element.get_stmt() {
                        AstStatement::ExpressionList(e) => AstNode::new(
                            AstStatement::ExpressionList(e.clone()),
                            inner.element.id,
                            inner.element.location.clone(),
                        ),
                        _ => panic!(
                            "Expected ExpressionList inside nested multiplied statement, got: {:?}",
                            inner.element.get_stmt()
                        ),
                    };

                    let inner_values = get_expression_list_nodes(&inner_node);
                    assert_eq!(inner_values.len(), 2);
                    assert_int_literal(inner_values[0], 1);
                    assert_int_literal(inner_values[1], 2);
                }
                _ => panic!("Expected nested MultipliedStatement, got: {:?}", outer.element.get_stmt()),
            }
        }
        _ => panic!("Expected top-level MultipliedStatement, got: {:?}", elements[0].get_stmt()),
    }
}

#[test]
fn test_byte_scope_shadowing_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            b : BYTE := 4;
        END_VAR

        PROGRAM prg
            VAR CONSTANT
                b : BYTE := 9;
            END_VAR
            VAR
                y : BYTE := b + 1;
            END_VAR
        END_PROGRAM
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(4)));
    debug_assert_eq!(find_member_value(&index, "prg", "b"), Some(&create_int_literal(9)));
    debug_assert_eq!(find_member_value(&index, "prg", "y"), Some(&create_int_literal(10)));
}

#[test]
fn test_pointer_default_null_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            p : POINTER TO INT;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "p"), Some(create_null_literal()));
}

#[test]
fn test_pointer_explicit_null_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            p : POINTER TO INT := NULL;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "p"), Some(create_null_literal()));
}

#[test]
fn test_pointer_reference_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            p0 : POINTER TO INT := NULL;
            p1 : POINTER TO INT := p0;
            p2 : POINTER TO INT := p1;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "p2"), Some(create_null_literal()));
}

#[test]
fn test_pointer_array_with_nulls_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..2] OF POINTER TO INT := [NULL, NULL, NULL];
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let expected =
        create_array_literal(vec![create_null_literal(), create_null_literal(), create_null_literal()]);
    debug_assert_eq!(&expected, find_constant_value(&index, "arr").unwrap());
}

//TODO hm??
#[test]
fn test_pointer_null_comparisons_resolve() {
    let (_index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            p : POINTER TO INT := NULL;
            eq_null : BOOL := p = NULL;
            neq_null : BOOL := p <> NULL;
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable.len(), 2);
}

#[test]
fn test_pointer_type_alias_default_and_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE PInt : POINTER TO INT; END_TYPE

        VAR_GLOBAL CONSTANT
            p0 : PInt;
            p1 : PInt := p0;
            p2 : POINTER TO INT := p1;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "p0"), Some(create_null_literal()));
    debug_assert_eq!(find_constant_value(&index, "p2"), Some(create_null_literal()));
}

#[test]
fn const_variables_default_value_compile_time_evaluation_t() {
    // GIVEN some Iconstants index used as initializers
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE myEnum : (a,b,c); END_TYPE
        VAR_GLOBAL CONSTANT
            false_bool      : BOOL;
            zero_int        : INT;
            zero_real       : LREAL;
            empty_string    : STRING;
            empty_wString   : WSTRING;
            null_ptr        : POINTER TO INT;
            zero_enum       : myEnum;
        END_VAR
        "#,
    );

    debug_assert_eq!(find_constant_value(&index, "false_bool").unwrap(), create_bool_literal(false));
    debug_assert_eq!(find_constant_value(&index, "zero_int").unwrap(), create_int_literal(0));
    debug_assert_eq!(find_constant_value(&index, "zero_real").unwrap(), create_real_literal(0.0));
    debug_assert_eq!(find_constant_value(&index, "empty_string").unwrap(), create_string_literal("", false));
    debug_assert_eq!(find_constant_value(&index, "empty_wString").unwrap(), create_string_literal("", true));
    debug_assert_eq!(find_constant_value(&index, "null_ptr").unwrap(), create_null_literal());

    debug_assert_eq!(unresolvable, EMPTY);

    debug_assert_eq!(find_constant_value(&index, "a").unwrap(), create_int_literal(0));
    debug_assert_eq!(find_constant_value(&index, "b").unwrap(), create_int_literal(1));
    debug_assert_eq!(find_constant_value(&index, "c").unwrap(), create_int_literal(2));
    debug_assert_eq!(find_constant_value(&index, "zero_enum").unwrap(), create_int_literal(0));
}

#[test]
fn const_variables_default_value_compile_time_evaluation_two() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE myEnum3 : (valA, valB := 10, valC) ; END_TYPE
        VAR_GLOBAL CONSTANT
            a : myEnum3 := ValA;
            b : myEnum3 := valB;
            c : myEnum3 := valC;
        END_VAR
        "#,
    );
    debug_assert_eq!(unresolvable, EMPTY);
    debug_assert_eq!(find_constant_value(&index, "a").unwrap(), create_int_literal(0));
    debug_assert_eq!(find_constant_value(&index, "valA").unwrap(), create_int_literal(0));
    debug_assert_eq!(find_constant_value(&index, "b").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "valB").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "c").unwrap(), create_int_literal(11));
    debug_assert_eq!(find_constant_value(&index, "valC").unwrap(), create_int_literal(11));
}

#[test]
fn const_variables_with_range_statement() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE MyInt : INT END_TYPE
        VAR_GLOBAL CONSTANT
            ExplosionLevel : MyInt(1..10) := 5;
        END_VAR
        "#,
    );
    debug_assert_eq!(unresolvable, EMPTY);
}

#[test]
fn const_variables_with_range_statement_one() {
    let src = codegen_with_new_constant_evaluator(
        r#"
           TYPE Point :
        STRUCT
            x : INT;
            y : INT;
        END_STRUCT
    END_TYPE
    VAR_GLOBAL CONSTANT
        b : ARRAY[0..1] OF Point := [(x := 10)];
        a : ARRAY[0..1] OF Point := [(x := 10, y := 20)];
    END_VAR
        "#,
    );

    filtered_assert_snapshot!(src);
}

#[test]
fn codegen_const_struct_field_access_from_global_struct() {
    let src = codegen_with_new_constant_evaluator(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE
        PROGRAM PRG
            VAR CONSTANT
                point : Point := (x := 10, y := 20);
            END_VAR
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            xValue : INT := prg.point.x;
            yValue : INT := prg.point.y;
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(src);
}

#[test]
fn codegen_const_nested_struct_field_access() {
    let src = codegen_with_new_constant_evaluator(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        TYPE Wrapper :
            STRUCT
                point : Point;
                scale : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            wrapper : Wrapper := (point := (x := 7, y := 8), scale := 2);
            nestedX : INT := wrapper.point.x;
            nestedY : INT := wrapper.point.y;
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(src);
}

#[test]
fn codegen_const_array_index_access_from_global_array() {
    let src = codegen_with_new_constant_evaluator(
        r#"
        VAR_GLOBAL CONSTANT
            values : ARRAY[0..3] OF INT := [3, 5, 8, 13];
            first : INT := values[0];
            third : INT := values[2];
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(src);
}

#[test]
fn codegen_const_array_of_struct_field_access() {
    let src = codegen_with_new_constant_evaluator(
        r#"
        TYPE Entry :
            STRUCT
                id : INT;
                value : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            entries : ARRAY[0..1] OF Entry := [
                (id := 1, value := 100),
                (id := 2, value := 200)
            ];
            secondId : INT := entries[1].id;
            firstValue : INT := entries[0].value;
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(src);
}

#[test]
fn codegen_const_program_struct_array_mixed_access() {
    let src = codegen_with_new_constant_evaluator(
        r#"
        TYPE Metrics :
            STRUCT
                samples : ARRAY[0..2] OF INT;
                offset : INT;
            END_STRUCT
        END_TYPE

        TYPE Config :
            STRUCT
                metrics : Metrics;
                entries : ARRAY[0..1] OF Metrics;
            END_STRUCT
        END_TYPE

        PROGRAM prg
            VAR CONSTANT
                config : Config := (
                    metrics := (samples := [10, 20, 30], offset := 5),
                    entries := [
                        (samples := [1, 2, 3], offset := 10),
                        (samples := [4, 5, 6], offset := 20)
                    ]
                );
            END_VAR
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            directSample : INT := prg.config.metrics.samples[1];
            nestedSample : INT := prg.config.entries[1].samples[2];
            nestedOffset : INT := prg.config.entries[0].offset;
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(src);
}

#[test]
fn const_variables_default_value_compile_time_evaluation_three() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyINT : INT := 7; END_TYPE

        TYPE MyRange : MyINT(1..10); END_TYPE

        VAR_GLOBAL
            a : MyINT;
            b : MyRange;
        END_VAR

        VAR_GLOBAL CONSTANT
            aa : MyINT;
            bb : MyRange;
            cc : INT := bb + aa;
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);
    debug_assert_eq!(&create_int_literal(7), find_constant_value(&index, "aa").unwrap());
    debug_assert_eq!(&create_int_literal(7), find_constant_value(&index, "bb").unwrap());
    debug_assert_eq!(&create_int_literal(14), find_constant_value(&index, "cc").unwrap());
}

#[test]
fn find_range_statements() {
    let (index, unresolvable) = eval_constants(
        r#"
        //TYPE MyINT: INT(1..1000) := 5; END_TYPE

        VAR_GLOBAL CONSTANT
            /*aa : MyINT;
            bb : MyINT := 55;
            bb : MyINT := 5555;*/
            x : SINT := -128;
            y : SINT := NOT NOT NOT x;
        END_VAR
        "#,
    );

    print!("HALLO");
    debug_assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("is out of SubRange "));
    debug_assert_eq!(&create_int_literal(5), find_constant_value(&index, "aa").unwrap());
    debug_assert_eq!(&create_int_literal(55), find_constant_value(&index, "bb").unwrap());
}

#[test]
fn test_struct_with_no_default_values() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            myPoint : Point := (x := 10, y := 20);
            myPointC : Point := (10, 20);
            myPointDefaultValues : Point;

            mPx : INT := myPoint.x;
            mPxx : INT := myPointC.x;
            mPy : INT := myPoint.y;
            mPoDx : INT := myPointDefaultValues.x;
            mPoDy : INT := myPointDefaultValues.y;
        END_VAR

        PROGRAM main
            VAR CONSTANT
                mainConstRef : INT := mPx;
        END_PROGRAM

        "#,
    );

    debug_assert_eq!(unresolvable.len(), 3);
}

#[test]
fn test_struct_with_default_values() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Config :
            STRUCT
                id : INT := 1;
                value : REAL := 0.5;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            defaultCfg : Config;
            customCfg : Config := (id := 99, value := 3.14);
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);
    let act_my_struct = extract_struct_as_strings(find_constant_value(&index, "customCfg").unwrap());
    assert_eq!(act_my_struct, vec!["99", "3.14"]);
}

#[test]
fn test_single_struct_all_types_with_ranges_and_calculations() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyStruct :
            STRUCT
                i      : LINT (0..750) := 10;
                j      : LINT (-50..50) := -5;
                r      : REAL := 1.5;
                b      : BOOL := TRUE;
                s      : STRING := 'hello';
                calcI  : LINT := 5 + 5 * 2;        // 15
                calcR  : REAL := 2.5 * 2.0;       // 5.0
                calcB  : BOOL := (10 > 3);        // TRUE
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            varA : LINT := 20;
            varB : LINT := 54354;
            varC : LINT := 5430;
            varE : LINT := 5430;
            varF : LINT := 88776;
            varG : LINT := 45420;

            myStruct : MyStruct := (
                i := varA * 3 + varC / 10,      // 20*3 + 5430/10 = 60 + 543 = 603
                j := -(varA / 2),               // -(20/2) = -10
                r := 3.14 + varA / 10.0,        // 3.14 + 2.0 = 5.14
                b := (varB > varF),             // 54354 > 88776 = FALSE
                s := 'calc',
                calcI := (varB / varA) + varC,  // 54354/20 = 2717 + 5430 = 8147
                calcR := (varF / 1000.0) + 1.5, // 88.776 + 1.5 = 90.276
                calcB := (varG < varF)          // 45420 < 88776 = TRUE
            );

            myStruct_Error : MyStruct := (
                i := (varA * 3 + varC / 10 ) * 2,      // 20*3 + 5430/10 = 60 + 543 = 1206
                j := -(varA / 2),               // -(20/2) = -10
                r := 3.14 + varA / 10.0,        // 3.14 + 2.0 = 5.14
                b := (varB > varF),             // 54354 > 88776 = FALSE
                s := 'calc',
                calcI := (varB / varA) + varC,  // 54354/20 = 2717 + 5430 = 8147
                calcR := (varF / 1000.0) + 1.5, // 88.776 + 1.5 = 90.276
                calcB := (varG < varF)          // 45420 < 88776 = TRUE
            );
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("Value 1206 is out of SubRange 0..750"));

    let vals = extract_struct_as_strings(find_constant_value(&index, "myStruct").unwrap());

    assert_eq!(vals, vec!["603", "-10", "5.140000000000001", "false", "calc", "8147", "90.276", "true"]);
}

#[test]
fn test_single_struct_all_types_with_safe_calculations() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyStruct :
            STRUCT
                i      : INT (0..100) := 10;
                j      : INT (-50..50) := 5;
                r      : REAL := 1.5;
                b      : BOOL := TRUE;
                s      : STRING := 'test';
                calcI  : INT := 5 + 3 * 2;        // 11
                calcR  : REAL := 2.0 * 1.5;       // 3.0
                calcB  : BOOL := (5 > 2);         // TRUE
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            varA : INT := 10;
            varB : INT := 12;
            varC : INT := 3;
            varD : INT := 2;

            myStruct : MyStruct := (
                i := varA * varC + varD,       // 10*3 + 2 = 32
                j := -(varC),                  // -3
                r := 1.0 + varD,              // 3.0
                b := (varA > varB),           // FALSE
                s := 'ok',
                calcI := varA + varB,         // 22
                calcR := varC * 1.5,          // 4.5
                calcB := (varB > varC)        // TRUE
            );
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);

    let vals = extract_struct_as_strings(find_constant_value(&index, "myStruct").unwrap());

    assert_eq!(
        vals,
        vec![
            "32",    // i
            "-3",    // j
            "3",     // r
            "false", // b
            "ok",    // s
            "22",    // calcI
            "4.5",   // calcR
            "true"   // calcB
        ]
    );
}

#[test]
//TODO irgendwas geht da mit enums nicht...
fn test_multiple_structs_with_integer_subranges_and_enum_calculations() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyEnum :
            (A := 1, B := 2, C := 4);
        END_TYPE

        TYPE StructA :
            STRUCT
                i : INT (0..100) := 10;
                j : INT (-50..50) := -5;
                r : REAL := 1.5;
                b : BOOL := TRUE;
            END_STRUCT
        END_TYPE

        TYPE StructB :
            STRUCT
                name : STRING := 'default';
                e : MyEnum := MyEnum.A;
                calcInt : INT := 5 + 5 * 2;
            END_STRUCT
        END_TYPE

        TYPE StructC :
            STRUCT
                nestedA : StructA;
                nestedB : StructB;
                mixedReal : REAL := 2.5 * 4.0;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            a : StructA := (
                i := 42,
                j := -10,
                r := 3.14,
                b := FALSE
            );

            b : StructB := (
                name := 'test',
                e := MyEnum.B,
                calcInt := (10 + 10) / 2
            );

            c : StructC := (
                nestedA := (i := 7, j := 0, r := 2.71, b := TRUE),
                nestedB := (name := 'nested', e := MyEnum.C, calcInt := 3 * 3),
                mixedReal := 1.5 + 2.5,
            );
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);

    let a_vals = extract_struct_as_strings(find_constant_value(&index, "a").unwrap());
    assert_eq!(a_vals, vec!["42", "-10", "3.14", "false"]);

    let b_vals = extract_struct_as_strings(find_constant_value(&index, "b").unwrap());
    assert_eq!(b_vals, vec!["test", "2", "10"]);
}

#[test]
fn enum_alias_and_subrange_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyEnum : (A := 1, B := 7, C := 11); END_TYPE
        TYPE MyEnumAlias : MyEnum; END_TYPE
        TYPE SmallRange : INT(0..20); END_TYPE

        VAR_GLOBAL CONSTANT
            e0 : MyEnum := MyEnum.B;
            e1 : MyEnumAlias := MyEnum.C;
            i0 : INT := e0;
            r0 : SmallRange := e1;
            sum : INT := i0 + r0;
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);
    debug_assert_eq!(find_constant_value(&index, "e0"), Some(create_int_literal(7)));
    debug_assert_eq!(find_constant_value(&index, "e1"), Some(create_int_literal(11)));
    debug_assert_eq!(find_constant_value(&index, "i0"), Some(create_int_literal(7)));
    debug_assert_eq!(find_constant_value(&index, "r0"), Some(create_int_literal(11)));
    debug_assert_eq!(find_constant_value(&index, "sum"), Some(create_int_literal(18)));
}

#[test]
fn enum_boolean_expression_chain_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Mode : (INIT := 0, RUN := 2, STOP := 5); END_TYPE
        TYPE BitRange : INT(0..1); END_TYPE

        VAR_GLOBAL CONSTANT
            gt : BOOL := Mode.STOP > Mode.RUN;
            eq : BOOL := Mode.INIT = 0;
            logic : BOOL := gt AND NOT FALSE AND (Mode.RUN <> Mode.INIT);
            marker : BitRange := 1;
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);
    debug_assert_eq!(find_constant_value(&index, "gt"), Some(create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "eq"), Some(create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "logic"), Some(create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "marker"), Some(create_int_literal(1)));
}

#[test]
fn struct_with_nested_enum_initializer_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE State : (Idle := 0, Run := 4); END_TYPE
        TYPE LevelRange : INT(0..10); END_TYPE

        TYPE Inner : STRUCT
            level : LevelRange;
            state : State;
        END_STRUCT END_TYPE

        TYPE Outer : STRUCT
            inner : Inner;
            checksum : INT;
        END_STRUCT END_TYPE

        VAR_GLOBAL CONSTANT
            cfg : Outer := (inner := (level := 3, state := State.Run), checksum := INT#State.Run + 3);
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);
    assert!(find_constant_value(&index, "cfg").is_some());
}

#[test]
//TODO ??
fn struct_field_access_in_expression_is_rejected() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE CoordRange : INT(0..10); END_TYPE

        TYPE Point :
            STRUCT
                x : CoordRange;
                y : CoordRange;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            p : Point := (x := 2, y := 4);
            cR : CoordRange := 20;
            overFlowP : Point := (x := 20, y := 40);
            bad : INT := p.x + p.y;
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable.len(), 2);
    //debug_assert_eq!(unresolvable.len(), 3);
    assert!(unresolvable[0].get_reason().unwrap().contains("out of SubRange"));
    assert!(unresolvable[1].get_reason().unwrap().contains("out of SubRange"));
    //assert!(unresolvable[2].get_reason().unwrap().contains("Struct field access is not supported in a Constant!"));
}

#[test]
fn enum_value_used_in_struct_initializer_expression_resolves() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Level : (LOW := 1, HIGH := 9); END_TYPE
        TYPE LevelRange : INT(1..9); END_TYPE

        TYPE Boxed : STRUCT
            value : LevelRange;
            valid : BOOL;
        END_STRUCT END_TYPE

        VAR_GLOBAL CONSTANT
            b : Boxed := (value := INT#Level.HIGH, valid := Level.LOW < Level.HIGH);
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);
    assert!(find_constant_value(&index, "b").is_some());
}

#[test]
fn constant_in_case_resolved() {
    let (_, unresolvable) = eval_constants(
        r#"
        PROGRAM main
            VAR
                DAYS_IN_MONTH : DINT;
            END_VAR
            VAR CONSTANT
                SIXTY : DINT := 60;
            END_VAR
            CASE DAYS_IN_MONTH OF
              32..SIXTY :   DAYS_IN_MONTH := 29;
              (SIXTY    + 2)..70 :  DAYS_IN_MONTH := 30;
            ELSE
              DAYS_IN_MONTH := 31;
            END_CASE;
        END_PROGRAM
        "#,
    );

    debug_assert_eq!(unresolvable, EMPTY);

    // DONE in Codegen tests:
    // AND the first case should be 32..60
    // AND the second case should be 62..70
}

#[test]
fn const_references_int_bit_functions_behavior_evaluation() {
    // GIVEN some bit-functions used as initializers
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            _0x00ff : WORD := 16#00FF;
        END_VAR

        VAR_GLOBAL CONSTANT
            a : WORD := 16#FFAB;
            b : WORD := a AND _0x00ff;
            c : WORD := a OR _0x00ff;
            d : WORD := a XOR _0x00ff;
            e : WORD := NOT a;
        END_VAR
        ",
    );
    debug_assert_eq!(unresolvable.len(), 1);
    // AND the index should have literal values
    debug_assert_eq!(&create_int_literal(0xFFAB), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(0x00AB), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(0xFFFF), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_int_literal(0xFF54), find_constant_value(&index, "d").unwrap());

    debug_assert_eq!(unresolvable[0].get_reason().expect("REASON").contains("overflow"), true);
}
#[test]
fn illegal_cast_should_not_be_resolved() {
    // GIVEN some bit-functions used as initializers
    let (_index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := BOOL#16#00FF;
        END_VAR
       ",
    );
    assert!(unresolvable[0]
        .get_reason()
        .unwrap()
        .contains("Implicit conversion from INT to BOOL failed: Only 0 or 1 are allowed, but got 255"));
}

#[test]
fn division_by_0_should_fail() {
    // GIVEN some bit-functions used as initializers
    let (_index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            zero_int : INT := 0;
            zero_real : REAL := 0.0;

            a : REAL := 5 / zero_int; //invalid (div by 0)
            b : REAL := 5 / zero_real; //valid (inf)
            c : REAL := 5.0 / zero_int; //valid (inf)
            d : REAL := 5.0 / zero_real; //valid (inf)

            aa : REAL := 5 MOD zero_int;
            bb : REAL := 5 MOD zero_real;
            cc : REAL := 5.0 MOD zero_int;
            dd : REAL := 5.0 MOD zero_real;
        END_VAR
       ",
    );

    debug_assert_eq!(unresolvable.len(), 8);
    for i in 0..unresolvable.len() {
        let reason = unresolvable[i].get_reason().unwrap();

        if i < 4 {
            assert!(reason.contains("Attempt to divide by zero"),);
        } else {
            assert!(reason.contains("Attempt to calculate the remainder with a divisor of zero"));
        }
    }
}

#[test]
fn const_references_not_function_with_signed_ints() {
    // GIVEN some bit-functions used as initializers
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            _0x00ff : INT := 16#00FF;   // 255
        END_VAR

        VAR_GLOBAL CONSTANT
            a : INT := INT#16#55;       // 85;
            b : INT := a AND _0x00ff;   // 85
            c : INT := a OR _0x00ff;    // 255
            d : INT := a XOR _0x00ff;   // 170
            e : INT := NOT a;           // -86
        END_VAR
        ",
    );

    // THEN everything got resolved
    debug_assert_eq!(EMPTY, unresolvable);
    // AND the index should have literal values
    debug_assert_eq!(&create_int_literal(85), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(85), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(255), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_int_literal(170), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_int_literal(-86), find_constant_value(&index, "e").unwrap());
}

#[test]
fn const_references_to_bool_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            x : BOOL := TRUE;
            y : BOOL := FALSE;
            z : BOOL := y;
        END_VAR

        VAR_GLOBAL CONSTANT
            a : BOOL := x;
            b : BOOL := y OR NOT y;
            c : BOOL := z AND NOT z;
        END_VAR
        ",
    );

    // THEN a,b,and c got their correct initial-literals
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(&create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(&create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(&create_bool_literal(false)));
}

#[test]
fn nested_array_struct_combo_is_resolvable() {
    let (_index, unresolvable) = eval_constants(
        "TYPE Particle :
            STRUCT
                pos : ARRAY[0..1] OF REAL;
                velocity : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            physics_system : ARRAY[0..0] OF Particle := [
                (pos := [1.5, 2.5], velocity := 9.81)
            ];
        END_VAR",
    );

    assert_eq!(unresolvable, EMPTY);
}

#[test]
fn struct_with_arrays_is_resolvable() {
    let (_index, unresolvable) = eval_constants(
        "TYPE DataCluster :
            STRUCT
                id : INT;
                samples : ARRAY[0..2] OF INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            myCluster : DataCluster := (id := 1, samples := [10, 20, 30]);
            myClusterTWO : DataCluster := (x := 1, samples := [10, 20, 30, 30]); //no field x in struct
            defaultCluster : DataCluster; //Resolved
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0].get_reason().unwrap().contains("Unknown struct member `x`"));
}

#[test]
fn array_of_structs_is_resolvable() {
    let (_index, unresolvable) = eval_constants(
        "TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            xOne : INT := 10;
            yOne : INT := 20;
            points : ARRAY[0..1] OF Point := [(x := xOne, y := yOne), (x := 30, y := 40)];
            not_resolvable_points : ARRAY[0..1] OF Point := [(x := 10, y := not_declared_y), (x := 30, y := 40)];
        END_VAR",
    );

    assert_eq!(unresolvable.len(), 1);
    assert!(unresolvable[0]
        .get_reason()
        .unwrap()
        .contains("Cannot resolve constant reference `not_declared_y`"));
}

#[test]
fn test_cycle_array_size_dependency() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            MAX_ELEMENTS : INT := c_NestedSize;
            c_NestedSize : INT := 10;
        END_VAR
        TYPE
            T_Array : ARRAY[0..MAX_ELEMENTS] OF BYTE;
        END_TYPE",
    );
    assert!(unresolvable.is_empty());
    debug_assert_eq!(find_constant_value(&index, "MAX_ELEMENTS"), Some(&create_int_literal(10)));
    debug_assert_eq!(find_constant_value(&index, "c_NestedSize"), Some(&create_int_literal(10)));
}

#[test]
fn test_struct_acceess() {
    let (_index, unresolvable) = eval_constants(
        "TYPE Point :
            STRUCT
                x : INT(0..10);
                y : INT(10..20);
            END_STRUCT
        END_TYPE
        VAR_GLOBAL CONSTANT
            aVar : INT := 10;
            myPointOne : Point;
            myPointTwo : Point := (x := 10, y := 20);
            myPointThree : Point := myPointTwo
        END_VAR
    ",
    );
    assert!(unresolvable.is_empty());
}

#[test]
fn test_all_possible_struct_init_of_point() {
    let (_index, unresolvable) = eval_constants(
        "TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            myPoint : Point;
            myPoint1 : Point := (x := 10, y := 20);
            myPoint3 : Point := (x := 10);
            myPoint4 : Point := (y := 20);
            myPoint5 : Point := (x := 5 * 2);
        END_VAR",
    );
    assert_eq!(unresolvable, EMPTY);
}

#[test]
fn const_references_to_int_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE OneInt : INT := 1; END_TYPE
        VAR_GLOBAL CONSTANT
            counter : OneInt;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "counter"), Some(create_int_literal(1)));
}

#[test]
fn local_const_references_to_int_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        "
        PROGRAM prg
            VAR CONSTANT
                iX : INT := 4;
                rX : LREAL := 4.2;
           END_VAR
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            a : INT := prg.iX;
            b : LREAL := prg.rX;
       END_VAR
        ",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_real_literal(4.2), find_constant_value(&index, "b").unwrap());
}

#[test]
fn test_sqrt_in_const_array() {
    let (index, unresolvable) = eval_constants(
        "
        VAR_GLOBAL CONSTANT
            a : REAL := 16.0;
            iX : ARRAY[0..3] OF INT := [1, SQRT(SQRT(a)), 3, 4];
            index : INT := iX[1];
       END_VAR
        ",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let arr = create_array_literal(vec![
        create_int_literal(1),
        create_int_literal(2),
        create_int_literal(3),
        create_int_literal(4),
    ]);
    debug_assert_eq!(find_constant_value(&index, "iX").unwrap(), arr);
}

#[test]
fn local_const_references_to_int_compile_time_evaluation_uses_correct_scopes() {
    let (index, unresolvable) = eval_constants(
        "
        VAR_GLOBAL CONSTANT
            a : INT := 5;
        END_VAR

        VAR_GLOBAL
            g : INT := a; // should be 5
            h : INT := prg.a; // should be 4
        END_VAR

        PROGRAM prg
            VAR CONSTANT
                a : INT := 4;
            END_VAR

            VAR_INPUT
                v : INT := a; // should be 4
            END_VAR
        END_PROGRAM
        ",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(5), find_constant_value(&index, "g").unwrap());
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "h").unwrap());
    debug_assert_eq!(&create_int_literal(4), find_member_value(&index, "prg", "v").unwrap());
}

#[test]
fn prg_members_initials_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        "
        VAR_GLOBAL CONSTANT
            TWO : INT := 2;
            FIVE : INT := TWO * 2 + 1;
            C_STR : STRING := 'hello world';
        END_VAR

        PROGRAM plc_prg
            VAR_INPUT
                a : INT := TWO;
                b : INT := TWO + 4;
                c : INT := FIVE;
                str : STRING := C_STR;
            END_VAR
        END_PROGRAM
        ",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(2), find_member_value(&index, "plc_prg", "a").unwrap());
    debug_assert_eq!(&create_int_literal(6), find_member_value(&index, "plc_prg", "b").unwrap());
    debug_assert_eq!(&create_int_literal(5), find_member_value(&index, "plc_prg", "c").unwrap());
    debug_assert_eq!(
        &create_string_literal("hello world", false),
        find_member_value(&index, "plc_prg", "str").unwrap()
    );
}

#[test]
fn const_references_to_negative_reference() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 4;
            b : INT := -a;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(-4), find_constant_value(&index, "b").unwrap());
}

#[test]
fn test_new_evaluator() {
    let (_index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            A : INT := 5;
            B : INT := -5;
            C : INT := -(-5);
            D : REAL := -3.14;
            E : BOOL := TRUE;
            F : BOOL := NOT TRUE;
            G : BOOL := NOT NOT FALSE;
            H : INT := NOT 0;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
}

#[test]
fn const_references_to_int_additions_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            iY : INT := iX;
            iZ : INT := iY + 7;
        END_VAR",
    );
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(11), find_constant_value(&index, "iZ").unwrap());
}

#[test]
fn const_references_to_int_subtractions_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            iY : INT := iX;
            iZ : INT := iY - 7;
        END_VAR",
    );
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(-3), find_constant_value(&index, "iZ").unwrap());
}

#[test]
fn const_references_to_int_multiplications_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            iY : INT := iX;
            iZ : INT := iY * 7;
        END_VAR",
    );
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(28), find_constant_value(&index, "iZ").unwrap());
}

#[test]
fn const_references_to_int_division_compile_time_evaluation() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            iX : INT := 40;
            iY : INT := iX;
            iZ : INT := iY / 8;
        END_VAR",
    );
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(5), find_constant_value(&index, "iZ").unwrap());
}

#[test]
fn const_lreal_initializers_should_be_resolved_correctly() {
    let (index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : LREAL := 22.2;
            b : REAL := 22.2;
            c : LREAL := b;
        END_VAR",
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_real_literal(22.2), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_real_literal(22.2), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_real_literal(22.2), find_constant_value(&index, "c").unwrap());
}

#[test]
fn array_literals_type_resolving() {
    let (_index, unresolvable) = eval_constants(
        "VAR_GLOBAL CONSTANT
            a : INT := 1;
            b : ARRAY[0..2] OF INT := [a, 2 + 1, 4];
        END_VAR",
    );
    debug_assert_eq!(EMPTY, unresolvable);
}

#[test]
fn function_block_initializers_constant_resolved_in_assignment() {
    let (_index, unresolvable) = eval_constants(
        "FUNCTION_BLOCK TON
            VAR_OUTPUT
                a : INT;
                b : INT;
            END_VAR
         END_FUNCTION_BLOCK

         PROGRAM main
            VAR CONSTANT
                TEN : INT := 10;
            END_VAR
            VAR
                struct1 : TON := (a := 10, b := TEN + 7);
                struct2 : TON := (b := 10, a := TEN + 7);
            END_VAR
         END_PROGRAM",
    );

    debug_assert_eq!(EMPTY, unresolvable);
}

#[test]
fn ref_initializer_is_marked_as_resolve_later() {
    let (_index, unresolvable) = eval_constants(
        r#"
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
       "#,
    );

    assert!(!unresolvable.is_empty());
}

#[test]
fn adr_initializer_is_marked_as_resolve_later() {
    let (_index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL
            s : STRING;
        END_VAR

        PROGRAM foo
        VAR
            ps: REF_TO STRING := ADR(s);
        END_VAR
        END_PROGRAM
       "#,
    );

    assert!(!unresolvable.is_empty());
}

#[test]
fn alias_initializer_is_marked_as_resolve_later() {
    let (_index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL
            gs : STRING;
        END_VAR

        PROGRAM foo
        VAR
            s : STRING;
            ps1 AT s : STRING;
            ps2 AT gs : STRING;
        END_VAR
        END_PROGRAM
       "#,
    );

    assert!(!unresolvable.is_empty());
}

#[test]
fn reference_to_initializer_is_marked_as_resolve_later() {
    let (_index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL
            s : STRING;
        END_VAR

        PROGRAM foo
        VAR
            ps : REFERENCE TO STRING := REF(s);
        END_VAR
        END_PROGRAM
       "#,
    );

    assert!(!unresolvable.is_empty());
}

#[test]
fn test_struct_ids_with_reference() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyStruct:
             STRUCT
                a : INT;
                b : INT;
            END_STRUCT
        END_TYPE
        VAR_GLOBAL CONSTANT
            a : myStruct := (a := 10, b := 20);
            b : ARRAY[0..1] OF myStruct := [(10, 20)];
            c : myStruct := a;
            d : myStruct := b;
        END_VAR
       "#,
    );

    assert!(unresolvable.is_empty());
    for name in ["a", "c"] {
        let act_my_struct = extract_struct_as_strings(find_constant_value(&index, name).unwrap());

        debug_assert_eq!(act_my_struct[0], "10");
        debug_assert_eq!(act_my_struct[1], "20");
    }

    for name in ["b", "d"] {
        let act_my_struct = extract_struct_as_strings(find_constant_value(&index, name).unwrap());

        debug_assert_eq!(act_my_struct[0], "[10, 20]");
    }
}

#[test]
fn test_struct_inits_which_are_inconsistent() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyStruct:
             STRUCT
                a : INT;
                b : INT;
            END_STRUCT
        END_TYPE
        VAR_GLOBAL CONSTANT
            a : myStruct := (a := 10, b := 20);
            b : myStruct := (10,20);
            c : myStruct := a;
            d : myStruct := (a := 10, 20);
            e : myStruct := (10, b := 20);
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 0);
    for name in ["a", "b", "c", "d", "e"] {
        let act_my_struct = extract_struct_as_strings(find_constant_value(&index, name).unwrap());

        debug_assert_eq!(act_my_struct[0], "10");
        debug_assert_eq!(act_my_struct[1], "20");
    }
}

#[test]
fn reference_to_initializer_is_marked_as_resolve_later_two() {
    let (index, _unresolvable) = eval_constants(
        r#"
        TYPE MyEnum : (A := 1, B := 7, C := 11); END_TYPE
        VAR_GLOBAL CONSTANT
            myArr : ARRAY[0..5] OF INT := [1,a];
        END_VAR
       "#,
    );

    debug_assert_eq!(find_constant_value(&index, "A").unwrap(), create_int_literal(1));
    debug_assert_eq!(find_constant_value(&index, "B").unwrap(), create_int_literal(7));
    debug_assert_eq!(find_constant_value(&index, "C").unwrap(), create_int_literal(11));
}

#[test]
fn test_const_math_functions_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (A := 0, B := 16, C := 100);
        END_TYPE

        VAR_GLOBAL CONSTANT
            a : REAL := SQRT(4.0);
            b : REAL := SQRT(16.0);
            c : REAL := SQRT(64.0) / 2.0;

            d : REAL := SQRT(a);
            e : REAL := SQRT(b);
            f : REAL := SQRT(c) / 2.0;

            g : INT := SQRT(101.0);

            nested : REAL := SQRT(SQRT(16.0));

            err_neg : REAL := SQRT(-1.0);
            err_int : REAL := SQRT(100);
            err_too_many_arguments : REAL := SQRT(2.0, 3.0);
            err_string : REAL := SQRT("Hallo");
            err_bool : REAL := SQRT(true);
            err_enum : REAL := SQRT(MyEnum.B);
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 6);

    debug_assert_eq!(find_constant_value(&index, "a").unwrap(), create_real_literal(2.0));
    debug_assert_eq!(find_constant_value(&index, "b").unwrap(), create_real_literal(4.0));
    debug_assert_eq!(find_constant_value(&index, "c").unwrap(), create_real_literal(4.0));
    debug_assert_eq!(find_constant_value(&index, "d").unwrap(), create_real_literal(2.0.sqrt()));
    debug_assert_eq!(find_constant_value(&index, "e").unwrap(), create_real_literal(4.0.sqrt()));
    debug_assert_eq!(find_constant_value(&index, "f").unwrap(), create_real_literal(4.0.sqrt() / 2.0));
    debug_assert_eq!(find_constant_value(&index, "g").unwrap(), create_int_literal(101.sqrt()));
    debug_assert_eq!(find_constant_value(&index, "nested").unwrap(), create_real_literal(2.0));

    let errors_str = format!("{:?}", unresolvable);
    assert!(errors_str.contains("must not be negative"));
    assert!(errors_str.contains("expects a REAL value"));
}

#[test]
fn test_const_abs_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (A := 0, B := -10, C := 100);
        END_TYPE

        VAR_GLOBAL CONSTANT
            // REAL Tests
            a : REAL := ABS(-4.5);
            b : REAL := ABS(16.0);
            c : REAL := ABS(-64.0) / 2.0;

            // INT Tests
            d : INT := ABS(-10);
            e : INT := ABS(20);
            f : INT := ABS(-100) + 5;

            // Verschachtelt & Gemischt
            nested_r : REAL := ABS(ABS(-16.0));
            nested_i : INT := ABS(ABS(-50));

            // Sonderfälle
            zero_r : REAL := ABS(0.0);
            zero_i : INT := ABS(0);
            enum : REAL := ABS(MyEnum.B);

            // Fehlerfälle (sollten in 'unresolvable' landen)
            err_too_many : REAL := ABS(-1.0, 2.0);
            err_string   : REAL := ABS("Negativ");
            err_bool     : REAL := ABS(false);
            err_empty    : REAL := ABS();
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 4);

    // REAL Assertions
    debug_assert_eq!(find_constant_value(&index, "a").unwrap(), create_real_literal(4.5));
    debug_assert_eq!(find_constant_value(&index, "b").unwrap(), create_real_literal(16.0));
    debug_assert_eq!(find_constant_value(&index, "c").unwrap(), create_real_literal(32.0));
    debug_assert_eq!(find_constant_value(&index, "zero_r").unwrap(), create_real_literal(0.0));
    debug_assert_eq!(find_constant_value(&index, "nested_r").unwrap(), create_real_literal(16.0));
    debug_assert_eq!(find_constant_value(&index, "enum").unwrap(), create_real_literal(10.0));

    // INT Assertions
    debug_assert_eq!(find_constant_value(&index, "d").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "e").unwrap(), create_int_literal(20));
    debug_assert_eq!(find_constant_value(&index, "f").unwrap(), create_int_literal(105));
    debug_assert_eq!(find_constant_value(&index, "zero_i").unwrap(), create_int_literal(0));
    debug_assert_eq!(find_constant_value(&index, "nested_i").unwrap(), create_int_literal(50));

    // Fehlerprüfung
    let errors_str = format!("{:?}", unresolvable);
    assert!(
        errors_str.contains("ABS expects a REAL or INT value")
            || errors_str.contains("expects exactly 1 argument")
    );
}
#[test]
fn test_const_mul_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (A := 1, B := 5, C := 10);
        END_TYPE

        VAR_GLOBAL CONSTANT
            mul_simple : INT := MUL(10, 5);
            mul_neg : INT := MUL(10, -2);
            mul_nested : INT := MUL(MUL(2, 2), MUL(3, 3));
            mul_real : REAL := MUL(1.5, 2.0);
            mul_mixed : INT := MUL(10, 2.5);
            mul_one_arg : INT := MUL(42);
            mul_zero : INT := MUL(10, 0, 5);

            err_mul_bool : INT := MUL(10, TRUE);
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 1);

    debug_assert_eq!(find_constant_value(&index, "mul_simple").unwrap(), create_int_literal(50));
    debug_assert_eq!(find_constant_value(&index, "mul_neg").unwrap(), create_int_literal(-20));
    debug_assert_eq!(find_constant_value(&index, "mul_nested").unwrap(), create_int_literal(36));
    debug_assert_eq!(find_constant_value(&index, "mul_one_arg").unwrap(), create_int_literal(42));
    debug_assert_eq!(find_constant_value(&index, "mul_real").unwrap(), create_real_literal(3.0));
    debug_assert_eq!(find_constant_value(&index, "mul_mixed").unwrap(), create_int_literal(25));
    debug_assert_eq!(find_constant_value(&index, "mul_zero").unwrap(), create_int_literal(0));
}

#[test]
fn test_const_sub_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            sub_simple : INT := SUB(100, 20);
            sub_neg : INT := SUB(10, 20);
            sub_real : REAL := SUB(5.5, 1.5);
            sub_mixed : REAL := SUB(10.0, 2);

            err_sub_args : INT := SUB(10, 20, 30);
            err_sub_type : INT := SUB(10, "X");
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 2);

    debug_assert_eq!(find_constant_value(&index, "sub_simple").unwrap(), create_int_literal(80));
    debug_assert_eq!(find_constant_value(&index, "sub_neg").unwrap(), create_int_literal(-10));
    debug_assert_eq!(find_constant_value(&index, "sub_real").unwrap(), create_real_literal(4.0));
    debug_assert_eq!(find_constant_value(&index, "sub_mixed").unwrap(), create_real_literal(8.0));
}
#[test]
fn test_const_div_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            div_simple : INT := DIV(100, 10);
            div_trunc : INT := DIV(7, 2);
            div_real : REAL := DIV(7.0, 2.0);
            div_mixed : REAL := DIV(10, 4.0);

            err_div_zero : INT := DIV(10, 0);
            err_div_args : INT := DIV(10);
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 2);

    debug_assert_eq!(find_constant_value(&index, "div_simple").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "div_trunc").unwrap(), create_int_literal(3));
    debug_assert_eq!(find_constant_value(&index, "div_real").unwrap(), create_real_literal(3.5));
    debug_assert_eq!(find_constant_value(&index, "div_mixed").unwrap(), create_real_literal(2.5));
}

#[test]
fn test_const_add_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (A := 0, B := -10, C := 100);

            MyStruct : STRUCT
                val1 : INT;
                val2 : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            add_simple : INT := ADD(10, 20);
            add_neg : DINT := ADD(100, -50);
            add_nested : INT := ADD(ADD(1, 2), ADD(3, 4));
            add_real : REAL := ADD(1.5, 2.5);
            add_huge : LINT := ADD(634634564356, 27);
            add_mixed : INT := ADD(10334, 2.5345);
            add_one_arg : INT := ADD(10);


            struct_positional : MyStruct := (val1 := ADD(10, MyEnum.C),val2 := ADD(1.0, 1.5));

            // 2. Named Assignment: MyStruct(field := value)
            struct_named : MyStruct := (val1 := ADD(MyEnum.B, 20), val2 := ADD(5.0, 5.0));

            // --- FEHLERFÄLLE ---
            err_add_bool : INT := ADD(10, TRUE);
            err_add_string : INT := ADD(10, "Fehler");
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 2);

    debug_assert_eq!(find_constant_value(&index, "add_simple").unwrap(), create_int_literal(30));
    debug_assert_eq!(find_constant_value(&index, "add_neg").unwrap(), create_int_literal(50));
    debug_assert_eq!(find_constant_value(&index, "add_nested").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "add_one_arg").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "add_real").unwrap(), create_real_literal(4.0));
    debug_assert_eq!(find_constant_value(&index, "add_huge").unwrap(), create_int_literal(634634564356 + 27));
    debug_assert_eq!(
        find_constant_value(&index, "add_mixed").unwrap(),
        create_int_literal((10334.0 + 2.5345) as i128)
    );
}

#[test]
fn test_const_min_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (A := 0, B := -10, C := 100);

            MyStruct : STRUCT
                val1 : INT;
                val2 : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            min_simple : INT := MIN(50, 10, 30);      // 10
            min_neg : DINT := MIN(-100, -50, 0);      // -100
            min_nested : INT := MIN(MIN(5, 10), MIN(1, 20)); // 1
            min_real : REAL := MIN(10.5, 2.5, 4.2);   // 2.5
            min_huge : LINT := MIN(170141183460469231731687303715884105727, 10); // 10
            min_mixed : REAL := MIN(100, 2.5);        // 2.5 (Casting auf REAL)
            min_one_arg : INT := MIN(42);             // 42
            struct_named : MyStruct := (val1 := MIN(MyEnum.B, -5), val2 := MIN(1.0, 9.9));

            err_min_bool : INT := MIN(10, TRUE);
            err_min_string : INT := MIN(10, "Fehler");
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 2);

    debug_assert_eq!(find_constant_value(&index, "min_simple").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "min_neg").unwrap(), create_int_literal(-100));
    debug_assert_eq!(find_constant_value(&index, "min_nested").unwrap(), create_int_literal(1));
    debug_assert_eq!(find_constant_value(&index, "min_one_arg").unwrap(), create_int_literal(42));
    debug_assert_eq!(find_constant_value(&index, "min_real").unwrap(), create_real_literal(2.5));
    debug_assert_eq!(find_constant_value(&index, "min_huge").unwrap(), create_int_literal(10));
    debug_assert_eq!(find_constant_value(&index, "min_mixed").unwrap(), create_real_literal(2.5));
}

#[test]
fn test_const_max_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (A := 0, B := -10, C := 100);

            MyStruct : STRUCT
                val1 : INT;
                val2 : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            max_simple : INT := MAX(50, 10, 30);      // 50
            max_neg : DINT := MAX(-100, -50, 0);      // 0
            max_nested : INT := MAX(MAX(5, 10), MAX(1, 20)); // 20
            max_real : REAL := MAX(10.5, 2.5, 4.2);   // 10.5
            max_huge : LINT := MAX(634634564356, 1);  // 634634564356
            max_mixed : REAL := MAX(1, 2.5);          // 2.5
            max_one_arg : INT := MAX(42);             // 42

            struct_named : MyStruct := (val1 := MAX(MyEnum.B, 20), val2 := MAX(5.0, 5.0));

            err_max_bool : INT := MAX(10, TRUE);
            err_max_string : INT := MAX(10, "Fehler");
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 2);

    // Checks Basis
    debug_assert_eq!(find_constant_value(&index, "max_simple").unwrap(), create_int_literal(50));
    debug_assert_eq!(find_constant_value(&index, "max_neg").unwrap(), create_int_literal(0));
    debug_assert_eq!(find_constant_value(&index, "max_nested").unwrap(), create_int_literal(20));
    debug_assert_eq!(find_constant_value(&index, "max_one_arg").unwrap(), create_int_literal(42));
    debug_assert_eq!(find_constant_value(&index, "max_real").unwrap(), create_real_literal(10.5));
    debug_assert_eq!(find_constant_value(&index, "max_huge").unwrap(), create_int_literal(634634564356));
    debug_assert_eq!(find_constant_value(&index, "max_mixed").unwrap(), create_real_literal(2.5));
}

#[test]
fn test_const_limit_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (LOW := 10, MID := 50, HIGH := 100);

            MyStruct : STRUCT
                val1 : INT;
                val2 : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            limit_in_range : INT := LIMIT(0, 50, 100);    // 50
            limit_below    : INT := LIMIT(0, -10, 100);   // 0
            limit_above    : INT := LIMIT(0, 150, 100);   // 100

            limit_real     : REAL := LIMIT(0.5, 0.7, 1.0); // 0.7
            limit_mixed    : REAL := LIMIT(10, 2.5, 100);  // 10.0 (unten begrenzt)

            limit_huge     : LINT := LIMIT(634634564300, 634634564356, 634634564400); // 634634564356

            err_limit_args   : INT := LIMIT(1, 2);          // Zu wenige
            err_limit_bool   : INT := LIMIT(0, TRUE, 100);  // Falscher Typ
            err_limit_string : INT := LIMIT("A", "B", "C"); // Komplett falsch
        END_VAR
       "#,
    );

    // Erwartete Fehler: Zu wenige Argumente, Bool-Input, String-Input
    assert_eq!(unresolvable.len(), 3);

    // Checks Basis INT
    debug_assert_eq!(find_constant_value(&index, "limit_in_range").unwrap(), create_int_literal(50));
    debug_assert_eq!(find_constant_value(&index, "limit_below").unwrap(), create_int_literal(0));
    debug_assert_eq!(find_constant_value(&index, "limit_above").unwrap(), create_int_literal(100));

    // Checks REAL / Mixed
    debug_assert_eq!(find_constant_value(&index, "limit_real").unwrap(), create_real_literal(0.7));
    debug_assert_eq!(find_constant_value(&index, "limit_mixed").unwrap(), create_real_literal(10.0));

    // Check Huge i128
    debug_assert_eq!(find_constant_value(&index, "limit_huge").unwrap(), create_int_literal(634634564356));
}

#[test]
fn test_structs_cast_float_correct_to_int() {
    let (index, unresolvable) = eval_constants(
        r#"
    TYPE Point :
        STRUCT
            x : INT;
            y : INT;
        END_STRUCT
    END_TYPE
    TYPE Point_TWO :
        STRUCT
            x : INT;
            y : INT;
        END_STRUCT
    END_TYPE
    PROGRAM Main
        VAR CONSTANT
            a : ARRAY[0..2] OF INT := [1,2,3];
            c : ARRAY[0..2] OF STRING := a;
            d : Point := (x := 20.2342, y := 20);
            e : Point_TWO := d;
            f : Point;
            g : Point := f;
        END_VAR
    END_PROGRAM
    "#,
    );

    assert_eq!(unresolvable.len(), 1);
    // the float was castet to int
    let act_my_struct = extract_struct_as_strings(find_member_value(&index, "Main", "d").unwrap());
    debug_assert_eq!(act_my_struct[0], "20");
    debug_assert_eq!(act_my_struct[1], "20");
}

#[test]
fn test_const_gt_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (LOW := 10, MID := 50, HIGH := 100);
        END_TYPE

        VAR_GLOBAL CONSTANT
            gt_int_true    : BOOL := GT(100, 50);    // TRUE
            gt_int_false   : BOOL := GT(50, 100);    // FALSE
            gt_int_equal   : BOOL := GT(50, 50);     // FALSE (da nicht >=)

            gt_real_true   : BOOL := GT(1.5, 0.7);   // TRUE
            gt_real_false  : BOOL := GT(0.5, 0.7);   // FALSE

            gt_mixed_true  : BOOL := GT(10, 2.5);    // TRUE (10.0 > 2.5)
            gt_mixed_false : BOOL := GT(2, 5.5);     // FALSE (2.0 > 5.5)

            gt_huge        : BOOL := GT(634634564400, 634634564300); // TRUE

            gt_enum        : BOOL := GT(MyEnum.HIGH, MyEnum.LOW);    // TRUE (100 > 10)

            // Fehlerfälle
            err_gt_args    : BOOL := GT(1);          // Zu wenige
            err_gt_bool    : BOOL := GT(TRUE, FALSE); // Falscher Typ (kein Real/Int)
            err_gt_string  : BOOL := GT("B", "A");    // Komplett falsch
        END_VAR
       "#,
    );

    // Erwartete Fehler: 1 Argument, Bool-Vergleich, String-Vergleich
    assert_eq!(unresolvable.len(), 3);

    // Checks Basis INT
    debug_assert_eq!(find_constant_value(&index, "gt_int_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "gt_int_false").unwrap(), create_bool_literal(false));
    debug_assert_eq!(find_constant_value(&index, "gt_int_equal").unwrap(), create_bool_literal(false));

    // Checks REAL
    debug_assert_eq!(find_constant_value(&index, "gt_real_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "gt_real_false").unwrap(), create_bool_literal(false));

    // Checks Mixed
    debug_assert_eq!(find_constant_value(&index, "gt_mixed_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "gt_mixed_false").unwrap(), create_bool_literal(false));

    // Check Huge
    debug_assert_eq!(find_constant_value(&index, "gt_huge").unwrap(), create_bool_literal(true));

    // Check Enum
    debug_assert_eq!(find_constant_value(&index, "gt_enum").unwrap(), create_bool_literal(true));
}

#[test]
fn test_const_ge_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (LOW := 10, MID := 50, HIGH := 100);
        END_TYPE

        VAR_GLOBAL CONSTANT
            ge_int_true    : BOOL := GE(100, 50);
            ge_int_true_eq : BOOL := GE(50, 50);
            ge_int_false   : BOOL := GE(10, 50);

            ge_real_true   : BOOL := GE(1.5, 0.7);
            ge_real_false  : BOOL := GE(0.5, 0.7);

            ge_mixed_true  : BOOL := GE(10, 2.5);
            ge_mixed_false : BOOL := GE(2, 5.5);

            ge_enum        : BOOL := GE(MyEnum.HIGH, MyEnum.HIGH);

            err_ge_args    : BOOL := GE(1);
            err_ge_bool    : BOOL := GE(TRUE, FALSE);
            err_ge_string  : BOOL := GE("B", "A");
        END_VAR
    "#,
    );

    assert_eq!(unresolvable.len(), 3);

    debug_assert_eq!(find_constant_value(&index, "ge_int_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "ge_int_true_eq").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "ge_int_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "ge_real_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "ge_real_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "ge_mixed_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "ge_mixed_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "ge_enum").unwrap(), create_bool_literal(true));
}

#[test]
fn test_const_lt_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (LOW := 10, MID := 50, HIGH := 100);
        END_TYPE

        VAR_GLOBAL CONSTANT
            lt_int_true    : BOOL := LT(10, 50);
            lt_int_false   : BOOL := LT(50, 10);
            lt_int_equal   : BOOL := LT(50, 50);

            lt_real_true   : BOOL := LT(0.5, 0.7);
            lt_real_false  : BOOL := LT(1.5, 0.7);

            lt_mixed_true  : BOOL := LT(2, 5.5);
            lt_mixed_false : BOOL := LT(10, 2.5);

            lt_enum        : BOOL := LT(MyEnum.LOW, MyEnum.HIGH);

            err_lt_args    : BOOL := LT(1);
            err_lt_bool    : BOOL := LT(TRUE, FALSE);
            err_lt_string  : BOOL := LT("A", "B");
        END_VAR
    "#,
    );

    assert_eq!(unresolvable.len(), 3);

    debug_assert_eq!(find_constant_value(&index, "lt_int_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "lt_int_false").unwrap(), create_bool_literal(false));
    debug_assert_eq!(find_constant_value(&index, "lt_int_equal").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "lt_real_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "lt_real_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "lt_mixed_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "lt_mixed_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "lt_enum").unwrap(), create_bool_literal(true));
}

#[test]
fn test_const_le_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (LOW := 10, MID := 50, HIGH := 100);
        END_TYPE

        VAR_GLOBAL CONSTANT
            le_int_true    : BOOL := LE(10, 50);
            le_int_true_eq : BOOL := LE(50, 50);
            le_int_false   : BOOL := LE(100, 50);

            le_real_true   : BOOL := LE(0.5, 0.7);
            le_real_false  : BOOL := LE(1.5, 0.7);

            le_mixed_true  : BOOL := LE(2, 5.5);
            le_mixed_false : BOOL := LE(10, 2.5);

            le_enum        : BOOL := LE(MyEnum.LOW, MyEnum.HIGH);

            // Fehlerfälle
            err_le_args    : BOOL := LE(1);
            err_le_bool    : BOOL := LE(TRUE, FALSE);
            err_le_string  : BOOL := LE("A", "B");
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 3);

    debug_assert_eq!(find_constant_value(&index, "le_int_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "le_int_true_eq").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "le_int_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "le_real_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "le_real_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "le_mixed_true").unwrap(), create_bool_literal(true));
}

#[test]
#[ignore] // Run with: cargo test -- --ignored perf_test_complex_struct_constants -- --nocapture
fn perf_test_complex_struct_constants() {
    use std::time::Instant;

    const NUM_RUNS: usize = 10;
    const NUM_CONSTANTS: usize = 10000; // Kannst du je nach Bedarf hochschrauben

    // Generiere 100 standardisierte INT-Einträge für das Initialisierungs-Array von a0
    let array_initializers = vec!["0"; 100].join(", ");

    // Basis-Typdefinitionen und globale Variablen vorbereiten
    let mut const_declarations = format!(
        r#"
        TYPE InnerConfig :
            STRUCT
                id : INT;
                valid : BOOL;
            END_STRUCT
        END_TYPE

        TYPE ComplexDataPoint :
            STRUCT
                config : InnerConfig;
                values : ARRAY[0..99] OF INT;
                active : BOOL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            a0 : ComplexDataPoint := (
                config := (id := 42, valid := TRUE), 
                values := [{}], 
                active := TRUE
            );
        "#,
        array_initializers
    );

    // Die Kette aufbauen: a1 := a0, a2 := a1, usw.
    for i in 1..NUM_CONSTANTS {
        const_declarations.push_str(&format!("    a{} : ComplexDataPoint := a{};\n", i, i - 1));
    }
    const_declarations.push_str("END_VAR\n");

    let mut times_ms = Vec::with_capacity(NUM_RUNS);
    let mut max_time = std::time::Duration::ZERO;
    let mut min_time = std::time::Duration::MAX;

    println!("\n========== STRUCT CONSTANT CHAIN TEST ({} Constants) ==========", NUM_CONSTANTS);
    println!("Type: DataPoint with 3 fields (value:INT, index:INT, active:BOOL)");
    println!("Chain: a0 := (init), a1 := a0, a2 := a1, ... a{} := a{}", NUM_CONSTANTS - 1, NUM_CONSTANTS - 2);
    println!("\nRunning {} iterations...\n", NUM_RUNS);

    for run in 1..=NUM_RUNS {
        let start = Instant::now();
        let (index, unresolvable) = eval_constants(&const_declarations);
        let duration = start.elapsed();

        // Verify no evaluation errors
        assert!(unresolvable.is_empty(), "Some constants failed to evaluate: {:?}", unresolvable);

        // Verify some values on first run only
        if run == 1 {
            assert!(find_constant_value(&index, "a0").is_some(), "a0 not found");
            assert!(find_constant_value(&index, "a1").is_some(), "a1 not found");
            assert!(find_constant_value(&index, "a100").is_some(), "a100 not found");
            assert!(find_constant_value(&index, "a499").is_some(), "a499 not found");
        }

        let ms = duration.as_secs_f64() * 1000.0;
        times_ms.push(ms);

        if duration > max_time {
            max_time = duration;
        }
        if duration < min_time {
            min_time = duration;
        }

        print!("Run {}: {:.2}ms  ", run, ms);
        if run % 5 == 0 {
            println!();
        }
    }
    println!("\n");

    // Calculate statistics
    let sum: f64 = times_ms.iter().sum();
    let mean = sum / times_ms.len() as f64;

    let variance = times_ms.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / times_ms.len() as f64;
    let std_dev = variance.sqrt();

    let sorted_times = {
        let mut sorted = times_ms.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
    };

    let median = if sorted_times.len() % 2 == 0 {
        (sorted_times[sorted_times.len() / 2 - 1] + sorted_times[sorted_times.len() / 2]) / 2.0
    } else {
        sorted_times[sorted_times.len() / 2]
    };

    // Print statistics
    println!("========== STATISTICS ==========");
    println!("Runs: {}", NUM_RUNS);
    println!("Mean: {:.2}ms", mean);
    println!("Median: {:.2}ms", median);
    println!("Std Dev: {:.2}ms", std_dev);
    println!("Min: {:.2}ms", min_time.as_secs_f64() * 1000.0);
    println!("Max: {:.2}ms", max_time.as_secs_f64() * 1000.0);
    println!("================================\n");

    // ASCII Bar Chart
    println!("========== HISTOGRAM (5ms bins) ==========");
    let max_ms = (max_time.as_secs_f64() * 1000.0).ceil() as u32;
    let min_ms = (min_time.as_secs_f64() * 1000.0).floor() as u32;
    let bin_size = 5;
    let num_bins = ((max_ms - min_ms) / bin_size + 1) as usize;
    let mut bins = vec![0; num_bins];

    for &time in &times_ms {
        let bin_idx = ((time as u32 - min_ms) / bin_size) as usize;
        if bin_idx < bins.len() {
            bins[bin_idx] += 1;
        }
    }

    let max_bin_count = *bins.iter().max().unwrap_or(&1);
    for (i, &count) in bins.iter().enumerate() {
        let bin_start = min_ms + (i as u32 * bin_size);
        let bin_end = bin_start + bin_size;
        let bar_width =
            if max_bin_count > 0 { (count as f64 / max_bin_count as f64 * 40.0) as usize } else { 0 };
        println!("[{:3}ms-{:3}ms) │ {} ({})", bin_start, bin_end, "█".repeat(bar_width), count);
    }
    println!("=========================================\n");

    // CSV output for external graphing
    println!("========== CSV DATA (for graphing) ==========");
    println!("run,time_ms");
    for (i, &time) in times_ms.iter().enumerate() {
        println!("{},{:.2}", i + 1, time);
    }
    println!("===========================================\n");

    // Summary line
    println!("📊 Performance Summary:");
    println!("   Cache efficiency test: {} runs of {} struct constants", NUM_RUNS, NUM_CONSTANTS);
    println!("   Average: {:.2}ms ± {:.2}ms (Mean ± StdDev)", mean, std_dev);
    println!("   Range: {:.2}ms - {:.2}ms", min_time.as_secs_f64() * 1000.0, max_time.as_secs_f64() * 1000.0);

    // Coefficient of variation
    let cv = (std_dev / mean) * 100.0;
    println!("   Coefficient of Variation: {:.1}%", cv);

    if cv < 5.0 {
        println!("   ✓ Excellent stability (low variance)");
    } else if cv < 10.0 {
        println!("   ~ Good stability");
    } else {
        println!("   ⚠ High variance detected");
    }
    println!();

    // Assert reasonable performance
    assert!(
        mean < 2000.0,
        "Performance degradation: Average time {:.2}ms is too high. Expected < 2000ms",
        mean
    );
}

#[test]
#[ignore] // Ignore by default - run with: cargo test -- --ignored perf_test_int_chain_dependencies -- --nocapture
fn perf_test_int_chain_dependencies() {
    use std::time::Instant;

    const NUM_RUNS: usize = 100;
    const NUM_CONSTANTS: usize = 45;

    // Generate 500 integer constants with multiple dependencies
    // a0 = 1
    // a1 = 1 + a0
    // a2 = a0 + a1 + a2
    // a3 = a0 + a1 + a2 + a3
    // ...
    let mut const_declarations = String::from("VAR_GLOBAL CONSTANT\n");
    const_declarations.push_str("    a0 : LINT := 1;\n");
    const_declarations.push_str("    a1 : LINT := 1 + a0;\n");

    for i in 2..NUM_CONSTANTS {
        // Each constant a_i depends on all previous constants a_0 to a_{i-1}
        let mut expr = String::from("a0");
        for j in 1..=i {
            if j < i {
                expr.push_str(&format!(" + a{}", j));
            }
        }
        const_declarations.push_str(&format!("    a{} : LINT := {} + a{};\n", i, expr, i - 1));
    }
    const_declarations.push_str("END_VAR\n");

    let mut times_ms = Vec::with_capacity(NUM_RUNS);
    let mut max_time = std::time::Duration::ZERO;
    let mut min_time = std::time::Duration::MAX;

    println!("\n========== INTEGER CHAIN DEPENDENCY TEST ({} Constants) ==========", NUM_CONSTANTS);
    println!("Type: INT with accumulating dependencies");
    println!("Pattern:");
    println!("  a0 := 1");
    println!("  a1 := 1 + a0");
    println!("  a2 := a0 + a1 + a1");
    println!("  a3 := a0 + a1 + a2 + a2");
    println!("  ... (each constant sums all previous + itself)");
    println!("\nRunning {} iterations...\n", NUM_RUNS);

    for run in 1..=NUM_RUNS {
        let start = Instant::now();
        let (index, unresolvable) = eval_constants(&const_declarations);
        let duration = start.elapsed();

        // Verify no evaluation errors
        assert!(
            unresolvable.is_empty(),
            "Run {}: Some constants failed to evaluate: {:?}",
            run,
            unresolvable
        );

        // Verify some values on first run only
        if run == 1 {
            assert!(find_constant_value(&index, "a0").is_some(), "a0 not found");
            assert!(find_constant_value(&index, "a1").is_some(), "a1 not found");
            assert!(find_constant_value(&index, "a10").is_some(), "a10 not found");
        }

        let ms = duration.as_secs_f64() * 1000.0;
        times_ms.push(ms);

        if duration > max_time {
            max_time = duration;
        }
        if duration < min_time {
            min_time = duration;
        }

        print!("Run {}: {:.2}ms  ", run, ms);
        if run % 5 == 0 {
            println!();
        }
    }
    println!("\n");

    // Calculate statistics
    let sum: f64 = times_ms.iter().sum();
    let mean = sum / times_ms.len() as f64;

    let variance = times_ms.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / times_ms.len() as f64;
    let std_dev = variance.sqrt();

    let sorted_times = {
        let mut sorted = times_ms.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
    };

    let median = if sorted_times.len() % 2 == 0 {
        (sorted_times[sorted_times.len() / 2 - 1] + sorted_times[sorted_times.len() / 2]) / 2.0
    } else {
        sorted_times[sorted_times.len() / 2]
    };

    // Print statistics
    println!("========== STATISTICS ==========");
    println!("Runs: {}", NUM_RUNS);
    println!("Mean: {:.2}ms", mean);
    println!("Median: {:.2}ms", median);
    println!("Std Dev: {:.2}ms", std_dev);
    println!("Min: {:.2}ms", min_time.as_secs_f64() * 1000.0);
    println!("Max: {:.2}ms", max_time.as_secs_f64() * 1000.0);
    println!("================================\n");

    // ASCII Bar Chart
    println!("========== HISTOGRAM (10ms bins) ==========");
    let max_ms = (max_time.as_secs_f64() * 1000.0).ceil() as u32;
    let min_ms = (min_time.as_secs_f64() * 1000.0).floor() as u32;
    let bin_size = 10;
    let num_bins = ((max_ms - min_ms) / bin_size + 1) as usize;
    let mut bins = vec![0; num_bins];

    for &time in &times_ms {
        let bin_idx = ((time as u32 - min_ms) / bin_size) as usize;
        if bin_idx < bins.len() {
            bins[bin_idx] += 1;
        }
    }

    let max_bin_count = *bins.iter().max().unwrap_or(&1);
    for (i, &count) in bins.iter().enumerate() {
        let bin_start = min_ms + (i as u32 * bin_size);
        let bin_end = bin_start + bin_size;
        let bar_width =
            if max_bin_count > 0 { (count as f64 / max_bin_count as f64 * 40.0) as usize } else { 0 };
        println!("[{:4}ms-{:4}ms) │ {} ({})", bin_start, bin_end, "█".repeat(bar_width), count);
    }
    println!("==========================================\n");

    // CSV output for external graphing
    println!("========== CSV DATA (for graphing) ==========");
    println!("run,time_ms");
    for (i, &time) in times_ms.iter().enumerate() {
        println!("{},{:.2}", i + 1, time);
    }
    println!("===========================================\n");

    // Summary line
    println!("📊 Performance Summary:");
    println!("   Complex dependency test: {} runs of {} integer constants", NUM_RUNS, NUM_CONSTANTS);
    println!("   Average: {:.2}ms ± {:.2}ms (Mean ± StdDev)", mean, std_dev);
    println!("   Range: {:.2}ms - {:.2}ms", min_time.as_secs_f64() * 1000.0, max_time.as_secs_f64() * 1000.0);

    // Coefficient of variation
    let cv = (std_dev / mean) * 100.0;
    println!("   Coefficient of Variation: {:.1}%", cv);

    if cv < 5.0 {
        println!("   ✓ Excellent stability (low variance)");
    } else if cv < 10.0 {
        println!("   ~ Good stability");
    } else {
        println!("   ⚠ High variance detected");
    }
    println!();

    // Assert reasonable performance
    assert!(
        mean < 5000.0,
        "Performance degradation: Average time {:.2}ms is too high. Expected < 5000ms",
        mean
    );
}

#[test]
fn test_const_ne_function_comprehensive() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE
            MyEnum : (LOW := 10, MID := 50, HIGH := 100);
        END_TYPE

        VAR_GLOBAL CONSTANT
            ne_int_true    : BOOL := NE(10, 50);
            ne_int_false   : BOOL := NE(50, 50);

            ne_real_true   : BOOL := NE(1.5, 0.7);
            ne_real_false  : BOOL := NE(1.5, 1.5);

            ne_mixed_true  : BOOL := NE(2, 5.5);
            ne_mixed_false : BOOL := NE(10, 10.0);

            ne_enum        : BOOL := NE(MyEnum.LOW, MyEnum.HIGH);

            // Fehlerfälle
            err_ne_args    : BOOL := NE(1);
            err_ne_bool    : BOOL := NE(TRUE, FALSE);
            err_ne_string  : BOOL := NE("A", "B");
        END_VAR
       "#,
    );

    assert_eq!(unresolvable.len(), 3);

    debug_assert_eq!(find_constant_value(&index, "ne_int_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "ne_int_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "ne_real_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "ne_real_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "ne_mixed_true").unwrap(), create_bool_literal(true));
    debug_assert_eq!(find_constant_value(&index, "ne_mixed_false").unwrap(), create_bool_literal(false));

    debug_assert_eq!(find_constant_value(&index, "ne_enum").unwrap(), create_bool_literal(true));
}

#[test]
fn test_struct_single_assignment_in_array() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE Particle :
            STRUCT
                pos : ARRAY[0..1] OF REAL;
                velocity : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            // Das hier hat vorher geknallt ("Cannot convert STRUCT to Particle")
            // weil die Zuweisung (pos := ...) einzeln im Array-Element liegt.
            physics_system : ARRAY[0..0] OF Particle := [
                (pos := [1.5, 2.5], velocity := 9.81)
            ];
        END_VAR
        "#,
    );

    assert!(
        unresolvable.is_empty(),
        "Should resolve complex struct assignment without errors, but got: {:?}",
        unresolvable.iter().map(|u| u.get_reason()).collect::<Vec<_>>()
    );
}

#[test]
fn test_struct_named_assignment_complete() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE Point :
            STRUCT
                x : REAL;
                y : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            // Testet normales Verhalten: Vollständiges Struct-Literal
            origin : Point := (x := 0.0, y := 0.0);
            points : ARRAY[0..1] OF Point := [(x := 1.0, y := 1.0), (x := 2.0, y := 2.0)];
        END_VAR
        "#,
    );

    assert!(unresolvable.is_empty(), "Standard struct assignments should still work");
}

#[test]
fn test_struct_partial_assignment() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE Config :
            STRUCT
                enable : BOOL;
                timeout : TIME;
                priority : INT;
            END_STRUCT
        END_TYPE
        VAR_GLOBAL CONSTANT
            cfg : Config := (priority := 5);
        END_VAR
        "#,
    );

    assert!(unresolvable.is_empty(), "Partial named assignment should be supported");
}

#[test]
fn test_overflow_while_computing_binary() {
    let (_index, unresolvable) = eval_constants(
        r#"
        VAR_GLOBAL CONSTANT
            a : INT := 20;
            myVar : SINT := ((100 + 100) + a) - 100;
        END_VAR)
        "#,
    );

    assert_eq!(unresolvable.len(), 0);
}

//IR TEST
#[test]
fn const_variables_default_value_compile_time_evaluation() {
    // GIVEN some Iconstants index used as initializers
    let ir = codegen_with_new_constant_evaluator(
        "
        TYPE myEnum : (a,b,c); END_TYPE
        VAR_GLOBAL CONSTANT
            false_bool      : BOOL;
            zero_int        : INT;
            zero_real       : LREAL;
            empty_string    : STRING;
            null_ptr        : POINTER TO INT;
            zero_enum       : myEnum;
        END_VAR
        ",
    );
    filtered_assert_snapshot!(ir);
}

#[test]
fn test_standard_function_ir_generation() {
    // GIVEN an enum with its first value using a const-initializer
    let ir = codegen_with_new_constant_evaluator(
        "

        VAR_GLOBAL CONSTANT
            a : INT := ADD(1,2,3);
            b : ARRAY[0..5] OF INT := [3(4), ADD(2,2), MUL(2,2)];
        END_VAR
        ",
    );

    // me should be three
    filtered_assert_snapshot!(ir);
}

#[test]
fn test_complex_constant_evaluation() {
    let ir = codegen_with_new_constant_evaluator(
        "
        TYPE Config :
        STRUCT
            x : INT;
            y : REAL;
            flag : BOOL;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            a : INT := ADD(1, MUL(2,3), SUB(10,5));     // 12
            b : REAL := DIV(ADD(5, 5.0), 2);            // 5.0
            c : BOOL := GT(a, 10);                      // TRUE

            arr : ARRAY[0..3] OF INT := [
                ADD(1,1),       // 2
                MUL(2,3),       // 6
                SUB(10,4),      // 6
                DIV(8,2)        // 4
            ];
        END_VAR
        ",
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn test_type_promotion_and_limits() {
    let ir = codegen_with_new_constant_evaluator(
        "
        VAR_GLOBAL CONSTANT
            a : REAL := ADD(1, 2.5, 3);          // 6.5 (REAL promotion)
            b : REAL := MIN(5, 2.5, 8);          // 2.5
            c : REAL := MAX(1.0, 2, 3.5);        // 3.5
            d : INT  := LIMIT(0, -5, 10);        // 0
            e : INT  := LIMIT(0, 20, 10);        // 10
        END_VAR
        ",
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn const_enum_variable_default_value_compile_time_evaluation() {
    // GIVEN an enum with its first value using a const-initializer
    let ir = codegen_with_new_constant_evaluator(
        "

        VAR_GLOBAL CONSTANT
            me          : MyEnum;
            THREE       : INT := 3;
        END_VAR

        TYPE MyEnum       :       (a := THREE, b, c); END_TYPE
        ",
    );

    // me should be three
    filtered_assert_snapshot!(ir);
}

#[test]
fn default_values_are_transitive_for_range_types() {
    // GIVEN a range type that inherits the default value from its referenced type
    let src = codegen_with_new_constant_evaluator(
        "
        TYPE MyINT : INT := 7; END_TYPE

        TYPE MyRange : MyINT(1..10); END_TYPE

        VAR_GLOBAL
            a : MyINT;
            b : MyRange;
        END_VAR

        VAR_GLOBAL CONSTANT
            aa : MyINT;
            bb : MyRange;
            cc : INT := bb + aa;
        END_VAR
        ",
    );

    // THEN we expect the default value to be considered transitively
    // a & b should be 7, cc should be 14
    filtered_assert_snapshot!(src);
}

#[test]
fn mega_const_array_struct_test() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE Inner :
            STRUCT
                v : INT;
            END_STRUCT
        END_TYPE

        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        TYPE Outer :
            STRUCT
                arr : ARRAY[0..1] OF Inner;
            END_STRUCT
        END_TYPE

        TYPE MyEnum : (A, B, C); END_TYPE

        VAR_GLOBAL CONSTANT
            a : INT := 5;
            arr_mult_mixed    : ARRAY[0..5] OF INT := [2(1), 3(5)];
            arr_const         : ARRAY[0..2] OF INT := [a, a+1, a+2];
            arr_enum          : ARRAY[0..2] OF MyEnum := [MyEnum.A, MyEnum.C];
            arr_nested        : ARRAY[0..1] OF ARRAY[0..2] OF INT := [[1,2,3],[4,5,6]];

            arr_struct_partial : ARRAY[0..2] OF Point := [(x := 5)];
            arr_struct_full    : ARRAY[0..1] OF Point := [(x := 10, y := 10), (x := 20, y := 20)];

            outer_struct       : Outer := (arr := [(v := 1), (v := 2)]);

            arr_struct_default : ARRAY[0..2] OF Point;

            arr_err_too_many   : ARRAY[0..2] OF INT := [1,2,3,4];
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn fn_test_const_array_elements_in_structs() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE INNER_POINT :
            STRUCT
                x : INT(0..100);
                y : INT(-50..50);
            END_STRUCT
            END_TYPE

            TYPE SENSOR_DATA :
            STRUCT
                id      : INT(0..10);
                values  : ARRAY[0..3] OF INT(0..1000);
                points  : ARRAY[1..2] OF INNER_POINT;
            END_STRUCT
            END_TYPE

            TYPE DEVICE :
            STRUCT
                name    : STRING[20];
                sensors : ARRAY[0..1] OF SENSOR_DATA;

                matrix  : ARRAY[0..1] OF ARRAY[0..2] OF INT(-10..10);
            END_STRUCT
            END_TYPE


            VAR_GLOBAL CONSTANT
                devices : ARRAY[0..1] OF DEVICE :=
                [
                    (
                        name := 'DevA',
                        sensors := [
                            (
                                id := 1,
                                values := [10, 20, 30, 40],
                                points := [
                                    (x := 10, y := 5),
                                    (x := 20, y := -5)
                                ]
                            ),
                            (
                                id := 2,
                                values := [100, 200, 300, 400],
                                points := [
                                    (x := 30, y := 10),
                                    (x := 40, y := -10)
                                ]
                            )
                        ],

                        matrix := [
                            [1, 2, 3],
                            [4, 5, 6]
                        ]
                    ),

                    (
                        name := 'DevB',
                        sensors := [
                            (
                                id := 3,
                                values := [11, 22, 33, 44],
                                points := [
                                    (x := 50, y := 0),
                                    (x := 60, y := 1)
                                ]
                            ),
                            (
                                id := 4,
                                values := [111, 222, 333, 444],
                                points := [
                                    (x := 70, y := -1),
                                    (x := 80, y := 2)
                                ]
                            )
                        ],

                        matrix := [
                            [9, 8, 7],
                            [6, 5, 4]
                        ]
                    )
                ];
            END_VAR
        "#,
    );
    filtered_assert_snapshot!(ir);
}

#[test]
fn test_struct_array_subrange_violations() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE INNER :
        STRUCT
            x : INT(0..10);
            y : INT(-5..5);
        END_STRUCT
        END_TYPE

        TYPE CONTAINER :
        STRUCT
            vals   : ARRAY[0..2] OF INT(0..100);
            points : ARRAY[0..1] OF INNER;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            data : ARRAY[0..1] OF CONTAINER :=
            [
                (
                    vals := [10, 20, 30],
                    points := [
                        (x := 5, y := 0),
                        (x := 11, y := 0)  //err
                    ]
                ),
                (
                    vals := [10, 20, 30],
                    points := [
                        (x := 3, y := 0),
                        (x := 4, y := 2)
                    ]
                )
            ];
            data : ARRAY[0..1] OF CONTAINER :=
            [
                (
                    vals := [10, 20, 30],
                    points := [
                        (x := 5, y := 0),
                        (x := 11, y := 0)
                    ]
                ),
                (
                    vals := [10, 200, 30], //err
                    points := [
                        (x := 3, y := 0),
                        (x := 4, y := 2)
                    ]
                )
            ];
            data : ARRAY[0..1] OF CONTAINER :=
            [
                (
                    vals := [10, 20, 30],
                    points := [
                        (x := 5, y := 0),
                        (x := 11, y := 0)
                    ]
                ),
                (
                    vals := [10, 20, 30],
                    points := [
                        (x := 3, y := -6), //err
                        (x := 4, y := 2)
                    ]
                )
            ];
        END_VAR
        "#,
    );

    assert_eq!(unresolvable.len(), 3);
    assert!(unresolvable[0].get_reason().unwrap().contains("SubRange"));
    assert!(unresolvable[1].get_reason().unwrap().contains("SubRange"));
    assert!(unresolvable[2].get_reason().unwrap().contains("SubRange"));
}

#[test]
fn test_struct_with_callstatement_in_field() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            b : ARRAY[0..1] OF Point := [(x := ADD(1,2,3), y := 20)];
            bx : INT := b[0].x;
            bx1 : INT := b[0].y;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let arr = find_constant_value(&index, "b").unwrap();
    let elements = get_expression_list_nodes(arr);
    assert_eq!(elements.len(), 1);
    debug_assert_eq!(find_constant_value(&index, "bx"), Some(create_int_literal(6)));
    debug_assert_eq!(find_constant_value(&index, "bx1"), Some(create_int_literal(20)));
}

#[test]
fn test_simple_struct_with_single_callstatement() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            p : Point := (x := ADD(10, 20), y := 30);
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let p = find_constant_value(&index, "p").unwrap();
    let vals = extract_struct_as_strings(p);
    assert_eq!(vals, vec!["30", "30"]);
}

#[test]
fn test_struct_with_multiple_callstatements() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Calc :
            STRUCT
                sum : INT;
                product : INT;
                max_val : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            calc : Calc := (
                sum := ADD(5, 10, 15),
                product := MUL(2, 3, 4),
                max_val := MAX(100, 50, 75)
            );
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let calc = find_constant_value(&index, "calc").unwrap();
    let vals = extract_struct_as_strings(calc);
    assert_eq!(vals, vec!["30", "24", "100"]);
}

#[test]
fn test_nested_struct_with_callstatement() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Inner :
            STRUCT
                val : INT;
            END_STRUCT
        END_TYPE

        TYPE Outer :
            STRUCT
                inner : Inner;
                extra : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            nested : Outer := (
                inner := (val := ADD(1, 2, 3)),
                extra := 42
            );
            nestedVal : INT := nested.inner.val;
            nestedExtra : INT := nested.extra;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "nestedVal"), Some(create_int_literal(6)));
    debug_assert_eq!(find_constant_value(&index, "nestedExtra"), Some(create_int_literal(42)));
}

#[test]
fn test_struct_in_array_with_callstatement() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Item :
            STRUCT
                id : INT;
                value : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            items : ARRAY[0..2] OF Item := [
                (id := 1, value := ADD(100, 200)),
                (id := 2, value := ADD(50, 50)),
                (id := 3, value := MUL(10, 5))
            ];
            item0Id : INT := items[0].id;
            item0Value : INT := items[0].value;
            item1Id : INT := items[1].id;
            item1Value : INT := items[1].value;
            item2Id : INT := items[2].id;
            item2Value : INT := items[2].value;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let items = find_constant_value(&index, "items").unwrap();
    let elements = get_expression_list_nodes(items);
    assert_eq!(elements.len(), 3);
    debug_assert_eq!(find_constant_value(&index, "item0Id"), Some(create_int_literal(1)));
    debug_assert_eq!(find_constant_value(&index, "item0Value"), Some(create_int_literal(300)));
    debug_assert_eq!(find_constant_value(&index, "item1Id"), Some(create_int_literal(2)));
    debug_assert_eq!(find_constant_value(&index, "item1Value"), Some(create_int_literal(100)));
    debug_assert_eq!(find_constant_value(&index, "item2Id"), Some(create_int_literal(3)));
    debug_assert_eq!(find_constant_value(&index, "item2Value"), Some(create_int_literal(50)));
}

#[test]
//TODO: check this error
fn test_struct_with_real_callstatement() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE RealData :
            STRUCT
                sqrt_val : REAL;
                abs_val : REAL;
                combined : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            data : RealData := (
                sqrt_val := SQRT(16.0),
                abs_val := ABS(-3.14),
                combined := ADD(SQRT(4.0), SQRT(9.0))
            );
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    let data = find_constant_value(&index, "data").unwrap();
    let vals = extract_struct_as_strings(data);
    assert_eq!(vals, vec!["4", "3.14", "5"]);
}

#[test]
fn test_deeply_nested_struct_with_callstatement() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Level3 :
            STRUCT
                val : INT;
            END_STRUCT
        END_TYPE

        TYPE Level2 :
            STRUCT
                l3 : Level3;
            END_STRUCT
        END_TYPE

        TYPE Level1 :
            STRUCT
                l2 : Level2;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            deep : Level1 := (
                l2 := (
                    l3 := (val := ADD(1, 2, 3, 4, 5))
                )
            );
            deepVal : INT := deep.l2.l3.val;
            
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "deepVal"), Some(create_int_literal(15)));
}

#[test]
fn test_struct_array_with_nested_callstatements() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Inner :
            STRUCT
                a : INT;
                b : INT;
            END_STRUCT
        END_TYPE

        TYPE Outer :
            STRUCT
                items : ARRAY[0..1] OF Inner;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            complex : Outer := (
                items := [
                    (a := ADD(1, 2), b := MUL(3, 4)),
                    (a := ADD(5, 5), b := MUL(2, 5))
                ]
            );
            firstA : INT := complex.items[0].a;
            firstB : INT := complex.items[0].b;
            secondA : INT := complex.items[1].a;
            secondB : INT := complex.items[1].b;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "firstA"), Some(create_int_literal(3)));
    debug_assert_eq!(find_constant_value(&index, "firstB"), Some(create_int_literal(12)));
    debug_assert_eq!(find_constant_value(&index, "secondA"), Some(create_int_literal(10)));
    debug_assert_eq!(find_constant_value(&index, "secondB"), Some(create_int_literal(10)));
}

#[test]
fn codegen_struct_with_callstatement_simple() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            p : Point := (x := ADD(10, 5), y := 20);
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn codegen_struct_with_multiple_callstatements() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE Calculations :
            STRUCT
                sum : INT;
                product : INT;
                min_val : INT;
                max_val : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            calc : Calculations := (
                sum := ADD(5, 10, 15),
                product := MUL(2, 3, 4),
                min_val := MIN(100, 50, 75),
                max_val := MAX(100, 50, 75)
            );
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn codegen_array_of_struct_with_callstatements() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE Entry :
            STRUCT
                id : INT;
                computed : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            entries : ARRAY[0..2] OF Entry := [
                (id := 1, computed := ADD(100, 50)),
                (id := 2, computed := MUL(10, 10)),
                (id := 3, computed := MAX(75, 25))
            ];
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn codegen_nested_struct_with_callstatements() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE Inner :
            STRUCT
                computed : INT;
            END_STRUCT
        END_TYPE

        TYPE Outer :
            STRUCT
                inner : Inner;
                direct : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            outer : Outer := (
                inner := (computed := ADD(1, 2, 3)),
                direct := MUL(5, 5)
            );
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn codegen_struct_with_real_functions() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE RealCalc :
            STRUCT
                sqrt_result : REAL;
                abs_result : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            real_data : RealCalc := (
                sqrt_result := SQRT(25.0),
                abs_result := ABS(-42.5)
            );
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn codegen_complex_struct_array_with_nested_callstatements() {
    let ir = codegen_with_new_constant_evaluator(
        r#"
        TYPE SubItem :
            STRUCT
                value : INT;
            END_STRUCT
        END_TYPE

        TYPE Item :
            STRUCT
                id : INT;
                subitems : ARRAY[0..1] OF SubItem;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            items : ARRAY[0..1] OF Item := [
                (
                    id := 1,
                    subitems := [
                        (value := ADD(10, 20)),
                        (value := ADD(30, 40))
                    ]
                ),
                (
                    id := 2,
                    subitems := [
                        (value := MUL(5, 5)),
                        (value := MUL(10, 10))
                    ]
                )
            ];
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(ir);
}

#[test]
fn test_const_array_of_struct_field_access() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE MyStruct :
        STRUCT
            a : INT;
            b : INT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            arr : ARRAY[0..2] OF MyStruct :=[(a := 42, b := 100), (a := 32, b := 40), (a := 50, b := 60)];
            myArrI : ARRAY[0..2] OF INT := [1,2222,3];
            myArrR : ARRAY[0..2] OF REAL := [6543.44, 22.22, 45543.1];
            myR : ARRAY[0..5] OF INT := [2(5)];

            a : INT := myArrI[1];
            b : INT := myArrR[0];
            c : REAL := myArrR[1];
            d : ARRAY[1..2] OF INT := myArrR[0];
            e : INT := myArrR[0];

            myArr : ARRAY[myArrI[0]..myArrI[1]] OF INT := [1,2,3,4];

            x : MyStruct := arr[0];
            y : MyStruct := arr[1];

            //errors
            err : MyStruct := arr[3];
            err1 : MyStruct := arr[-3];
            err2 : MyStruct := arr[2.1];
            err3 : MyStruct := tarr[-3];
        END_VAR
        "#,
    );

    debug_assert_eq!(unresolvable.len(), 4);
    unresolvable[0].get_reason().unwrap().contains("SubRange");
    unresolvable[1].get_reason().unwrap().contains("Invalid array index");
    unresolvable[2].get_reason().unwrap().contains("Array index must be an integer");
    unresolvable[3].get_reason().unwrap().contains("Cannot use index access");

    debug_assert_eq!(find_constant_value(&index, "a"), Some(create_int_literal(2222)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(create_int_literal(6543)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(create_real_literal(22.22)));

    let act_my_struct = extract_struct_as_strings(find_constant_value(&index, "x").unwrap());
    debug_assert_eq!(act_my_struct[0], "42");
    debug_assert_eq!(act_my_struct[1], "100");

    let act_my_struct = extract_struct_as_strings(find_constant_value(&index, "y").unwrap());
    debug_assert_eq!(act_my_struct[0], "32");
    debug_assert_eq!(act_my_struct[1], "40");

    let arr = create_array_literal(vec![
        create_int_literal(1),
        create_int_literal(2),
        create_int_literal(3),
        create_int_literal(4),
    ]);
    debug_assert_eq!(&arr, find_constant_value(&index, "myArr").unwrap());
}


#[test]
fn test_struct_field_access_basic_success() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            myPoint : Point := (x := 5, y := 10);
            myPointX : Point := (x := 5);
            myPointY : Point := (y := 10);
            xValue : INT := myPoint.x;
            yValue : INT := myPoint.y;
            xxValue : INT := myPointX.x;
            yValueErr : INT := myPointX.y;
            xValueErr : INT := myPointY.x;
            yyValue : INT := myPointY.y;
        END_VAR
        "#,
    );

    debug_assert_eq!(2, unresolvable.len());
    debug_assert_eq!(find_constant_value(&index, "xValue"), Some(create_int_literal(5)));
    debug_assert_eq!(find_constant_value(&index, "yValue"), Some(create_int_literal(10)));
    debug_assert_eq!(find_constant_value(&index, "xxValue"), Some(create_int_literal(5)));
    debug_assert_eq!(find_constant_value(&index, "yyValue"), Some(create_int_literal(10)));
}

/// TEST 2: Nested Struct Field Access - success
#[test]
fn test_struct_field_access_nested_success() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE InnerPoint :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        TYPE OuterStruct :
            STRUCT
                point : InnerPoint;
                label : STRING;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            outer : OuterStruct := (
                point := (x := 7, y := 8),
                label := 'test'
            );
            nestedX : INT := outer.point.x;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "nestedX"), Some(create_int_literal(7)));
}

#[test]
fn test_program_struct_field_access_success() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Point :
            STRUCT
                x : ARRAY[0..4] OF INT;
                y : INT;
            END_STRUCT
        END_TYPE

        PROGRAM prg
            VAR CONSTANT
                point : Point := (x := [1,2,3,11,5], y := 22);
            END_VAR
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            xValue : INT := prg.point.x[3];
            yValue : INT := prg.point.y;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "xValue"), Some(create_int_literal(11)));
    debug_assert_eq!(find_constant_value(&index, "yValue"), Some(create_int_literal(22)));
}

#[test]
fn test_struct_and_array_combined_access_success() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE InnerElement :
            STRUCT
                id : INT;
                metrics : ARRAY[0..2] OF INT;
            END_STRUCT
        END_TYPE

        TYPE ComplexConfig :
            STRUCT
                elements : ARRAY[0..1] OF InnerElement;
                active : BOOL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            myConfig : ComplexConfig := (
                elements := [
                    (id := 10, metrics := [100, 101, 102]),
                    (id := 20, metrics := [200, 201, 202])
                ],
                active := TRUE
            );

            pointArray : ARRAY[0..1] OF InnerElement := [
                (id := 5, metrics := [50, 51, 52]),
                (id := 6, metrics := [60, 61, 62])
            ];

            nestedVal1 : INT := myConfig.elements[1].metrics[2]; // Erwartet: 202
            nestedVal2 : INT := myConfig.elements[0].id;         // Erwartet: 10

            arrayStructField : INT := pointArray[1].id;          // Erwartet: 6
            arrayStructArray : INT := pointArray[0].metrics[1];  // Erwartet: 51

            errOutOfBounds : INT := myConfig.elements[2].id;      // Fehler: Index 2 bei ARRAY[0..1] existiert nicht
            errInvalidField : INT := pointArray[0].wrongField;    // Fehler: Feld existiert nicht im Struct
        END_VAR
        "#,
    );

    debug_assert_eq!(2, unresolvable.len());

    debug_assert_eq!(find_constant_value(&index, "nestedVal1"), Some(create_int_literal(202)));
    debug_assert_eq!(find_constant_value(&index, "nestedVal2"), Some(create_int_literal(10)));
    debug_assert_eq!(find_constant_value(&index, "arrayStructField"), Some(create_int_literal(6)));
    debug_assert_eq!(find_constant_value(&index, "arrayStructArray"), Some(create_int_literal(51)));
}

/// TEST 4: Enum vs Struct Distinction
#[test]
fn test_struct_field_access_enum_distinction() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Color : (Red, Green, Blue); END_TYPE

        VAR_GLOBAL CONSTANT
            c1 : Color := Color.Red;
            c2 : INT := INT#Color.Green;
        END_VAR
        "#,
    );

    debug_assert_eq!(EMPTY, unresolvable);
    // enum constant resolves to numeric underlying value
    debug_assert_eq!(find_constant_value(&index, "c1"), Some(create_int_literal(0)));
    debug_assert_eq!(find_constant_value(&index, "c2"), Some(create_int_literal(1)));
}

/// TEST 5: Missing Field Access (should FAIL)
#[test]
fn test_struct_field_access_missing_field_fails() {
    let (_index, unresolvable) = eval_constants(
        r#"
        TYPE Point :
            STRUCT
                x : INT;
                y : INT;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            myPoint : Point := (x := 5, y := 10);
            badField : INT := myPoint.nonexistent;
        END_VAR
        "#,
    );

    assert_eq!(unresolvable.len(), 1);
    let reason = unresolvable[0].get_reason().unwrap_or_default();
    assert!(reason.contains("not found") || reason.contains("Field") || reason.contains("unknown"));
}

/// TEST 6: Partial Initialization and Defaults (BONUS)
#[test]
fn test_struct_field_access_partial_initialization() {
    let (index, unresolvable) = eval_constants(
        r#"
        TYPE Config :
            STRUCT
                id : INT := 1;
                value : REAL := 0.5;
                name : STRING := 'default';
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            cfg1 : Config;
            cfg2 : Config := (id := 99, value := 2.5);
            id1 : INT := cfg1.id;
            val2 : REAL := cfg2.value;
            name1 : STRING := cfg1.name;
        END_VAR
        "#,
    );

    debug_assert_eq!(2, unresolvable.len());
    debug_assert_eq!(find_constant_value(&index, "val2"), Some(create_real_literal(2.5)));
}

/// NO TYPE HINT FOR 2 DIM ARRAY
#[test]
fn test_struct_field_access_2dim_array_initializations() {
    let (index, unresolvable) = eval_constants(
        r#"
          TYPE LeafStruct :
            STRUCT
                target_val : INT;
            END_STRUCT
        END_TYPE

        TYPE Struct3 :
            STRUCT
                arr2 : ARRAY[0..1] OF LeafStruct;
            END_STRUCT
        END_TYPE

        TYPE Struct2 :
            STRUCT
                // Das Array von Arrays
                matrix : ARRAY[0..1] OF ARRAY[0..1] OF Struct3;
            END_STRUCT
        END_TYPE

        TYPE RootStruct :
            STRUCT
                arr1 : ARRAY[0..1] OF Struct2;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            megaStructure : RootStruct := (
                arr1 := [
                    (
                        matrix := [
                            [ // matrix[0]
                                (arr2 := [(target_val := 10), (target_val := 20)]), // matrix[0][0]
                                (arr2 := [(target_val := 30), (target_val := 99)])  // matrix[0][1]
                            ],
                            [ // matrix[1]
                                (arr2 := [(target_val := 40), (target_val := 50)]), // matrix[1][0]
                                (arr2 := [(target_val := 60), (target_val := 70)])  // matrix[1][1]
                            ]
                        ]
                    ),
                    (
                        matrix := [
                            [
                                (arr2 := [(target_val := 110), (target_val := 120)]),
                                (arr2 := [(target_val := 130), (target_val := 140)])
                            ],
                            [
                                (arr2 := [(target_val := 150), (target_val := 160)]),
                                (arr2 := [(target_val := 170), (target_val := 180)])
                            ]
                        ]
                    )
                ]
            );

            // Zugriffskette mit fortlaufendem Array-Index: ...matrix[0][1]...
            nestedResult : INT := megaStructure.arr1[0].matrix[0][1].arr2[1].target_val; // Erwartet: 99
        END_VAR
        "#,
    );

    debug_assert_eq!(0, unresolvable.len());
    debug_assert_eq!(find_constant_value(&index, "nestedResult"), Some(create_int_literal(99)));
}

#[test]
fn codegen_const_ultra_nested_struct_array_access() {
    let src = codegen_with_new_constant_evaluator(
        r#"
        TYPE LeafStruct :
            STRUCT
                target_val : INT;
            END_STRUCT
        END_TYPE

        TYPE Struct3 :
            STRUCT
                arr2 : ARRAY[0..1] OF LeafStruct;
            END_STRUCT
        END_TYPE

        TYPE Struct2 :
            STRUCT
                arr_mid : ARRAY[0..1] OF Struct3;
            END_STRUCT
        END_TYPE

        TYPE RootStruct :
            STRUCT
                arr1 : ARRAY[0..1] OF Struct2;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL CONSTANT
            megaStructure : RootStruct := (
                arr1 := [
                    (
                        arr_mid := [
                            (arr2 := [(target_val := 10), (target_val := 20)]), // arr_mid[0]
                            (arr2 := [(target_val := 30), (target_val := 99)])  // arr_mid[1]
                        ]
                    ),
                    (
                        arr_mid := [
                            (arr2 := [(target_val := 110), (target_val := 120)]),
                            (arr2 := [(target_val := 130), (target_val := 140)])
                        ]
                    )
                ]
            );

            // Zugriffskette: Struct -> Array -> Struct -> Array -> Struct -> Array -> Struct -> Feld
            nestedResult : INT := megaStructure.arr1[0].arr_mid[1].arr2[1].target_val; // Erwartet: 99
        END_VAR
        "#,
    );

    filtered_assert_snapshot!(src);
}
