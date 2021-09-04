use crate::resolver::const_evaluator::{evaluate_constants, LiteralValue};
use crate::resolver::tests::parse;

const EMPTY: Vec<String> = vec![];

#[test]
fn const_references_to_int_compile_time_evaluation() {
    // GIVEN some INT constants used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            rX : LREAL := 4.2;
            iY : INT := iX;
            rY : LREAL := iX;
            iZ : INT := iY;
            rZ : LREAL := rY;
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : INT := iX;
            b : INT := iY;
            c : INT := iZ;
            d : LREAL := rX;
            e : LREAL := rY;
            f : LREAL := rZ;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (constants, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("a").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("b").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("c").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.2), constants.get("d").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.0), constants.get("e").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.0), constants.get("f").unwrap());
}

#[test]
fn const_references_to_int_additions_compile_time_evaluation() {
    // GIVEN some INT constants used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            rX : LREAL := 4.2;
            iY : INT := iX;
            rY : LREAL := iX;
            iZ : INT := iY + 7;
            rZ : LREAL := rY + 7.7;
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : INT := iX;
            b : INT := iY;
            c : INT := iZ;
            d : LREAL := rX;
            e : LREAL := rY;
            f : LREAL := rZ;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (constants, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("a").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("b").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(11), constants.get("c").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.2), constants.get("d").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.0), constants.get("e").unwrap());
    assert_eq!(
        &LiteralValue::RealLiteral(11.7),
        constants.get("f").unwrap()
    );
}

#[test]
fn const_references_to_int_subtractions_compile_time_evaluation() {
    // GIVEN some INT constants used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            rX : LREAL := 4.2;
            iY : INT := iX;
            rY : LREAL := iX;
            iZ : INT := iY - 7;
            rZ : LREAL := rY - 7.7;
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : INT := iX;
            b : INT := iY;
            c : INT := iZ;
            d : LREAL := rX;
            e : LREAL := rY;
            f : LREAL := rZ;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (constants, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("a").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("b").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(-3), constants.get("c").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.2), constants.get("d").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.0), constants.get("e").unwrap());
    assert_eq!(
        &LiteralValue::RealLiteral(-3.7),
        constants.get("f").unwrap()
    );
}

#[test]
fn const_references_to_int_multiplications_compile_time_evaluation() {
    // GIVEN some INT constants used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            rX : LREAL := 4.2;
            iY : INT := iX;
            rY : LREAL := iX;
            iZ : INT := iY * 7;
            rZ : LREAL := rY * 7.7;
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : INT := iX;
            b : INT := iY;
            c : INT := iZ;
            d : LREAL := rX;
            e : LREAL := rY;
            f : LREAL := rZ;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (constants, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("a").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(4), constants.get("b").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(28), constants.get("c").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.2), constants.get("d").unwrap());
    assert_eq!(&LiteralValue::RealLiteral(4.0), constants.get("e").unwrap());
    assert_eq!(
        &LiteralValue::RealLiteral(30.8),
        constants.get("f").unwrap()
    );
}

#[test]
fn const_references_to_int_division_compile_time_evaluation() {
    // GIVEN some INT constants used as initializers
    let (_, index) = parse(
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
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (constants, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(&LiteralValue::IntLiteral(40), constants.get("a").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(40), constants.get("b").unwrap());
    assert_eq!(&LiteralValue::IntLiteral(5), constants.get("c").unwrap());
    assert_eq!(
        &LiteralValue::RealLiteral(40.2),
        constants.get("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(40.0),
        constants.get("e").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(40_f64 / 7.7),
        constants.get("f").unwrap()
    );
}

#[test]
fn const_references_int_float_type_behavior_evaluation() {
    // GIVEN some INT constants used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            // INT - INT
            int_plus_int : INT := 3 + 1;
            int_minus_int : INT := 3 - 1;
            int_mul_int : INT := 3 * 2;
            int_div_int : INT := 5 / 2;
            int_mod_int : INT := 5 MOD 2;
            int_eq_int : INT := 5 = 5;
            int_neq_int : INT := 5 <> 5;

            // INT - REAL
            int_plus_real : REAL := 3 + 1.1;
            int_minus_real : REAL := 3 - 1.1;
            int_mul_real : REAL := 3 * 1.1;
            int_div_real : REAL := 5 / 2.1;
            int_mod_real : REAL := 5 MOD 2.1;
            int_eq_real : REAL := 5 = 2.1;
            int_neq_real : REAL := 5 <> 2.1;

            // REAL - INT
            real_plus_int : REAL := 3.3 + 1;
            real_minus_int : REAL := 3.3 - 1;
            real_mul_int : REAL := 3.3 * 2;
            real_div_int : REAL := 5.2 / 2;
            real_mod_int : REAL := 5.2 MOD 2;
            real_eq_int : REAL := 5.2 = 2;
            real_neq_int : REAL := 5.2 <> 2;

            // REAL - REAL
            real_plus_real : REAL := 3.3 + 1.1;
            real_minus_real : REAL := 3.3 - 1.1;
            real_mul_real : REAL := 3.3 * 1.1;
            real_div_real : REAL := 5.3 / 2.1;
            real_mod_real : REAL := 5.3 MOD 2.1;
            real_eq_real : REAL := 5.3 = 2.1;
            real_neq_real : REAL := 5.3 <> 2.1;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (constants, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(
        vec![
            "real_eq_real".to_string(),
            "real_neq_real".to_string(),
            "int_eq_real".to_string(),
            "int_neq_real".to_string(),
            "real_eq_int".to_string(),
            "real_neq_int".to_string(),
        ],
        unresolvable
    );
    // INT - INT
    assert_eq!(
        &LiteralValue::IntLiteral(4),
        constants.get("int_plus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::IntLiteral(2),
        constants.get("int_minus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::IntLiteral(6),
        constants.get("int_mul_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::IntLiteral(2),
        constants.get("int_div_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::IntLiteral(5 % 2),
        constants.get("int_mod_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::BoolLiteral(true),
        constants.get("int_eq_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::BoolLiteral(false),
        constants.get("int_neq_int").unwrap()
    );
    // INT - REAL
    assert_eq!(
        &LiteralValue::RealLiteral(4.1),
        constants.get("int_plus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(3.0 - 1.1),
        constants.get("int_minus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(3.0 * 1.1),
        constants.get("int_mul_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(5.0 / 2.1),
        constants.get("int_div_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(5.0 % 2.1),
        constants.get("int_mod_real").unwrap()
    );
    // REAL - INT
    assert_eq!(
        &LiteralValue::RealLiteral(4.3),
        constants.get("real_plus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(2.3),
        constants.get("real_minus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(6.6),
        constants.get("real_mul_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(5.2 / 2.0),
        constants.get("real_div_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(5.2 % 2.0),
        constants.get("real_mod_int").unwrap()
    );
    // REAL - REAL
    assert_eq!(
        &LiteralValue::RealLiteral(4.4),
        constants.get("real_plus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(3.3 - 1.1),
        constants.get("real_minus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(3.3 * 1.1),
        constants.get("real_mul_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(5.3 / 2.1),
        constants.get("real_div_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::RealLiteral(5.3 % 2.1),
        constants.get("real_mod_real").unwrap()
    );
}

#[test]
fn const_references_to_bool_compile_time_evaluation() {
    // GIVEN some BOOL constants used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            x : BOOL := TRUE;
            y : BOOL := FALSE;
            z : BOOL := y;
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : BOOL := x;
            b : BOOL := y;
            c : BOOL := z;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (constants, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(constants.get("a"), Some(&LiteralValue::BoolLiteral(true)));
    assert_eq!(constants.get("b"), Some(&LiteralValue::BoolLiteral(false)));
    assert_eq!(constants.get("c"), Some(&LiteralValue::BoolLiteral(false)));
}

#[test]
fn not_evaluatable_consts_are_reported() {
    // GIVEN some BOOL constants used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            a : INT := 1;
            b : INT := a;
            c : INT;
            d : INT := c;
        END_VAR",
    );

    // WHEN compile-time evaluation is applied
    let (_, unresolvable) = evaluate_constants(&index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(vec!["c".to_string(), "d".to_string()], unresolvable);
}
