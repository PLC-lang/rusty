// Import common functionality into the integration tests
mod common;

use common::{compile_and_run, get_includes};
use std::fmt::Debug;

#[derive(Default, Debug)]
struct MainType<T>
where
    T: Default + Debug,
{
    a: T,
    b: T,
    c: T,
}

#[test]
fn sqrt_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := SQRT(REAL#4.0);
            b := SQRT(REAL#1.0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a.abs(), 2.0f32);
    assert_eq!(maintype.b.abs(), 1.0f32);
}

#[test]
fn sqrt_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := SQRT(LREAL#4.0);
            b := SQRT(LREAL#1.0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a.abs(), 2.0f64);
    assert_eq!(maintype.b.abs(), 1.0f64);
}

#[test]
#[ignore = "No auto conversion of generic types, we need the conversion function to be done PR#21"]
fn sqrt_called_on_none_real() {
    let src = r"PROGRAM main
            VAR
                a,b : DINT;
            END_VAR
            a := SQRT(DINT_TO_REAL(2));
            b := SQRT(DINT_TO_REAL(1));
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st", "num.st"]);
    let mut maintype = MainType::<i32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a.abs(), 2);
    assert_eq!(maintype.b.abs(), 1);
}

#[test]
fn ln_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := LN(E_REAL) - 1.0;
            b := LN(REAL#1.0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
    assert!(maintype.b.abs() <= f32::EPSILON);
}

#[test]
fn ln_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := LN(E_LREAL) - 1.0;
            b := LN(LREAL#1.0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
    assert!(maintype.b.abs() <= f64::EPSILON);
}

#[test]
fn log_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
                a := LOG(REAL#10);
                b := LOG(REAL#100);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a.abs(), 1.0f32);
    assert_eq!(maintype.b.abs(), 2.0f32);
}

#[test]
fn log_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
                a := LOG(LREAL#10);
                b := LOG(LREAL#100);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a.abs(), 1.0f64);
    assert_eq!(maintype.b.abs(), 2.0f64);
}

#[test]
fn exp_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := EXP(REAL#1.0) - E_REAL;
            b := EXP(REAL#0.0) - REAL#1.0;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
    assert!(maintype.b.abs() <= f32::EPSILON);
}

#[test]
fn exp_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := EXP(LREAL#1.0) - E_LREAL;
            b := EXP(LREAL#0.0) - LREAL#1.0;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
    assert!(maintype.b.abs() <= f64::EPSILON);
}

#[test]
fn sin_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := SIN(FRAC_PI_2_REAL) - REAL#1.0;
            b := SIN(REAL#0.0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
    assert!(maintype.b.abs() <= f32::EPSILON);
}

#[test]
fn sin_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := SIN(FRAC_PI_2_LREAL) - LREAL#1.0;
            b := SIN(LREAL#0.0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
    assert!(maintype.b.abs() <= f64::EPSILON);
}

#[test]
fn cos_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := COS(PI_REAL) + REAL#1.0;
            b := COS(REAL#0.0) - REAL#1.0;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
    assert!(maintype.b.abs() <= f32::EPSILON);
}

#[test]
fn cos_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := COS(PI_LREAL) + LREAL#1.0;
            b := COS(LREAL#0.0) - LREAL#1.0;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
    assert!(maintype.b.abs() <= f64::EPSILON);
}

#[test]
fn tan_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := TAN(FRAC_PI_4_REAL) - REAL#1.0;
            b := TAN(PI_REAL);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
    assert!(maintype.b.abs() <= f32::EPSILON);
}

#[test]
fn tan_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := TAN(FRAC_PI_4_LREAL) - LREAL#1.0;
            a := TAN(PI_LREAL);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
    assert!(maintype.b.abs() <= f64::EPSILON);
}

#[test]
fn asin_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := ASIN(REAL#1.0) - FRAC_PI_2_REAL;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
}

#[test]
fn asin_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := ASIN(LREAL#1.0) - FRAC_PI_2_LREAL;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
}

#[test]
fn acos_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := ACOS(REAL#0.0) - FRAC_PI_2_REAL;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
}

#[test]
fn acos_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := ACOS(LREAL#0.0) - FRAC_PI_2_LREAL;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
}

#[test]
fn atan_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := ATAN(REAL#1.0) - FRAC_PI_4_REAL;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
}

#[test]
fn atan_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := ATAN(LREAL#1.0) - FRAC_PI_4_LREAL;
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
}

#[test]
fn atan2_called_on_real() {
    let src = r"PROGRAM main
            VAR
                a,b : REAL;
            END_VAR
            a := ATAN2(REAL#-3.0, REAL#3.0) + FRAC_PI_4_REAL;
            b := ATAN2(REAL#3.0, REAL#-3.0) - (REAL#3.0 * FRAC_PI_4_REAL);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f32::EPSILON);
    assert!(maintype.b.abs() <= f32::EPSILON);
}

#[test]
fn atan2_called_on_lreal() {
    let src = r"PROGRAM main
            VAR
                a,b : LREAL;
            END_VAR
            a := ATAN2(LREAL#-3.0, LREAL#3.0) + FRAC_PI_4_LREAL;
            b := ATAN2(LREAL#3.0, LREAL#-3.0) - (REAL#3.0 * FRAC_PI_4_LREAL);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!(maintype.a.abs() <= f64::EPSILON);
    assert!(maintype.b.abs() <= f64::EPSILON);
}

#[test]
fn expt_called_on_real() {
    let src = r#"
        PROGRAM main
        VAR
            a: REAL;
        END_VAR
            a := EXPT(REAL#2.0, REAL#7.0);
        END_PROGRAM
    "#;
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!((maintype.a - 128.0) <= f32::EPSILON);
}

#[test]
fn expt_called_on_dint_literal() {
    let src = r#"
        PROGRAM main
        VAR
            a: DINT;
            b: DINT;
        END_VAR
            a := EXPT(2, 2);
            b := EXPT(DINT#2, 7);
        END_PROGRAM
    "#;
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<i32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(4, maintype.a);
    assert_eq!(128, maintype.b);
}

#[test]
fn expt_called_on_lint_lreal() {
    let src = r#"
        PROGRAM main
        VAR
            a: LREAL;
            b: LINT;
        END_VAR
            a := EXPT(DINT#2, DINT#-2);
            b := EXPT(LINT#2, LREAL#7.0);
        END_PROGRAM
    "#;
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(0.25_f64, maintype.a);
    assert!(maintype.b - 128_f64 <= f64::EPSILON);
}

#[test]
fn expt_called_with_operator() {
    let src = r#"
        PROGRAM main
        VAR
            a: REAL;
        END_VAR
            a := 2**7;
        END_PROGRAM
    "#;

    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert!((maintype.a - 128.0) <= f32::EPSILON);
}

#[test]
fn expt_called_with_references() {
    let src = r#"
        PROGRAM main
        VAR
            a: REAL;
            b: REAL;
            c: REAL;
        END_VAR
        VAR_TEMP
            x: REAL := 2.0;
            y: SINT := -2;
            z: UDINT := 2;
        END_VAR
            a := x**y;
            b := x**z;
            c := z**x;
        END_PROGRAM
    "#;
    let sources = vec![src.into()];
    let includes = get_includes(&["arithmetic_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(0.25, maintype.a);
    assert_eq!(4.0, maintype.b);
    assert_eq!(4.0, maintype.c);
}
