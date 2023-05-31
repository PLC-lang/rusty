// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::compile_and_run;

#[allow(dead_code)]
#[repr(C)]
#[derive(PartialEq, Debug)]
struct MainType {
    _byte: i8,
    _sint: i8,
    _usint: u8,
    _word: i16,
    _int: i16,
    _uint: u16,
    _dint: i32,
    _udint: u32,
    _lint: i64,
    _ulint: u64,
}

#[test]
fn sub_range_chooses_right_implementation() {
    let function = r"
        FUNCTION CheckRangeSigned : DINT
            VAR_INPUT v: DINT; low: DINT; up: DINT; END_VAR
            CheckRangeSigned := -7;
        END_FUNCTION

        FUNCTION CheckRangeUnsigned : UDINT
            VAR_INPUT v: UDINT; low: UDINT; up: UDINT; END_VAR
            CheckRangeUnsigned := 7;
        END_FUNCTION

        FUNCTION CheckLRangeSigned : LINT
            VAR_INPUT v: LINT; low: LINT; up: LINT; END_VAR
            CheckLRangeSigned := -77;
        END_FUNCTION

        FUNCTION CheckLRangeUnsigned : ULINT
            VAR_INPUT v: ULINT; low: ULINT; up: ULINT; END_VAR
            CheckLRangeUnsigned := 77;
        END_FUNCTION
        
        PROGRAM main
        VAR
            a   : BYTE(0 .. 100);
            b   : SINT(-100 .. 100);
            c   : USINT(0 .. 100);
            d   : WORD(0 .. 100);
            e   : INT(-100 .. 100);
            f   : UINT(0 .. 100);
            g   : DINT(-100 .. 100);
            h   : UDINT(0 .. 100);
            i   : LINT(-100 .. 100);
            j   : ULINT(0 .. 100);
        END_VAR
        a := 1; b := 1; c := 1; d := 1; e := 1;
        f := 1; g := 1; h := 1; i := 1; j := 1;
        END_PROGRAM
        ";

    let mut maintype = MainType {
        _byte: 0,
        _sint: 0,
        _usint: 0,
        _word: 0,
        _int: 0,
        _uint: 0,
        _dint: 0,
        _udint: 0,
        _lint: 0,
        _ulint: 0,
    };

    let _: i32 = compile_and_run(function, &mut maintype);
    let expected = MainType {
        _byte: 7,
        _sint: -7,
        _usint: 7,
        _word: 7,
        _int: -7,
        _uint: 7,
        _dint: -7,
        _udint: 7,
        _lint: -77,
        _ulint: 77,
    };
    assert_eq!(expected, maintype);
}
