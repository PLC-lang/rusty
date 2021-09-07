// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    ret: i32,
}

#[test]
fn adds_in_result() {
    let prog = "
    FUNCTION main : DINT
        main := 10 + 50;
    END_FUNCTION
    ";

    let (res, _) = compile_and_run(prog.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 60)
}

#[test]
fn int_division_in_result() {
    let prog = "
    FUNCTION main : DINT
        //        int division results in 3 * 100
        main := (10 / 3) * 100;
    END_FUNCTION
    ";

    let (res, _) = compile_and_run(prog.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 300)
}

#[test]
fn real_division_in_result() {
    let prog = "
    FUNCTION main : DINT
        //        real division results in 3.3333.. * 100
        main := (REAL#10 / 3) * 100;
    END_FUNCTION
    ";

    let (res, _) = compile_and_run(prog.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 333)
}

#[test]
fn real_division_by_zero() {
    #[derive(Debug, PartialEq)]
    struct MainType {
        r: f64,
        z: f64,
    }

    let prog = "
    FUNCTION main : DINT
        VAR
            r : LREAL;
            rZero: LREAL;
        END_VAR
        r := (1.0 / rZero);
    END_FUNCTION
    ";

    let mut main = MainType { r: 0.0, z: 0.0 };

    compile_and_run(prog.to_string(), &mut main);
    assert!(main.r.is_infinite());
}
