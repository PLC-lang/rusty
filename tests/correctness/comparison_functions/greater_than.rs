use driver::runner::{compile_and_run, MainType};

#[test]
fn builtin_gt_with_ints_descending() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        i1, i2, i3 : DINT;
    END_VAR
        i1 := 3;
        i2 := 2;
        i3 := 1;
        main := GT(i1, i2, i3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, true);
}

#[test]
fn builtin_gt_with_ints() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        i1, i2, i3 : DINT;
    END_VAR
        i1 := 3;
        i2 := 2; // not greater than i3, should return false
        i3 := 3; 
        main := GT(i1, i2, i3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, false);
}

#[test]
fn builtin_gt_with_floats_descending() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        r1, r2, r3 : REAL;
    END_VAR
        r1 := 3.0;
        r2 := 2.9;
        r3 := 2.8;
        main := GT(r1, r2, r3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, true);
}

#[test]
fn builtin_gt_with_floats() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        r1, r2, r3 : REAL;
    END_VAR
        r1 := 3.0;
        r2 := 2.9; // not greater than r3, should return false
        r3 := 3.2; 
        main := GT(r1, r2, r3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, false);
}

#[test]
fn builtin_gt_with_mixed_ints_and_floats() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        i1 : DINT;
        r1, r2 : REAL;
    END_VAR
        i1 := 5;
        r1 := 4.5;
        r2 := 3.2;
        main := GT(i1, r1, r2);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, true);
}
