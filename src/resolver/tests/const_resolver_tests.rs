use crate::index::LiteralValue;
use crate::resolver::const_evaluator::{evaluate_constants, UnresolvableConstant};
use crate::resolver::tests::parse;

const EMPTY: Vec<UnresolvableConstant> = vec![];

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
            real_eq_real : REAL := 5.3 = 2.1;
            real_neq_real : REAL := 5.3 <> 2.1;
            real_g_real : BOOL := 5.0 > 5.0;
            real_ge_real : BOOL := 5.0 >= 5.0;
            real_l_real : BOOL := 5.0 < 5.0;
            real_le_real : BOOL := 5.0 <= 5.0;

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
    let (index, unresolvable) = evaluate_constants(index);

    // THEN some type mixed comparisons could not be resolved (note that real == real or real <> real also dont work)
    let mut expected = vec![
        "real_eq_real",
        "real_neq_real",
        "int_eq_real",
        "int_neq_real",
        "real_eq_int",
        "real_neq_int",
        "int_g_real",
        "int_ge_real",
        "int_l_real",
        "int_le_real",
        "real_g_int",
        "real_ge_int",
        "real_l_int",
        "real_le_int",
        "real_g_real",
        "real_ge_real",
        "real_l_real",
        "real_le_real",
    ];
    expected.sort_unstable();
    let mut unresolvable: Vec<&str> = unresolvable
        .iter()
        .map(|it| it.qualified_name.as_str())
        .collect();
    unresolvable.sort_unstable();
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
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("int_g_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(true),
        index.find_constant_value("int_ge_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(false),
        index.find_constant_value("int_l_int").unwrap()
    );
    assert_eq!(
        &LiteralValue::Bool(true),
        index.find_constant_value("int_le_int").unwrap()
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
            a : INT := BOOL#16#00FF;
        END_VAR
       ",
    );

    // WHEN compile-time evaluation is applied
    let (_constants, unresolvable) = evaluate_constants(index);

    // THEN everything got resolved
    assert_eq!(
        vec![UnresolvableConstant::new(
            "a",
            "Cannot resolve constant: BOOL#LiteralInteger { value: 255 }"
        )],
        unresolvable
    );
}

#[test]
fn division_by_0_should_fail() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = parse(
        "VAR_GLOBAL CONSTANT
            zero_int : INT := 0;
            zero_real : REAL := 0.0;

            a : REAL := 5 / zero_int;
            b : REAL := 5 / zero_real;
            c : REAL := 5.0 / zero_int;
            d : REAL := 5.0 / zero_real;
            
            aa : REAL := 5 MOD zero_int;
            bb : REAL := 5 MOD zero_real;
            cc : REAL := 5.0 MOD zero_int;
            dd : REAL := 5.0 MOD zero_real;

        END_VAR
       ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);
    // THEN division by 0 are reported - note that division by 0.0 results in infinity
    assert_eq!(
        vec![
            UnresolvableConstant::new("a", "Attempt to divide by zero"),
            UnresolvableConstant::new("c", "Attempt to divide by zero"),
            UnresolvableConstant::new(
                "aa",
                "Attempt to calculate the remainder with a divisor of zero"
            ),
            UnresolvableConstant::new(
                "cc",
                "Attempt to calculate the remainder with a divisor of zero"
            ),
        ],
        unresolvable
    );
    // AND the real divisions are inf or nan
    assert_eq!(
        &LiteralValue::Real(f64::INFINITY),
        index.find_constant_value("b").unwrap()
    );
    assert_eq!(
        &LiteralValue::Real(f64::INFINITY),
        index.find_constant_value("d").unwrap()
    );

    if let LiteralValue::Real(bb) = index.find_constant_value("bb").unwrap() {
        assert!(bb.is_nan());
    } else {
        unreachable!()
    }
    if let LiteralValue::Real(dd) = index.find_constant_value("dd").unwrap() {
        assert!(dd.is_nan());
    } else {
        unreachable!()
    }
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
            b : BOOL := y OR NOT y;
            c : BOOL := z AND NOT z;
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
        Some(&LiteralValue::Bool(true))
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
    assert_eq!(
        vec![
            UnresolvableConstant::no_initial_value("c"),
            UnresolvableConstant::incomplete_initialzation("d"),
        ],
        unresolvable
    );
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
    assert_eq!(
        vec![
            UnresolvableConstant::incomplete_initialzation("a"),
            UnresolvableConstant::incomplete_initialzation("b"),
            UnresolvableConstant::incomplete_initialzation("c"),
            UnresolvableConstant::incomplete_initialzation("d"),
        ],
        unresolvable
    );
    assert_eq!(index.find_constant_value("aa"), Some(&LiteralValue::Int(4)));
    assert_eq!(index.find_constant_value("bb"), Some(&LiteralValue::Int(4)));
}

#[test]
fn evaluating_constant_strings() {
    // GIVEN some STRING constants used as initializers
    let (_, index) = parse(
        r#"VAR_GLOBAL CONSTANT
            a : STRING := 'Hello';
            b : WSTRING := "World";
        END_VAR
        
        VAR_GLOBAL CONSTANT
            aa : STRING := a;
            bb : WSTRING := b;
        END_VAR
        "#,
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    assert_eq!(EMPTY, unresolvable);

    // AND the globals should have gotten their values
    assert_eq!(
        index.find_constant_value("aa"),
        Some(&LiteralValue::String("Hello".to_string()))
    );
    assert_eq!(
        index.find_constant_value("bb"),
        Some(&LiteralValue::WString("World".to_string()))
    );
}

#[test]
fn const_string_initializers_should_be_converted() {
    // GIVEN some STRING constants used as initializers
    let (_, index) = parse(
        r#"VAR_GLOBAL CONSTANT
            a : STRING := 'Hello';
            b : WSTRING := "World";
        END_VAR
        
        VAR_GLOBAL CONSTANT
            aa : STRING := b;
            bb : WSTRING := a;
        END_VAR
        "#,
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    assert_eq!(EMPTY, unresolvable);

    // AND the globals should have gotten their values
    assert_eq!(
        index.find_constant_value("aa"),
        Some(&LiteralValue::String("World".to_string()))
    );
    assert_eq!(
        index.find_constant_value("bb"),
        Some(&LiteralValue::WString("Hello".to_string()))
    );
}
