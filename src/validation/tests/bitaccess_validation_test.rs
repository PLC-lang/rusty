use insta::assert_snapshot;

use crate::{test_utils::tests::parse_and_validate, validation::tests::make_readable};

#[test]
fn bitaccess_only_on_bit_types() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
            invalid : BOOL;
            invalid2 : STRING;
            valid : BYTE;
            valid2 : USINT;
            valid3 : SINT;
            END_VAR

            invalid.1;
            invalid2.1;
            valid.1.2; (*Invalid*)
            valid.1;
            valid2.1;
            valid3.1;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn byteaccess_only_on_bigger_sizes() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
            invalid : BYTE;
            invalid2 : SINT;
            invalid3 : USINT;
            valid : INT;
            END_VAR

            invalid.%B1;
            invalid2.%B1;
            invalid3.%B1;
            valid.%B1;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn wordaccess_only_on_bigger_sizes() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
            invalid : WORD;
            invalid2 : INT;
            invalid3 : UINT;
            valid : DINT;
            END_VAR

            invalid.%W1;
            invalid2.%W1;
            invalid3.%W1;
            valid.%W1;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn dwordaccess_only_on_bigger_sizes() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
            invalid : DWORD;
            invalid2 : DINT;
            invalid3 : UDINT;
            valid : LINT;
            END_VAR

            invalid.%D1;
            invalid2.%D1;
            invalid3.%D1;
            valid.%D1;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn bitaccess_range_test() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
                a : BYTE; b : WORD; c : DWORD; d : LWORD;
            END_VAR
                a.8;
                b.16;
                c.32;
                d.64;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn byteaccess_range_test() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
                b : WORD; c : DWORD; d : LWORD;
            END_VAR
                b.%B2;
                c.%B4;
                d.%B8;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn wordaccess_range_test() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
                c : DWORD; d : LWORD;
            END_VAR
                c.%W2;
                d.%W4;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn dwordaccess_range_test() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
                d : LWORD;
            END_VAR
                d.%D2;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}

#[test]
fn reference_direct_access_only_with_ints() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
            VAR 
                c : DWORD; d : INT; e : LREAL; f : REAL;
            END_VAR
                c.%Xd;
                c.%Xe;
                c.%Xf;
           END_PROGRAM
       ",
    );

    assert_snapshot!(make_readable(&diagnostics));
}
