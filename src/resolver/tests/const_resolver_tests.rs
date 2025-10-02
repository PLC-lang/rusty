use plc_ast::ast::{AstFactory, AstNode, AstStatement};
use plc_ast::literals::{Array, AstLiteral};
use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocation;
use plc_util::filtered_assert_snapshot;

use crate::index::const_expressions::{ConstExpression, UnresolvableKind};
use crate::index::Index;

use crate::resolver::const_evaluator::{evaluate_constants, UnresolvableConstant};
use crate::resolver::AnnotationMap;
use crate::test_utils::tests::{annotate_with_ids, codegen, index, index_with_ids};
use crate::typesystem::DataTypeInformation;

const EMPTY: Vec<UnresolvableConstant> = vec![];

///locally overwerite assert_eq to assert the Debug-Equality
macro_rules! debug_assert_eq {
    ($left:expr, $right:expr) => {
        assert_eq!(format!("{:#?}", $left), format!("{:#?}", $right))
    };
}

macro_rules! global {
    ($index:expr, $name:expr) => {
        $index.find_global_variable($name).unwrap().initial_value.unwrap()
    };
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

fn create_string_literal(v: &str, wide: bool) -> AstNode {
    AstFactory::create_literal(AstLiteral::new_string(v.to_string(), wide), SourceLocation::internal(), 0)
}

fn create_real_literal(v: f64) -> AstNode {
    AstFactory::create_literal(AstLiteral::new_real(format!("{v:}")), SourceLocation::internal(), 0)
}

fn create_bool_literal(v: bool) -> AstNode {
    AstFactory::create_literal(AstLiteral::new_bool(v), SourceLocation::internal(), 0)
}

#[test]
fn const_references_to_int_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
    let (_, index) = index(
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

    // THEN a to f got their correct initial-literals
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_real_literal(4.2), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_real_literal(4.0), find_constant_value(&index, "e").unwrap());
    debug_assert_eq!(&create_real_literal(4.0), find_constant_value(&index, "f").unwrap());
}

#[test]
fn const_variables_default_value_compile_time_evaluation() {
    // GIVEN some Iconstants index used as initializers
    let ir = codegen(
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
fn const_enum_variable_default_value_compile_time_evaluation() {
    // GIVEN an enum with its first value using a const-initializer
    let ir = codegen(
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
fn local_const_references_to_int_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
    let (_, index) = index(
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

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a to f got their correct initial-literals
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_real_literal(4.2), find_constant_value(&index, "b").unwrap());
}

#[test]
fn local_const_references_to_int_compile_time_evaluation_uses_correct_scopes() {
    // GIVEN some global and local constants
    let (_, index) = index(
        "
        VAR_GLOBAL CONSTANT
            a : INT := 5;
        END_VAR

        VAR_GLOBAL
            g : INT := a; //should be 5
            h : INT := prg.a; // should be 4
        END_VAR

        PROGRAM prg
            VAR CONSTANT
                a : INT := 4;
            END_VAR

            VAR_INPUT
                v : INT := a; //should be 4
            END_VAR
        END_PROGRAM
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);
    debug_assert_eq!(EMPTY, unresolvable);

    // THEN g should resolve its inital value to global 'a'
    debug_assert_eq!(&create_int_literal(5), find_constant_value(&index, "g").unwrap());
    // THEN h should resolve its inital value to 'prg.a'
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "h").unwrap());
    // AND prg.v should resolve its initial value to 'prg.a'
    debug_assert_eq!(&create_int_literal(4), find_member_value(&index, "prg", "v").unwrap());
}

#[test]
fn non_const_references_to_int_compile_time_evaluation() {
    // GIVEN some global consts
    // AND some NON-constants
    let (_, index) = index(
        "VAR_GLOBAL CONSTANT
            iX : INT := 2;
        END_VAR

        VAR_GLOBAL
            a : INT := 3;
            b : INT := 4;
        END_VAR

        VAR_GLOBAL CONSTANT
            ok      : INT := iX;
            nok_a   : INT := iX + a;
            nok_b   : INT := iX + b;

            temp        : INT := a;
            incomplete  : INT := temp;
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a to f got their correct initial-literals
    debug_assert_eq!(&create_int_literal(2), find_constant_value(&index, "ok").unwrap());

    debug_assert_eq!(
        vec![
            UnresolvableConstant::new(global!(index, "nok_a"), "`a` is no const reference"),
            UnresolvableConstant::new(global!(index, "nok_b"), "`b` is no const reference"),
            UnresolvableConstant::new(global!(index, "temp"), "`a` is no const reference"),
            UnresolvableConstant::incomplete_initialzation(&global!(index, "incomplete")), //this one is fine, but one depency cannot be resolved
        ],
        unresolvable
    );
}

#[test]
fn prg_members_initials_compile_time_evaluation() {
    // GIVEN some member variables with const initializers
    let (_, index) = index(
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
       END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN everything got resolved
    debug_assert_eq!(EMPTY, unresolvable);
    // AND the program-members got their correct initial-literals
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
    // GIVEN some INT index used as initializers
    let (_, index) = index(
        "VAR_GLOBAL CONSTANT
            iX : INT := 4;
            rX : LREAL := 4.2;
        END_VAR

        VAR_GLOBAL CONSTANT
            a : INT := -iX;
            b : LREAL := -rX;
            c : INT := -5;
       END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a,b,and c got their correct initial-literals
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(-4), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_real_literal(-4.2), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(-5), find_constant_value(&index, "c").unwrap());
}

#[test]
fn const_references_to_int_additions_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
    let (_, index) = index(
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
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(11), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_real_literal(4.2), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_real_literal(4.0), find_constant_value(&index, "e").unwrap());
    debug_assert_eq!(&create_real_literal(11.7), find_constant_value(&index, "f").unwrap());
}

#[test]
fn const_references_to_int_subtractions_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
    let (_, index) = index(
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
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(-3), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_real_literal(4.2), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_real_literal(4.0), find_constant_value(&index, "e").unwrap());
    debug_assert_eq!(&create_real_literal(-3.7), find_constant_value(&index, "f").unwrap());
}

#[test]
fn const_references_to_int_multiplications_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
    let (_, index) = index(
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
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(28), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_real_literal(4.2), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_real_literal(4.0), find_constant_value(&index, "e").unwrap());
    debug_assert_eq!(&create_real_literal(30.8), find_constant_value(&index, "f").unwrap());
}

#[test]
fn const_references_to_int_division_compile_time_evaluation() {
    // GIVEN some INT index used as initializers
    let (_, index) = index(
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
    // GIVEN some INT index used as initializers
    let (_, index) = index(
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
    let (index, _) = evaluate_constants(index);

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

    let mut unresolvable: Vec<&str> = index
        .get_globals()
        .values()
        .filter(|it| {
            let const_expr =
                index.get_const_expressions().find_const_expression(it.initial_value.as_ref().unwrap());
            matches!(const_expr, Some(ConstExpression::Unresolvable { .. }))
        })
        .map(|it| it.get_qualified_name())
        .collect();
    unresolvable.sort_unstable();
    debug_assert_eq!(expected, unresolvable);

    //
    // INT - INT
    debug_assert_eq!(&create_int_literal(4), find_constant_value(&index, "int_plus_int").unwrap());
    debug_assert_eq!(&create_int_literal(2), find_constant_value(&index, "int_minus_int").unwrap());
    debug_assert_eq!(&create_int_literal(6), find_constant_value(&index, "int_mul_int").unwrap());
    debug_assert_eq!(&create_int_literal(2), find_constant_value(&index, "int_div_int").unwrap());
    debug_assert_eq!(&create_int_literal(5 % 2), find_constant_value(&index, "int_mod_int").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "int_eq_int").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "int_neq_int").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "int_g_int").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "int_ge_int").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "int_l_int").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "int_le_int").unwrap());
    // INT - REAL
    debug_assert_eq!(&create_real_literal(4.1), find_constant_value(&index, "int_plus_real").unwrap());
    debug_assert_eq!(&create_real_literal(3.0 - 1.1), find_constant_value(&index, "int_minus_real").unwrap());
    debug_assert_eq!(&create_real_literal(3.0 * 1.1), find_constant_value(&index, "int_mul_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.0 / 2.1), find_constant_value(&index, "int_div_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.0 % 2.1), find_constant_value(&index, "int_mod_real").unwrap());
    // REAL - INT
    debug_assert_eq!(&create_real_literal(4.3), find_constant_value(&index, "real_plus_int").unwrap());
    debug_assert_eq!(&create_real_literal(2.3), find_constant_value(&index, "real_minus_int").unwrap());
    debug_assert_eq!(&create_real_literal(6.6), find_constant_value(&index, "real_mul_int").unwrap());
    debug_assert_eq!(&create_real_literal(5.2 / 2.0), find_constant_value(&index, "real_div_int").unwrap());
    debug_assert_eq!(&create_real_literal(5.2 % 2.0), find_constant_value(&index, "real_mod_int").unwrap());
    // REAL - REAL
    debug_assert_eq!(&create_real_literal(4.4), find_constant_value(&index, "real_plus_real").unwrap());
    debug_assert_eq!(
        &create_real_literal(3.3 - 1.1),
        find_constant_value(&index, "real_minus_real").unwrap()
    );
    debug_assert_eq!(&create_real_literal(3.3 * 1.1), find_constant_value(&index, "real_mul_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.3 / 2.1), find_constant_value(&index, "real_div_real").unwrap());
    debug_assert_eq!(&create_real_literal(5.3 % 2.1), find_constant_value(&index, "real_mod_real").unwrap());
    // BOOL - BOOL
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "bool_and_bool").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "bool_or_bool").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "bool_xor_bool").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "not_bool").unwrap());
}

#[test]
fn const_references_bool_bit_functions_behavior_evaluation() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = index(
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
    debug_assert_eq!(EMPTY, unresolvable);
    // AND the index should have literal values
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_bool_literal(true), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_bool_literal(false), find_constant_value(&index, "e").unwrap());
}

#[test]
fn const_references_int_bit_functions_behavior_evaluation() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = index(
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
    debug_assert_eq!(EMPTY, unresolvable);
    // AND the index should have literal values
    debug_assert_eq!(&create_int_literal(0xFFAB), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(0x00AB), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(0xFFFF), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_int_literal(0xFF54), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_int_literal(0x0054), find_constant_value(&index, "e").unwrap());
}
#[test]
fn illegal_cast_should_not_be_resolved() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = index(
        "VAR_GLOBAL CONSTANT
            a : INT := BOOL#16#00FF;
        END_VAR
       ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN a could not be resolved, because the literal is invalid
    let expected = UnresolvableConstant::new(global!(index, "a"), "").with_kind(UnresolvableKind::Overflow(
        "This will overflow for type BOOL".into(),
        SourceLocation::undefined(),
    ));
    debug_assert_eq!(expected.id, unresolvable[0].id);
    debug_assert_eq!(expected.get_reason(), unresolvable[0].get_reason());
}

#[test]
fn division_by_0_should_fail() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = index(
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
    // THEN division by 0 are reported - note that division by 0.0 results in infinitya
    debug_assert_eq!(
        vec![
            UnresolvableConstant::new(global!(&index, "a"), "Attempt to divide by zero"),
            UnresolvableConstant::new(global!(&index, "b"), "Attempt to divide by zero"),
            UnresolvableConstant::new(global!(&index, "c"), "Attempt to divide by zero"),
            UnresolvableConstant::new(global!(&index, "d"), "Attempt to divide by zero"),
            UnresolvableConstant::new(
                global!(&index, "aa"),
                "Attempt to calculate the remainder with a divisor of zero"
            ),
            UnresolvableConstant::new(
                global!(&index, "bb"),
                "Attempt to calculate the remainder with a divisor of zero"
            ),
            UnresolvableConstant::new(
                global!(&index, "cc"),
                "Attempt to calculate the remainder with a divisor of zero"
            ),
            UnresolvableConstant::new(
                global!(&index, "dd"),
                "Attempt to calculate the remainder with a divisor of zero"
            ),
        ],
        unresolvable
    );

    // AND the real divisions are unresolved (= Binary Expressions)
    assert!(find_constant_value(&index, "a").unwrap().is_binary_expression());
    assert!(find_constant_value(&index, "b").unwrap().is_binary_expression());
    assert!(find_constant_value(&index, "c").unwrap().is_binary_expression());
    assert!(find_constant_value(&index, "d").unwrap().is_binary_expression());

    assert!(find_constant_value(&index, "aa").unwrap().is_binary_expression());
    assert!(find_constant_value(&index, "bb").unwrap().is_binary_expression());
    assert!(find_constant_value(&index, "cc").unwrap().is_binary_expression());
    assert!(find_constant_value(&index, "dd").unwrap().is_binary_expression());
}

#[test]
fn const_references_not_function_with_signed_ints() {
    // GIVEN some bit-functions used as initializers
    let (_, index) = index(
        "VAR_GLOBAL CONSTANT
            _0x00ff : INT := 16#00FF;   // 255
        END_VAR

        VAR_GLOBAL CONSTANT
            a : INT := INT#16#55;       // 85;
            aa : INT := WORD#16#FFAB;   // 65xxx;
            b : INT := a AND _0x00ff;   // 85
            c : INT := a OR _0x00ff;    // 255
            d : INT := a XOR _0x00ff;   // 170
            e : INT := NOT a;           // -86
        END_VAR
        ",
    );

    // WHEN compile-time evaluation is applied
    let (index, unresolvable) = evaluate_constants(index);

    // THEN everything got resolved
    debug_assert_eq!(EMPTY, unresolvable);
    // AND the index should have literal values
    debug_assert_eq!(&create_int_literal(85), find_constant_value(&index, "a").unwrap());
    debug_assert_eq!(&create_int_literal(0x0000_ffab), find_constant_value(&index, "aa").unwrap());
    debug_assert_eq!(&create_int_literal(85), find_constant_value(&index, "b").unwrap());
    debug_assert_eq!(&create_int_literal(255), find_constant_value(&index, "c").unwrap());
    debug_assert_eq!(&create_int_literal(170), find_constant_value(&index, "d").unwrap());
    debug_assert_eq!(&create_int_literal(-86), find_constant_value(&index, "e").unwrap());
}

#[test]
fn const_references_to_bool_compile_time_evaluation() {
    // GIVEN some BOOL index used as initializers
    let (_, index) = index(
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
    debug_assert_eq!(EMPTY, unresolvable);
    debug_assert_eq!(find_constant_value(&index, "a"), Some(&create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "b"), Some(&create_bool_literal(true)));
    debug_assert_eq!(find_constant_value(&index, "c"), Some(&create_bool_literal(false)));
}

#[test]
fn not_evaluatable_consts_are_reported() {
    // GIVEN some BOOL index used as initializers
    let (_, index) = index(
        "VAR_GLOBAL CONSTANT
            a : INT := 1;
            b : INT := a;
            c : INT;
            d : INT := c;
        END_VAR",
    );

    // WHEN compile-time evaluation is applied
    let (_, unresolvable) = evaluate_constants(index);

    // THEN d can still be evaluated, c = 0
    debug_assert_eq!(unresolvable, vec![] as Vec<UnresolvableConstant>);
}

#[test]
fn evaluating_constants_can_handle_recursion() {
    // GIVEN some BOOL index used as initializers
    let (_, index) = index(
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

    // THEN a,b,c,d could not be resolved (ciruclar dependency)
    debug_assert_eq!(
        vec![
            UnresolvableConstant::incomplete_initialzation(&global!(index, "a")),
            UnresolvableConstant::incomplete_initialzation(&global!(index, "b")),
            UnresolvableConstant::incomplete_initialzation(&global!(index, "c")),
            UnresolvableConstant::incomplete_initialzation(&global!(index, "d")),
        ],
        unresolvable
    );
    // AND aa and bb where resolved correctly
    debug_assert_eq!(find_constant_value(&index, "aa"), Some(&create_int_literal(4)));
    debug_assert_eq!(find_constant_value(&index, "bb"), Some(&create_int_literal(4)));
}

#[test]
fn const_string_initializers_should_be_converted() {
    // GIVEN some STRING constants used as initializers
    let (_, index) = index(
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
    debug_assert_eq!(EMPTY, unresolvable);

    // AND the globals should have gotten their values

    debug_assert_eq!(find_constant_value(&index, "aa"), Some(create_string_literal("World", false)));
    debug_assert_eq!(find_constant_value(&index, "bb"), Some(create_string_literal("Hello", true)));
}

#[test]
fn const_lreal_initializers_should_be_resolved_correctly() {
    // GIVEN some STRING constants used as initializers
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(
        r#"
        VAR_GLOBAL CONSTANT
            clreal : LREAL := 3.1415;
        END_VAR

        VAR_GLOBAL CONSTANT
            tau : LREAL := 2 * clreal;
        END_VAR
        "#,
        id_provider.clone(),
    );

    // WHEN compile-time evaluation is applied
    // AND types are resolved
    let annotations = annotate_with_ids(&parse_result, &mut index, id_provider);
    let (index, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    debug_assert_eq!(EMPTY, unresolvable);

    // AND the globals should have gotten their values
    debug_assert_eq!(find_constant_value(&index, "tau"), Some(create_real_literal("6.283".parse().unwrap())));

    //AND the type is correctly associated
    let i = index.find_global_variable("tau").unwrap().initial_value.unwrap();
    assert_eq!(index.get_const_expressions().find_expression_target_type(&i).unwrap(), "LREAL");

    assert_eq!(
        annotations.get_type(index.get_const_expressions().get_constant_statement(&i).unwrap(), &index),
        index.find_effective_type_by_name("LREAL")
    );
}

#[test]
fn array_size_from_constant() {
    // GIVEN some an array with const-expr -dimensions
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(
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
        id_provider.clone(),
    );

    // WHEN compile-time evaluation is applied
    // AND types are resolved
    annotate_with_ids(&parse_result, &mut index, id_provider);
    let (_, unresolvable) = evaluate_constants(index);

    debug_assert_eq!(EMPTY, unresolvable);
}

#[test]
fn array_literals_type_resolving() {
    // GIVEN some STRING constants used as initializers
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(
        r#"
        VAR_GLOBAL CONSTANT
            a : ARRAY[0..5] OF BYTE := [1,2,3,4];
        END_VAR
       "#,
        id_provider.clone(),
    );

    // WHEN compile-time evaluation is applied
    // AND types are resolved
    let annotations = annotate_with_ids(&parse_result, &mut index, id_provider);
    let (index, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    debug_assert_eq!(EMPTY, unresolvable);

    let a = index.find_global_variable("a").unwrap();
    let i = a.initial_value.unwrap();
    assert_eq!(
        index.get_const_expressions().find_expression_target_type(&i),
        Some(index.find_global_variable("a").unwrap().get_type_name())
    );

    // AND the array-literals types are associated correctly
    if let AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) =
        parse_result.global_vars[0].variables[0].initializer.as_ref().unwrap().get_stmt()
    {
        if let AstStatement::ExpressionList(expressions) = elements.as_ref().get_stmt() {
            for ele in expressions.iter() {
                assert_eq!(annotations.get_type_hint(ele, &index), index.find_effective_type_by_name("BYTE"));
            }
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }

    assert_eq!(
        annotations.get_type_hint(index.get_const_expressions().get_constant_statement(&i).unwrap(), &index),
        index.find_effective_type_by_name(a.get_type_name())
    );
}

#[test]
fn nested_array_literals_type_resolving() {
    // GIVEN a multi-nested Array Type with an initializer
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(
        r#"
        VAR_GLOBAL CONSTANT
            a : ARRAY[0..1] OF ARRAY[0..1] OF BYTE  := [[1,2],[3,4]];
        END_VAR
       "#,
        id_provider.clone(),
    );

    // WHEN compile-time evaluation is applied
    // AND types are resolved
    let annotations = annotate_with_ids(&parse_result, &mut index, id_provider);
    let (index, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    debug_assert_eq!(EMPTY, unresolvable);

    //AND the type is correctly associated
    let a = index.find_global_variable("a").unwrap();
    let i = a.initial_value.unwrap();
    assert_eq!(
        index.get_const_expressions().find_expression_target_type(&i),
        Some(index.find_global_variable("a").unwrap().get_type_name())
    );
    //check the initializer's type
    let initializer = index.get_const_expressions().get_constant_statement(&i).unwrap();
    assert_eq!(
        annotations.get_type_hint(initializer, &index),
        index.find_effective_type_by_name(a.get_type_name())
    );

    //check the initializer's array-element's types
    if let AstStatement::Literal(AstLiteral::Array(Array { elements: Some(e) })) = initializer.get_stmt() {
        if let Some(DataTypeInformation::Array { inner_type_name, .. }) =
            index.find_effective_type_by_name(a.get_type_name()).map(|t| t.get_type_information())
        {
            //check the type of the expression-list has the same type as the variable itself
            /*assert_eq!(
                annotations.get_type_hint(e, &index),
                index.find_type(a.get_type_name())
            );*/

            // check if the array's elements have the array's inner type
            for ele in AstNode::get_as_list(e) {
                let element_hint = annotations.get_type_hint(ele, &index).unwrap();
                assert_eq!(Some(element_hint), index.find_effective_type_by_name(inner_type_name))
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

#[test]
fn nested_array_literals_multiplied_statement_type_resolving() {
    // GIVEN a multi-nested Array Type with an initializer
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(
        r#"
        VAR_GLOBAL CONSTANT
            a : ARRAY[0..1] OF ARRAY[0..1] OF BYTE  := [[2(2)],[2(3)]];
        END_VAR
       "#,
        id_provider.clone(),
    );

    // WHEN compile-time evaluation is applied
    // AND types are resolved
    let annotations = annotate_with_ids(&parse_result, &mut index, id_provider);
    let (index, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    debug_assert_eq!(EMPTY, unresolvable);

    //AND the type is correctly associated
    let a = index.find_global_variable("a").unwrap();
    let i = a.initial_value.unwrap();
    assert_eq!(
        index.get_const_expressions().find_expression_target_type(&i),
        Some(index.find_global_variable("a").unwrap().get_type_name())
    );
    //check the initializer's type
    let initializer = index.get_const_expressions().get_constant_statement(&i).unwrap();

    assert_eq!(
        annotations.get_type_hint(initializer, &index),
        index.find_effective_type_by_name(a.get_type_name())
    );

    //check the initializer's array-element's types
    // [[2(2)],[2(3)]]
    if let AstStatement::Literal(AstLiteral::Array(Array { elements: Some(outer_expression_list) })) =
        initializer.get_stmt()
    {
        // outer_expression_list = [2(2)],[2(3)]
        if let Some(DataTypeInformation::Array { inner_type_name: array_of_byte, .. }) =
            index.find_effective_type_by_name(a.get_type_name()).map(|t| t.get_type_information())
        {
            //check the type of the expression-list has the same type as the variable itself
            assert_eq!(
                annotations.get_type_hint(outer_expression_list, &index),
                index.find_effective_type_by_name(a.get_type_name())
            );

            // check if the array's elements have the array's inner type
            for inner_array in AstNode::get_as_list(outer_expression_list) {
                // [2(2)]
                let element_hint = annotations.get_type_hint(inner_array, &index).unwrap();
                assert_eq!(Some(element_hint), index.find_effective_type_by_name(array_of_byte));

                //check if the inner array statement's also got the type-annotations

                if let AstStatement::Literal(AstLiteral::Array(Array {
                    elements: Some(inner_multiplied_stmt),
                })) = inner_array.get_stmt()
                {
                    // inner_multiplied_stmt = 2(2)
                    for inner_multiplied_stmt in AstNode::get_as_list(inner_multiplied_stmt) {
                        if let AstStatement::MultipliedStatement(data) = inner_multiplied_stmt.get_stmt() {
                            //check if the inner thing really got the BYTE hint
                            // multiplied-element = 2
                            assert_eq!(
                                annotations.get_type_hint(data.element.as_ref(), &index),
                                index.find_effective_type_by_name("BYTE")
                            );
                        } else {
                            unreachable!()
                        }
                    }
                } else {
                    unreachable!()
                }
            }
        } else {
            unreachable!()
        }
    } else {
        unreachable!()
    }
}

#[test]
fn function_block_initializers_constant_resolved_in_assignment() {
    // GIVEN a multi-nested Array Type with an initializer
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(
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
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    // WHEN compile-time evaluation is applied
    // AND types are resolved
    annotate_with_ids(&parse_result, &mut index, id_provider);
    let (_, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    debug_assert_eq!(EMPTY, unresolvable);
}

#[test]
fn contants_in_case_statements_resolved() {
    let id_provider = IdProvider::default();
    let (parse_result, mut index) = index_with_ids(
        "
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
        ",
        id_provider.clone(),
    );

    // WHEN compile-time evaluation is applied
    // AND types are resolved
    annotate_with_ids(&parse_result, &mut index, id_provider);
    let (_, unresolvable) = evaluate_constants(index);

    // THEN all should be resolved
    debug_assert_eq!(EMPTY, unresolvable);

    // DONE in Codegen tests:
    // AND the first case should be 32..60
    // AND the second case should be 62..70
}

#[test]
fn default_values_are_transitive_for_range_types() {
    // GIVEN a range type that inherits the default value from its referenced type
    let src = codegen(
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
fn floating_point_type_casting_of_valid_types_is_resolvable() {
    let (_, index) = index(
        "VAR_GLOBAL CONSTANT
            a : REAL  :=       7 / 2;
            b : REAL  :=  REAL#7 / 2;
            c : REAL  := LREAL#7 / 2;

            d : LREAL :=       7 / 2;
            e : LREAL :=  REAL#7 / 2;
            f : LREAL := LREAL#7 / 2;

            g : LREAL := LREAL#7.0 / 2.0;
            h : LREAL := LREAL#7.0 / 2.0;
        END_VAR
       ",
    );

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 0);
}

#[test]
fn floating_point_type_casting_of_invalid_types_is_unresolvable() {
    let (_, index) = index(
        r#"
        VAR_GLOBAL CONSTANT
            a : STRING := REAL#'abc';
            b : STRING := REAL#"abc";
        END_VAR
       "#,
    );

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 2);
    assert_eq!(
        unresolvable[0].get_reason(),
        Some(r#"Expected floating point type, got: Some(LiteralString { value: "abc", is_wide: false })"#)
    );
    assert_eq!(
        unresolvable[1].get_reason(),
        Some(r#"Expected floating point type, got: Some(LiteralString { value: "abc", is_wide: true })"#)
    );
}

#[test]
fn ref_initializer_is_marked_as_resolve_later() {
    let (_, index) = index(
        r#"
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
       "#,
    );

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 1);
    assert_eq!(unresolvable[0].get_reason(), Some(r#"Try to re-resolve during codegen"#));

    let Some(UnresolvableKind::Address(ref init)) = unresolvable[0].kind else { panic!() };

    assert_eq!(init.scope, Some("foo".into()));
    assert_eq!(init.lhs, Some("ps".into()));
    assert_eq!(init.target_type_name, Some("__foo_ps".to_string()));
}

#[test]
fn adr_initializer_is_marked_as_resolve_later() {
    let (_, index) = index(
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

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 1);
    assert_eq!(unresolvable[0].get_reason(), Some(r#"Try to re-resolve during codegen"#));

    let Some(UnresolvableKind::Address(ref init)) = unresolvable[0].kind else { panic!() };

    assert_eq!(init.scope, Some("foo".into()));
    assert_eq!(init.lhs, Some("ps".into()));
    assert_eq!(init.target_type_name, Some("__foo_ps".to_string()));
}

#[test]
fn alias_initializer_is_marked_as_resolve_later() {
    let (_, index) = index(
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

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 2);
    assert_eq!(unresolvable[0].get_reason(), Some(r#"Try to re-resolve during codegen"#));
    assert_eq!(unresolvable[1].get_reason(), Some(r#"Try to re-resolve during codegen"#));

    let Some(UnresolvableKind::Address(ref init)) = unresolvable[0].kind else { panic!() };

    assert_eq!(init.scope, Some("foo".into()));
    assert_eq!(init.lhs, Some("ps1".into()));
    assert_eq!(init.target_type_name, Some("__foo_ps1".to_string()));

    let Some(UnresolvableKind::Address(ref init)) = unresolvable[1].kind else { panic!() };

    assert_eq!(init.scope, Some("foo".into()));
    assert_eq!(init.lhs, Some("ps2".into()));
    assert_eq!(init.target_type_name, Some("__foo_ps2".to_string()));
}

#[test]
fn reference_to_initializer_is_marked_as_resolve_later() {
    let (_, index) = index(
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

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 1);
    assert_eq!(unresolvable[0].get_reason(), Some(r#"Try to re-resolve during codegen"#));

    let Some(UnresolvableKind::Address(ref init)) = unresolvable[0].kind else { panic!() };

    assert_eq!(init.scope, Some("foo".into()));
    assert_eq!(init.lhs, Some("ps".into()));
    assert_eq!(init.target_type_name, Some("__foo_ps".to_string()));
}

#[test]
fn leading_unary_plus_in_global_const_reference_is_resolvable() {
    let (_, index) = index(
        r#"
        VAR_GLOBAL CONSTANT g1 : INT ; END_VAR

        PROGRAM exp
        VAR
            x : INT := +g1;
        END_VAR
       "#,
    );

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 0);
}
