use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess("Bit", 1, (223..224).into()),
            Diagnostic::incompatible_directaccess("Bit", 1, (247..248).into()),
            Diagnostic::incompatible_directaccess("Bit", 1, (270..271).into()),
        ]
    );
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess("Byte", 8, (195..198).into()),
            Diagnostic::incompatible_directaccess("Byte", 8, (221..224).into()),
            Diagnostic::incompatible_directaccess("Byte", 8, (247..250).into()),
        ]
    );
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess("Word", 16, (194..197).into()),
            Diagnostic::incompatible_directaccess("Word", 16, (220..223).into()),
            Diagnostic::incompatible_directaccess("Word", 16, (246..249).into()),
        ]
    );
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess("DWord", 32, (197..200).into()),
            Diagnostic::incompatible_directaccess("DWord", 32, (223..226).into()),
            Diagnostic::incompatible_directaccess("DWord", 32, (249..252).into()),
        ]
    );
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess_range("Bit", "BYTE", 0..7, (138..139).into()),
            Diagnostic::incompatible_directaccess_range("Bit", "WORD", 0..15, (159..161).into()),
            Diagnostic::incompatible_directaccess_range("Bit", "DWORD", 0..31, (181..183).into()),
            Diagnostic::incompatible_directaccess_range("Bit", "LWORD", 0..63, (203..205).into()),
        ]
    );
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess_range("Byte", "WORD", 0..1, (128..131).into()),
            Diagnostic::incompatible_directaccess_range("Byte", "DWORD", 0..3, (151..154).into()),
            Diagnostic::incompatible_directaccess_range("Byte", "LWORD", 0..7, (174..177).into()),
        ]
    );
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess_range("Word", "DWORD", 0..1, (118..121).into()),
            Diagnostic::incompatible_directaccess_range("Word", "LWORD", 0..3, (141..144).into()),
        ]
    );
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

    assert_eq!(
        diagnostics,
        vec![Diagnostic::incompatible_directaccess_range("DWord", "LWORD", 0..1, (107..110).into()),]
    );
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_directaccess_variable("LREAL", (160..163).into()),
            Diagnostic::incompatible_directaccess_variable("REAL", (183..186).into()),
        ]
    );
}
