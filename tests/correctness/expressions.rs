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

#[test]
fn enums_can_be_compared() {
    struct Main {
        a: bool,
        b: bool,
        c: bool,
    }

    let mut main = Main {
        a: false,
        b: false,
        c: false,
    };

    let function = "
        TYPE MyEnum : BYTE (zero, aa, bb := 7, cc); END_TYPE

        FUNCTION main : DINT 
            VAR a,b,c : BOOL; END_VAR

            VAR_TEMP
                x : MyEnum := 1;
                y : MyEnum := bb;
                z : MyEnum := cc;
            END_VAR

            IF x = aa THEN
                a := TRUE;
            END_IF

            IF y = 7 THEN
                b := TRUE;
            END_IF
            
            IF z = 8 THEN
                c := TRUE;
            END_IF

        END_FUNCTION 
    ";
    let _: i32 = compile_and_run(function, &mut main);
    assert_eq!([true, true, true], [main.a, main.b, main.c]);
}

#[test]
fn binary_expressions_for_pointers() {
    #[derive(Default)]
    struct Main {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        equal: bool,
        not_equal: bool,
        less: bool,
        greater: bool,
        less_or_equal: bool,
        greater_or_equal: bool,
    }

    let function = "
	PROGRAM main
	VAR
		a : CHAR;
		b : CHAR;
		c : CHAR;
		d : CHAR;
		e : CHAR;
		equal : BOOL;
		not_equal : BOOL;
		less : BOOL;
		greater : BOOL;
		less_or_equal : BOOL;
		greater_or_equal : BOOL;
	END_VAR
	VAR_TEMP
		arr : ARRAY[0..3] OF CHAR := ['a','b','c','d'];
		ptr : REF_TO CHAR;
		negative : INT := -1;
	END_VAR
		ptr := &(arr);

		ptr := ptr + 2;
		a := ptr^;
		ptr := ptr + 1;
		b := ptr^;
		ptr := ptr - 1;
		c := ptr^;
		ptr := ptr + negative;
		d := ptr^;
		ptr := ptr - negative;
		e := ptr^;

		equal := ptr = ptr;
		not_equal := ptr <> ptr;
		less := ptr < ptr;
		greater := ptr > ptr;
		less_or_equal := ptr <= ptr;
		greater_or_equal := ptr >= ptr;
	END_PROGRAM
	";
    let mut main = Main::default();
    let _: i32 = compile_and_run(function, &mut main);
    assert_eq!(main.a, "c".as_bytes()[0]);
    assert_eq!(main.b, "d".as_bytes()[0]);
    assert_eq!(main.c, "c".as_bytes()[0]);
    assert_eq!(main.d, "b".as_bytes()[0]);
    assert_eq!(main.e, "c".as_bytes()[0]);
    assert_eq!(main.equal, true);
    assert_eq!(main.not_equal, false);
    assert_eq!(main.less, false);
    assert_eq!(main.greater, false);
    assert_eq!(main.less_or_equal, true);
    assert_eq!(main.greater_or_equal, true);
}
