// Import common functionality into the integration tests
mod common;

use plc::codegen::CodegenContext;
use plc_source::Compilable;

use crate::common::compile_and_load as compile;
use crate::common::compile_and_run_no_params;
use crate::common::get_includes;

#[test]
fn test_mux() {
    let src = r"FUNCTION main : DINT
    VAR
        a,b,c,d : DINT;
    END_VAR
    a := 1;
    b := 2;
    c := 3;
    main := MUX(2,a,b,c);
    END_FUNCTION";

    let res: i32 = compile_and_run_no_params(src.containers(), vec![]);
    assert_eq!(res, 3);
}

#[test]
fn test_sel() {
    let src = r"FUNCTION main : DINT
    VAR
        a,b,c : DINT;
    END_VAR
    a := 1;
    b := 2;
    main := SEL(FALSE,a,b);
    END_FUNCTION";

    let res: i32 = compile_and_run_no_params(src.containers(), vec![]);
    assert_eq!(res, 1);
}

#[test]
fn test_move() {
    let src = r"FUNCTION main : DINT
    VAR
        a : DINT;
    END_VAR
    a := 1;
    main := MOVE(a);
    END_FUNCTION";

    let res: i32 = compile_and_run_no_params(src.containers(), vec![]);
    assert_eq!(res, 1);
}

#[test]
fn test_max_int() {
    let src = r"FUNCTION main : INT
    main := MAX(INT#5,INT#2,INT#1,INT#3,INT#4,INT#7,INT#-1);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i16 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, 7);
}

#[test]
fn test_max_dint() {
    let src = r"FUNCTION main : DINT
    main := MAX(DINT#5,DINT#2,DINT#1,DINT#3,DINT#4,DINT#7,DINT#-1);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i32 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, 7);
}

#[test]
fn test_max_lint() {
    let src = r"FUNCTION main : LINT
    main := MAX(LINT#5,LINT#2,LINT#1,LINT#3,LINT#4,LINT#7,LINT#-1);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, 7);
}

#[test]
fn test_max_char() {
    let src = r"FUNCTION main : CHAR
    main := MAX(CHAR#'a',CHAR#'d',CHAR#'e',CHAR#'g',CHAR#'f',CHAR#'c',CHAR#'b');
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: u8 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, b'g');
}

#[test]
fn test_max_date() {
    let src = r"FUNCTION main : TIME
    main := MAX(T#35ms, T#40ms,T#1ms,T#30ms);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, 40_000_000);
}

#[test]
fn test_max_real() {
    let src = r"FUNCTION main : REAL
    main := MAX(0.5, 0.1,1.5,1.2);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: f32 = compile_and_run_no_params(vec![src.into()], includes);
    assert!((res - 1.5_f32).abs() < f32::EPSILON);
}

#[test]
fn test_max_lreal() {
    let src = r"FUNCTION main : LREAL
    main := MAX(LREAL#0.5, 0.1, 1.5, 1.2);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: f64 = compile_and_run_no_params(vec![src.into()], includes);
    assert!((res - 1.5_f64).abs() < f64::EPSILON);
}

#[test]
fn test_min_int() {
    let src = r"FUNCTION main : INT
    main := MIN(INT#5,INT#2,INT#-1,INT#3,INT#4,INT#7,INT#1);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i16 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, -1);
}

#[test]
fn test_min_dint() {
    let src = r"FUNCTION main : DINT
    main := MIN(DINT#5,DINT#2,DINT#-1,DINT#3,DINT#4,DINT#7,DINT#1);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i32 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, -1);
}

#[test]
fn test_min_lint() {
    let src = r"FUNCTION main : LINT
    main := MIN(LINT#5,LINT#2,LINT#-1,LINT#3,LINT#4,LINT#7,LINT#1);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, -1);
}

#[test]
fn test_min_char() {
    let src = r"FUNCTION main : CHAR
    main := MIN(CHAR#'b',CHAR#'d',CHAR#'e',CHAR#'g',CHAR#'f',CHAR#'a',CHAR#'c');
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: u8 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, b'a');
}

#[test]
fn test_min_date() {
    let src = r"FUNCTION main : TIME
    main := MIN(T#40ms,T#1d,T#30ms,T#5m);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(res, 30_000_000);
}

#[test]
fn test_min_real() {
    let src = r"FUNCTION main : REAL
    main := MIN(0.5,0.1,1.5,1.2);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: f32 = compile_and_run_no_params(vec![src.into()], includes);
    assert!((res - 0.1_f32).abs() <= f32::EPSILON);
}

#[test]
fn test_min_lreal() {
    let src = r"FUNCTION main : LREAL
    main := MIN(LREAL#0.5, LREAL#0.1,LREAL#1.5,LREAL#1.2);
    END_FUNCTION";

    let includes = get_includes(&["selectors.st"]);
    let res: f64 = compile_and_run_no_params(vec![src.into()], includes);
    assert!((res - 0.1_f64).abs() < f64::EPSILON);
}

#[test]
fn test_limit_int() {
    let src = r#"
        FUNCTION main : INT
        VAR_INPUT {ref}
            in : INT;
        END_VAR
        main := LIMIT(INT#10,in,INT#50);
        END_FUNCTION
    "#;

    let includes = get_includes(&["selectors.st"]);
    let context = CodegenContext::create();
    let module = compile(&context, vec![src.into()], includes);

    //In range No Actions
    let res: i16 = module.run("main", &mut 30i16);
    assert_eq!(30, res);

    //below range, min returned
    let res: i16 = module.run("main", &mut 1i16);
    assert_eq!(10, res);

    //above range, max returned
    let res: i16 = module.run("main", &mut 60i16);
    assert_eq!(50, res);
}

#[test]
fn test_limit_dint() {
    let src = r#"
        FUNCTION main : DINT
        VAR_INPUT {ref}
            in : DINT;
        END_VAR
        main := LIMIT(10,in,50);
        END_FUNCTION
    "#;

    let includes = get_includes(&["selectors.st"]);
    let context = CodegenContext::create();
    let module = compile(&context, vec![src.into()], includes);

    //In range No Actions
    let res: i32 = module.run("main", &mut 30i32);
    assert_eq!(30, res);

    //below range, min returned
    let res: i32 = module.run("main", &mut 1i32);
    assert_eq!(10, res);

    //above range, max returned
    let res: i32 = module.run("main", &mut 60i32);
    assert_eq!(50, res);
}

#[test]
fn test_limit_lint() {
    let src = r#"
        FUNCTION main : LINT
        VAR_INPUT {ref}
            in : LINT;
        END_VAR
        main := LIMIT(10,in,50);
        END_FUNCTION
    "#;

    let includes = get_includes(&["selectors.st"]);
    let context = CodegenContext::create();
    let module = compile(&context, vec![src.into()], includes);

    //In range No Actions
    let res: i64 = module.run("main", &mut 30i64);
    assert_eq!(30, res);

    //below range, min returned
    let res: i64 = module.run("main", &mut 1i64);
    assert_eq!(10, res);

    //above range, max returned
    let res: i64 = module.run("main", &mut 60i64);
    assert_eq!(50, res);
}

#[test]
fn test_limit_char() {
    let src = r#"
        FUNCTION main : CHAR
        VAR_INPUT {ref}
            in : CHAR;
        END_VAR
        main := LIMIT(CHAR#'b',in,CHAR#'d');
        END_FUNCTION
    "#;

    let includes = get_includes(&["selectors.st"]);
    let context = CodegenContext::create();
    let module = compile(&context, vec![src.into()], includes);

    //In range No Actions
    let res: u8 = module.run("main", &mut b'c');
    assert_eq!(b'c', res);

    //below range, min returned
    let res: u8 = module.run("main", &mut b'a');
    assert_eq!(b'b', res);

    //above range, max returned
    let res: u8 = module.run("main", &mut b'f');
    assert_eq!(b'd', res);
}

#[test]
fn test_limit_real() {
    let src = r#"
        FUNCTION main : REAL
        VAR_INPUT {ref}
            in : REAL;
        END_VAR
        main := LIMIT(10,in,50);
        END_FUNCTION
    "#;

    let includes = get_includes(&["selectors.st"]);
    let context = CodegenContext::create();
    let module = compile(&context, vec![src.into()], includes);

    //In range No Actions
    let res: f32 = module.run("main", &mut 10.5f32);
    assert!((res - 10.5f32).abs() <= f32::EPSILON);

    //below range, min returned
    let res: f32 = module.run("main", &mut -1f32);
    assert!((res - 10f32).abs() <= f32::EPSILON);

    //above range, max returned
    let res: f32 = module.run("main", &mut 60f32);
    assert!((res - 50f32).abs() <= f32::EPSILON);
}

#[test]
fn test_limit_lreal() {
    let src = r#"
        FUNCTION main : LREAL
        VAR_INPUT {ref}
            in : LREAL;
        END_VAR
        main := LIMIT(10,in,50);
        END_FUNCTION
    "#;

    let includes = get_includes(&["selectors.st"]);
    let context = CodegenContext::create();
    let module = compile(&context, vec![src.into()], includes);

    //In range No Actions
    let res: f64 = module.run("main", &mut 10.5f64);
    assert!((res - 10.5f64).abs() <= f64::EPSILON);

    //below range, min returned
    let res: f64 = module.run("main", &mut -1f64);
    assert!((res - 10f64).abs() <= f64::EPSILON);

    //above range, max returned
    let res: f64 = module.run("main", &mut 60f64);
    assert!((res - 50f64).abs() <= f64::EPSILON);
}
