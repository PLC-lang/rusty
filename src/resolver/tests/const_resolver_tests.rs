use crate::index::LiteralValue;
use crate::resolver::const_evaluator::evaluate_constants;
use crate::resolver::tests::parse;

const EMPTY: Vec<String> = vec![];

#[test]
fn const_references_to_int_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
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
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.2),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.0),
        index.find_constant_value("e").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.0),
        index.find_constant_value("f").unwrap()
    );
}

#[test]
fn const_references_to_int_additions_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
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
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(11),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.2),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.0),
        index.find_constant_value("e").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(11.7),
        index.find_constant_value("f").unwrap()
    );
}

#[test]
fn const_references_to_int_subtractions_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
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
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(-3),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.2),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.0),
        index.find_constant_value("e").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(-3.7),
        index.find_constant_value("f").unwrap()
    );
}

#[test]
fn const_references_to_int_multiplications_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
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
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(28),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.2),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(4.0),
        index.find_constant_value("e").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(30.8),
        index.find_constant_value("f").unwrap()
    );
}

#[test]
fn const_references_to_int_division_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
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
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(
        &LiteralValue::Int(40),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(40),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(5),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(40.2),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(40.0),
        index.find_constant_value("e").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(40_f64 / 7.7),
        index.find_constant_value("f").unwrap()
    );
}

#[test]
fn const_references_int_float_type_behavior_evaluation() {
    // GIVEN some INT index used as initializers
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

            //BOOL - BOOL
            _true_ : BOOL := TRUE;
            _false_ : BOOL := FALSE;
            bool_and_bool : BOOL := _true_ AND _true_;
            bool_or_bool : BOOL := _true_ OR _false_;
            bool_xor_bool : BOOL := _true_ XOR _true_;
            not_bool : BOOL := NOT _true_;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, mut unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    let mut expected = vec![
        "real_eq_real".to_string(),
        "real_neq_real".to_string(),
        "int_eq_real".to_string(),
        "int_neq_real".to_string(),
        "real_eq_int".to_string(),
        "real_neq_int".to_string(),
    ];
    expected.sort();
    unresolvable.sort();
    assert_eq!(expected, unresolvable);
    // INT - INT
    assert_eq!(
        &LiteralValue::Int(4),
        index.find_constant_value("int_plus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(2),
        index.find_constant_value("int_minus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(6),
        index.find_constant_value("int_mul_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(2),
        index.find_constant_value("int_div_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(5 % 2),
        index.find_constant_value("int_mod_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(true),
        index.find_constant_value("int_eq_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("int_neq_int").unwrap()
    );
    // INT - REAL
    assert_eq!(
        &LiteralValue::Real(4.1),
        index.find_constant_value("int_plus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(3.0 - 1.1),
        index.find_constant_value("int_minus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(3.0 * 1.1),
        index.find_constant_value("int_mul_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(5.0 / 2.1),
        index.find_constant_value("int_div_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(5.0 % 2.1),
        index.find_constant_value("int_mod_real").unwrap()
    );
    // REAL - INT
    assert_eq!(
        &LiteralValue::Real(4.3),
        index.find_constant_value("real_plus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(2.3),
        index.find_constant_value("real_minus_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(6.6),
        index.find_constant_value("real_mul_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(5.2 / 2.0),
        index.find_constant_value("real_div_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(5.2 % 2.0),
        index.find_constant_value("real_mod_int").unwrap()
    );
    // REAL - REAL
    assert_eq!(
        &LiteralValue::Real(4.4),
        index.find_constant_value("real_plus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(3.3 - 1.1),
        index.find_constant_value("real_minus_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(3.3 * 1.1),
        index.find_constant_value("real_mul_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(5.3 / 2.1),
        index.find_constant_value("real_div_real").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(5.3 % 2.1),
        index.find_constant_value("real_mod_real").unwrap()
    );
    // BOOL - BOOL
    assert_eq!(
        &LiteralValue::Bool(true),
        index.find_constant_value("bool_and_bool").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(true),
        index.find_constant_value("bool_or_bool").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("bool_xor_bool").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("not_bool").unwrap()
    );
}

#[test]
fn const_references_bool_bit_functions_behavior_evaluation() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            _true : BOOL := TRUE;
            _false : BOOL := FALSE;
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : WORD := _true;
            b : WORD := a AND _false;
            c : WORD := a OR _false;
            d : WORD := a XOR _true;
            e : WORD := NOT a;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN everything got resolved
    assert_eq!(EMPTY, unresolvable);
    // AND the index should have literal values
    assert_eq!(
        &LiteralValue::Bool(true),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(true),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("e").unwrap()
    );
}

#[test]
fn const_references_int_bit_functions_behavior_evaluation() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = parse(
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

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN everything got resolved
    assert_eq!(EMPTY, unresolvable);
    // AND the index should have literal values
    assert_eq!(
        &LiteralValue::Int(0xFFAB),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(0x00AB),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(0xFFFF),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(0xFF54),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(0x0054),
        index.find_constant_value("e").unwrap()
    );
}
#[test]
fn illegal_cast_should_not_be_resolved() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            a : INT := BOOL#00FF;
        END_VAR
       ",
    );

    // WHEN compile-time evaluation is applied
    let (_constants, unresolvable) = evaluate_constants(index);

    // THEN everything got resolved
    assert_eq!(vec!["a"], unresolvable);
}

#[test]
fn const_references_not_function_with_signed_ints() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            _0x00ff : INT := 16#00FF; //255
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : INT := INT#16#FFAB;//-85;
            aa : INT := WORD#16#FFAB;//65xxx;
            b : INT := a AND _0x00ff; //171
            c : INT := a OR _0x00ff; //-1
            d : INT := a XOR _0x00ff; //-172
            e : INT := NOT a; //84
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN everything got resolved
    assert_eq!(EMPTY, unresolvable);
    // AND the index should have literal values
    assert_eq!(
        &LiteralValue::Int(-85),
        index.find_constant_value("a").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(0x0000_ffab),
        index.find_constant_value("aa").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(171),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(-1),
        index.find_constant_value("c").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(-172),
        index.find_constant_value("d").unwrap()
    );
    assert_eq!(
        &LiteralValue::Int(84),
        index.find_constant_value("e").unwrap()
    );
}

#[test]
fn const_references_to_bool_compile_time_evaluation() {
    // GIVEN some BOOL index used as initializers
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
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(EMPTY, unresolvable);
    assert_eq!(
        index.find_constant_value("a"),
        Some(&LiteralValue::Bool(true))
    );
    assert_eq!(
        index.find_constant_value("b"),
        Some(&LiteralValue::Bool(false))
    );
    assert_eq!(
        index.find_constant_value("c"),
        Some(&LiteralValue::Bool(false))
    );
}

#[test]
fn not_evaluatable_consts_are_reported() {
    // GIVEN some BOOL index used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            a : INT := 1;
            b : INT := a;
            c : INT;
            d : INT := c;
        END_VAR",
    );

    // WHEN compile-time evaluation is applied
    let (_, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(vec!["c".to_string(), "d".to_string()], unresolvable);
}

#[test]
fn evaluating_constants_can_handle_recursion() {
    // GIVEN some BOOL index used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            a : INT := d;
            b : INT := a;
            c : INT := b;
            d : INT := a;

            aa : INT := 4;
            bb : INT := aa;
        END_VAR",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    assert_eq!(vec!["a", "b", "c", "d"], unresolvable);
    assert_eq!(index.find_constant_value("aa"), Some(&LiteralValue::Int(4)));
    assert_eq!(index.find_constant_value("bb"), Some(&LiteralValue::Int(4)));
}
