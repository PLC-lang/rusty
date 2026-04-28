// Import common functionality into the integration tests
mod common;

use common::compile_and_run;

use crate::common::get_includes;

#[derive(Default)]
struct MainType<T: Default> {
    a: T,
    b: T,
    c: T,
    res: T,
}

#[test]
fn absolute_on_sint_test() {
    let src = r"PROGRAM main
            VAR
                a,b,c,res : SINT;
            END_VAR
            a := ABS(SINT#-120);
            b := ABS(SINT#-0);
            c := ABS(SINT#121);
            res := ABS(SINT#0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let mut maintype = MainType::<i8>::default();
    let _: i8 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.res, 0i8);
    assert_eq!(maintype.a, 120i8);
    assert_eq!(maintype.b, 0i8);
    assert_eq!(maintype.c, 121i8);
}

#[test]
fn absolute_on_int_test() {
    let src = r"PROGRAM main
            VAR
                a,b,c,res : INT;
            END_VAR
            a := ABS(INT#-99);
            b := ABS(INT#-0);
            c := ABS(INT#98);
            res := ABS(INT#0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let mut maintype = MainType::<i16>::default();
    let _: i16 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.res, 0i16);
    assert_eq!(maintype.a, 99i16);
    assert_eq!(maintype.b, 0i16);
    assert_eq!(maintype.c, 98i16);
}

#[test]
fn absolute_on_dint_test() {
    let src = r"PROGRAM main
            VAR
                a,b,c,res : DINT;
            END_VAR
            a := ABS(-77);
            b := ABS(-0);
            c := ABS(78);
            res := ABS(0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let mut maintype = MainType::<i32>::default();
    let _: i32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.res, 0i32);
    assert_eq!(maintype.a, 77i32);
    assert_eq!(maintype.b, 0i32);
    assert_eq!(maintype.c, 78i32);
}

#[test]
fn absolute_on_lint_test() {
    let src = r"PROGRAM main
            VAR
                a,b,c,res : LINT;
            END_VAR
            a := ABS(LINT#-88);
            b := ABS(LINT#-0);
            c := ABS(LINT#89);
            res := ABS(LINT#0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.res, 0i64);
    assert_eq!(maintype.a, 88i64);
    assert_eq!(maintype.b, 0i64);
    assert_eq!(maintype.c, 89i64);
}

#[test]
fn absolute_on_real_test() {
    let src = r"PROGRAM main
            VAR
                a,b,c,res : REAL;
            END_VAR
            a := ABS(REAL#-3.2);
            b := ABS(REAL#-0);
            c := ABS(REAL#3.3);
            res := ABS(REAL#0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let mut maintype = MainType::<f32>::default();
    let _: f32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.res, 0.0f32);
    assert_eq!(maintype.a, 3.2f32);
    assert_eq!(maintype.b, 0.0f32);
    assert_eq!(maintype.c, 3.3f32);
}

#[test]
fn absolute_on_lreal_test() {
    let src = r"PROGRAM main
            VAR
                a,b,c,res : LREAL;
            END_VAR
            a := ABS(LREAL#-2.5);
            b := ABS(LREAL#-0);
            c := ABS(LREAL#2.6);
            res := ABS(LREAL#0);
            END_PROGRAM
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let mut maintype = MainType::<f64>::default();
    let _: f64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.res, 0.0f64);
    assert_eq!(maintype.a, 2.5f64);
    assert_eq!(maintype.b, 0.0f64);
    assert_eq!(maintype.c, 2.6f64);
}

#[test]
fn test_round_real() {
    let src = r"
        FUNCTION main : REAL
            main := ROUND(REAL#2.5);
        END_FUNCTION
        ";
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let mut maintype = plc_driver::runner::MainType::default();
    let res: f32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(res, 3.0f32);
}

#[test]
fn test_round_lreal() {
    let src = r"
        FUNCTION main : LREAL
            main := ROUND(LREAL#2.5);
        END_FUNCTION
        ";
    let mut maintype = plc_driver::runner::MainType::default();
    let sources = vec![src.into()];
    let includes = get_includes(&["numerical_functions.st"]);
    let res: f64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(res, 3.0f64);
}
