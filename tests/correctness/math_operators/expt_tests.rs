use rusty::runner::{MainType, compile_and_run};

use crate::assert_almost_eq;

#[test]
fn int_to_int_expt() {
    let prog = "
    FUNCTION main : DINT
        main := 2**3;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 8)

}

#[test]
fn int_to_real_expt() {
    let prog = "
    FUNCTION main : REAL
        main := 2**REAL#0.5;
    END_FUNCTION
    ";

    let res: f32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_almost_eq!(res,2.0f32.sqrt(), f32::EPSILON);
}

#[test]
fn real_to_real_expt() {
    let prog = "
    FUNCTION main : REAL
        main := REAL#2**REAL#0.1;
    END_FUNCTION
    ";

    let res: f32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_almost_eq!(res,2.0f32.powf(0.1), f32::EPSILON);
}

#[test]
fn real_to_int_expt() {
    let prog = "
    FUNCTION main : REAL
        main := REAL#3.0**2;
    END_FUNCTION
    ";

    let res: f32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_almost_eq!(res,9.0f32, f32::EPSILON);
}

#[test]
fn lreal_to_real_expt() {
    let prog = "
    FUNCTION main : LREAL
        main := LREAL#3**REAL#0.2;
    END_FUNCTION
    ";

    let res: f64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_almost_eq!(res,3.0f64.powf(0.2), f64::EPSILON);

}

#[test]
fn real_to_lreal_expt() {
    let prog = "
    FUNCTION main : LREAL
        main := REAL#4**LREAL#0.3;
    END_FUNCTION
    ";

    let res: f64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_almost_eq!(res,4.0f64.powf(0.3), f64::EPSILON);
}

#[test]
fn lreal_to_lreal_expt() {
    let prog = "
    FUNCTION main : LREAL
        main := LREAL#3**LREAL#0.2;
    END_FUNCTION
    ";

    let res: f64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_almost_eq!(res,5.0f64.powf(0.4), f64::EPSILON);
}
