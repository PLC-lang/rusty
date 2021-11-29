// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[derive(Default)]
#[repr(C)]
struct MainType {
    a: f32,
    b: f32,
    c: f64,
    d: f64,
}

#[test]
fn real_negation() {
    let function = "
            FUNCTION main : DINT
            VAR
                a,b : REAL;
                c,d : LREAL;
            END_VAR
                a := -2.0;
                b := -a;
                c := -3.0;
                d := -c;
            END_FUNCTION
    ";
    let mut maintype = MainType::default();
    let _: i32 = compile_and_run(function, &mut maintype);
    assert_eq!(-2.0, maintype.a);
    assert_eq!(2.0, maintype.b);
    assert_eq!(-3.0, maintype.c);
    assert_eq!(3.0, maintype.d);
}

#[test]
fn equal_comparison_with_arbitrary_datatypes() {
    struct Main {
        results: [i32; 2],
    }

    let mut main = Main { results: [0, 0] };

    let function = "
            FUNCTION STRING_EQUAL : BOOL
                VAR_INPUT a,b : STRING; END_VAR
                STRING_EQUAL := TRUE;
            END_FUNCTION

            FUNCTION main : DINT
            VAR_INPUT
                result1 : DINT;
                result2 : DINT;
            END_VAR
            VAR_TEMP
                a,b : STRING[1];
            END_VAR

            IF (a = b) THEN
                result1 := 1;
            ELSE
                result1 := 0;
            END_IF

            IF (a <> b) THEN
                result2 := 1;
            ELSE
                result2 := -1;
            END_IF

           END_FUNCTION
    ";
    let _: i32 = compile_and_run(function, &mut main);
    assert_eq!([1, -1], main.results);
}

#[test]
fn less_or_equal_comparison_with_arbitrary_datatypes() {
    struct Main {
        results: [i32; 3],
    }

    let mut main = Main { results: [0, 0, 0] };

    let function = "
            FUNCTION STRING_EQUAL : BOOL
                VAR_INPUT a,b : STRING; END_VAR
                STRING_EQUAL := FALSE;
            END_FUNCTION

            FUNCTION STRING_LESS : BOOL
                VAR_INPUT a,b : STRING; END_VAR
                STRING_LESS := TRUE;
            END_FUNCTION

            FUNCTION main : DINT
            VAR_INPUT
                result1 : DINT;
                result2 : DINT;
                result3 : DINT;
            END_VAR
            VAR_TEMP
                a,b : STRING[1];
            END_VAR

            IF (a = b) THEN
                result1 := 1;
            ELSE
                result1 := -1;
            END_IF

            IF (a < b) THEN
                result2 := 1;
            ELSE
                result2 := -1;
            END_IF

            IF (a <= b) THEN
                result3 := 1;
            ELSE
                result3 := -1;
            END_IF

           END_FUNCTION
    ";
    let _: i32 = compile_and_run(function, &mut main);
    assert_eq!([-1, 1, 1], main.results);
}

#[test]
fn greater_or_equal_comparison_with_arbitrary_datatypes() {
    struct Main {
        results: [i32; 3],
    }

    let mut main = Main { results: [0, 0, 0] };

    let function = "
            FUNCTION STRING_EQUAL : BOOL
                VAR_INPUT a,b : STRING; END_VAR
                STRING_EQUAL := FALSE;
            END_FUNCTION

            FUNCTION STRING_GREATER : BOOL
                VAR_INPUT a,b : STRING; END_VAR
                STRING_GREATER := TRUE;
            END_FUNCTION

            FUNCTION main : DINT
            VAR_INPUT
                result1 : DINT;
                result2 : DINT;
                result3 : DINT;
            END_VAR
            VAR_TEMP
                a,b : STRING[1];
            END_VAR

            IF (a = b) THEN
                result1 := 1;
            ELSE
                result1 := -1;
            END_IF

            IF (a > b) THEN
                result2 := 1;
            ELSE
                result2 := -1;
            END_IF

            IF (a >= b) THEN
                result3 := 1;
            ELSE
                result3 := -1;
            END_IF

           END_FUNCTION
    ";
    let _: i32 = compile_and_run(function, &mut main);
    assert_eq!([-1, 1, 1], main.results);
}
