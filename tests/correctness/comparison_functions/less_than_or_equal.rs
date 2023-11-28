use driver::runner::{compile_and_run, MainType};

#[test]
fn builtin_le_with_ints_monotonic() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        i1, i2, i3 : DINT;
    END_VAR
        i1 := 1;
        i2 := 1;
        i3 := 1;
        main := LE(i1, i2, i3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, true);
}

#[test]
fn builtin_le_with_ints() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        i1, i2, i3 : DINT;
    END_VAR
        i1 := 3;
        i2 := 2; 
        i3 := 3; // not less or equal to i2, should return false
        main := LE(i1, i2, i3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, false);
}

#[test]
fn builtin_le_with_floats_monotonic() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        r1, r2, r3 : REAL;
    END_VAR
        r1 := 2.9;
        r2 := 2.9;
        r3 := 2.9;
        main := LE(r1, r2, r3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, true);
}

#[test]
fn builtin_le_with_floats() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        r1, r2, r3 : REAL;
    END_VAR
        r1 := 3.0;
        r2 := 2.9; 
        r3 := 3.2; // not less or equal to r3, should return false
        main := LE(r1, r2, r3);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, false);
}

#[test]
fn builtin_le_with_mixed_ints_and_floats() {
    let prog = r#"
    FUNCTION main : BOOL
    VAR
        i1 : DINT;
        r1, r2 : REAL;
    END_VAR
        i1 := 5;
        r1 := 4.5;
        r2 := 3.2;
        main := LE(i1, r1, r2);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: bool = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, false);
}
