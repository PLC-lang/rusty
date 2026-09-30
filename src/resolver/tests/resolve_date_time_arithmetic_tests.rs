use insta::assert_snapshot;
use plc_ast::{
    ast::{flatten_expression_list, Assignment, AstNode, AstStatement, CallStatement, CompilationUnit},
    literals::AstLiteral,
    provider::IdProvider,
};

use crate::{
    index::Index,
    resolver::{AnnotationMap, AnnotationMapImpl, StatementAnnotation},
    test_utils::tests::{annotate_with_ids, index_with_ids, DATE_TIME_ARITHMETIC_FUNCTIONS},
};

fn annotate(body: &str) -> (CompilationUnit, Index, AnnotationMapImpl) {
    let id_provider = IdProvider::default();
    let (unit, mut index) =
        index_with_ids(format!("{body}{DATE_TIME_ARITHMETIC_FUNCTIONS}"), id_provider.clone());
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    (unit, index, annotations)
}

// Renders the type of the right side of every assignment in the first POU, one per line
fn right_side_types(body: &str) -> String {
    let (unit, index, annotations) = annotate(body);
    unit.implementations[0]
        .statements
        .iter()
        .map(|statement| {
            let AstStatement::Assignment(Assignment { right, .. }) = statement.get_stmt() else {
                panic!("expected an assignment, got {statement:?}");
            };
            annotations.get_type_or_void(right, &index).get_name().to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Renders the right side of every assignment in the first POU: a replaced expression as the call
// it became, with nested replacements rendered the same way, a kept expression as `plain`
fn right_side_replacements(body: &str) -> String {
    let (unit, _, annotations) = annotate(body);
    unit.implementations[0]
        .statements
        .iter()
        .map(|statement| {
            let AstStatement::Assignment(Assignment { right, .. }) = statement.get_stmt() else {
                panic!("expected an assignment, got {statement:?}");
            };
            describe(&annotations, right)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn describe(annotations: &AnnotationMapImpl, node: &AstNode) -> String {
    let Some(StatementAnnotation::ReplacementAst { statement }) = annotations.get(node) else {
        return match node.get_stmt() {
            AstStatement::Literal(AstLiteral::Integer(value)) => value.to_string(),
            AstStatement::Literal(AstLiteral::Real(value)) => value.to_string(),
            AstStatement::ParenExpression(inner) => describe(annotations, inner),
            _ => node.get_flat_reference_name().unwrap_or("plain").to_string(),
        };
    };
    let AstStatement::CallStatement(CallStatement { operator, parameters }) = statement.get_stmt() else {
        panic!("expected a call, got {statement:?}");
    };
    let arguments = parameters
        .as_ref()
        .map(|it| flatten_expression_list(it))
        .unwrap_or_default()
        .iter()
        .map(|argument| describe(annotations, argument))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}({arguments})", operator.get_flat_reference_name().unwrap_or("?"))
}

#[test]
fn short_family_operators_are_replaced_by_the_standard_calls() {
    let replacements = right_side_replacements(
        r#"
        PROGRAM main
        VAR
            t1, t2 : TIME;
            tod1, tod2 : TOD;
            d1, d2 : DATE;
            dt1, dt2 : DT;
        END_VAR
            t1 := t1 + t2;
            tod1 := tod1 + t1;
            dt1 := dt1 + t1;
            t1 := t1 - t2;
            t1 := d1 - d2;
            tod1 := tod1 - t1;
            t1 := tod1 - tod2;
            dt1 := dt1 - t1;
            t1 := dt1 - dt2;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(replacements, @"
    ADD_TIME(t1, t2)
    ADD_TOD_TIME(tod1, t1)
    ADD_DT_TIME(dt1, t1)
    SUB_TIME(t1, t2)
    SUB_DATE_DATE(d1, d2)
    SUB_TOD_TIME(tod1, t1)
    SUB_TOD_TOD(tod1, tod2)
    SUB_DT_TIME(dt1, t1)
    SUB_DT_DT(dt1, dt2)
    ");
}

#[test]
fn scaling_calls_the_implementation_for_the_kind_of_number_with_the_duration_first() {
    let replacements = right_side_replacements(
        r#"
        PROGRAM main
        VAR
            t : TIME;
            lt : LTIME;
            n : DINT;
            u : UINT;
            r : REAL;
            lr : LREAL;
        END_VAR
            t := t * 2;
            t := 2 * t;
            t := t * u;
            t := t * r;
            t := t / lr;
            lt := lt * n;
            lt := r * lt;
            lt := lt / 4;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(replacements, @"
    MUL_TIME__LINT(t, 2)
    MUL_TIME__LINT(t, 2)
    MUL_TIME__ULINT(t, u)
    MUL_TIME__REAL(t, r)
    DIV_TIME__LREAL(t, lr)
    MUL_LTIME__LINT(lt, n)
    MUL_LTIME__REAL(lt, r)
    DIV_LTIME__LINT(lt, 4)
    ");
}

#[test]
fn a_chain_is_replaced_from_the_inside_out() {
    let replacements = right_side_replacements(
        r#"
        PROGRAM main
        VAR
            t, t2 : TIME;
            dt : DT;
        END_VAR
            dt := dt + t + t * 2 - t2;
            t := (dt + t) - dt;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(replacements, @"
    SUB_DT_TIME(ADD_DT_TIME(ADD_DT_TIME(dt, t), MUL_TIME__LINT(t, 2)), t2)
    SUB_DT_DT(ADD_DT_TIME(dt, t), dt)
    ");
}

#[test]
fn expressions_without_a_call_are_not_replaced() {
    let replacements = right_side_replacements(
        r#"
        PROGRAM main
        VAR
            t : TIME;
            dt : DT;
            n : DINT;
        END_VAR
            t := t + 5;
            t := n + t;
            dt := dt + dt;
            t := n / t;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(replacements, @"
    plain
    plain
    plain
    plain
    ");
}

#[test]
fn short_family_operators_are_typed_by_their_result() {
    let types = right_side_types(
        r#"
        PROGRAM main
        VAR
            t1, t2 : TIME;
            tod1, tod2 : TOD;
            d1, d2 : DATE;
            dt1, dt2 : DT;
        END_VAR
            t1 := t1 + t2;
            tod1 := tod1 + t1;
            dt1 := dt1 + t1;
            t1 := t1 - t2;
            t1 := d1 - d2;
            tod1 := tod1 - t1;
            t1 := tod1 - tod2;
            dt1 := dt1 - t1;
            t1 := dt1 - dt2;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(types, @"
    TIME
    TIME_OF_DAY
    DATE_AND_TIME
    TIME
    TIME
    TIME_OF_DAY
    TIME
    DATE_AND_TIME
    TIME
    ");
}

#[test]
fn long_family_operators_are_typed_by_their_result() {
    let types = right_side_types(
        r#"
        PROGRAM main
        VAR
            t1, t2 : LTIME;
            tod1, tod2 : LTOD;
            d1, d2 : LDATE;
            dt1, dt2 : LDT;
        END_VAR
            t1 := t1 + t2;
            tod1 := tod1 + t1;
            dt1 := dt1 + t1;
            t1 := t1 - t2;
            t1 := d1 - d2;
            tod1 := tod1 - t1;
            t1 := tod1 - tod2;
            dt1 := dt1 - t1;
            t1 := dt1 - dt2;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(types, @"
    LTIME
    LTIME_OF_DAY
    LDATE_AND_TIME
    LTIME
    LTIME
    LTIME_OF_DAY
    LTIME
    LDATE_AND_TIME
    LTIME
    ");
}

#[test]
fn a_scaled_duration_keeps_its_type_in_both_operand_orders() {
    let types = right_side_types(
        r#"
        PROGRAM main
        VAR
            t : TIME;
            lt : LTIME;
            n : DINT;
            r : LREAL;
        END_VAR
            t := t * 2;
            t := 2 * t;
            t := t * r;
            t := t / n;
            t := t / 2.5;
            lt := lt * n;
            lt := r * lt;
            lt := lt / 4;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(types, @"
    TIME
    TIME
    TIME
    TIME
    TIME
    LTIME
    LTIME
    LTIME
    ");
}

#[test]
fn a_chain_is_typed_from_the_inside_out() {
    let types = right_side_types(
        r#"
        PROGRAM main
        VAR
            t, t2 : TIME;
            dt : DT;
        END_VAR
            dt := dt + t + t * 2 - t2;
            t := (dt + t) - dt;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(types, @"
    DATE_AND_TIME
    TIME
    ");
}

#[test]
fn a_bare_integer_with_a_duration_takes_the_duration_type() {
    let body = r#"
        PROGRAM main
        VAR
            t : TIME;
            lt : LTIME;
            n : DINT;
        END_VAR
            t := t + 5;
            t := t - n;
            t := 5 + t;
            lt := lt + 5;
        END_PROGRAM
        "#;
    assert_snapshot!(right_side_types(body), @"
    TIME
    TIME
    TIME
    LTIME
    ");

    // the integer operand is hinted with the duration's type so that codegen widens or narrows it
    let (unit, index, annotations) = annotate(body);
    let hints = unit.implementations[0]
        .statements
        .iter()
        .map(|statement| {
            let AstStatement::Assignment(Assignment { right, .. }) = statement.get_stmt() else {
                panic!("expected an assignment, got {statement:?}");
            };
            let AstStatement::BinaryExpression(expression) = right.get_stmt() else {
                panic!("expected a binary expression, got {right:?}");
            };
            let integer = if expression.left.is_literal() { &expression.left } else { &expression.right };
            annotations.get_type_hint(integer, &index).map(|it| it.get_name()).unwrap_or("none").to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_snapshot!(hints, @"
    TIME
    TIME
    TIME
    LTIME
    ");
}

#[test]
fn an_undefined_combination_falls_back_to_the_numeric_typing() {
    let types = right_side_types(
        r#"
        PROGRAM main
        VAR
            t : TIME;
            lt : LTIME;
            dt : DT;
            n : DINT;
        END_VAR
            dt := dt + dt;
            t := n / t;
            lt := t + lt;
        END_PROGRAM
        "#,
    );

    assert_snapshot!(types, @"
    DATE_AND_TIME
    DINT
    LTIME
    ");
}
