use crate::ast::SourceRange;
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
            Diagnostic::incompatible_directaccess("Bit", 1, SourceRange::new(223..224,Some(10),Some(21),Some(10),Some(22))),
            Diagnostic::incompatible_directaccess("Bit", 1, SourceRange::new(247..248,Some(11),Some(22),Some(11),Some(23))),
            Diagnostic::incompatible_directaccess("Bit", 1, SourceRange::new(270..271,Some(12),Some(21),Some(12),Some(22))),
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
            Diagnostic::incompatible_directaccess("Byte", 8, SourceRange::new(195..198,Some(9),Some(21),Some(9),Some(24))),
            Diagnostic::incompatible_directaccess("Byte", 8, SourceRange::new(221..224,Some(10),Some(22),Some(10),Some(25))),
            Diagnostic::incompatible_directaccess("Byte", 8, SourceRange::new(247..250,Some(11),Some(22),Some(11),Some(25))),
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
            Diagnostic::incompatible_directaccess("Word", 16, SourceRange::new(194..197,Some(9),Some(21),Some(9),Some(24))),
            Diagnostic::incompatible_directaccess("Word", 16, SourceRange::new(220..223,Some(10),Some(22),Some(10),Some(25))),
            Diagnostic::incompatible_directaccess("Word", 16, SourceRange::new(246..249,Some(11),Some(22),Some(11),Some(25))),
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
            Diagnostic::incompatible_directaccess("DWord", 32, SourceRange::new(197..200,Some(9),Some(21),Some(9),Some(24))),
            Diagnostic::incompatible_directaccess("DWord", 32, SourceRange::new(223..226,Some(10),Some(22),Some(10),Some(25))),
            Diagnostic::incompatible_directaccess("DWord", 32, SourceRange::new(249..252,Some(11),Some(22),Some(11),Some(25))),
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
            Diagnostic::incompatible_directaccess_range("Bit", "BYTE", 0..7, SourceRange::new(138..139,Some(5),Some(19),Some(5),Some(20))),
            Diagnostic::incompatible_directaccess_range("Bit", "WORD", 0..15, SourceRange::new(159..161,Some(6),Some(19),Some(6),Some(22))),
            Diagnostic::incompatible_directaccess_range("Bit", "DWORD", 0..31, SourceRange::new(181..183,Some(7),Some(19),Some(7),Some(22))),
            Diagnostic::incompatible_directaccess_range("Bit", "LWORD", 0..63, SourceRange::new(203..205,Some(8),Some(19),Some(8),Some(22))),
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
            Diagnostic::incompatible_directaccess_range("Byte", "WORD", 0..1, SourceRange::new(128..131,Some(5),Some(19),Some(5),Some(22))),
            Diagnostic::incompatible_directaccess_range("Byte", "DWORD", 0..3, SourceRange::new(151..154,Some(6),Some(19),Some(6),Some(22))),
            Diagnostic::incompatible_directaccess_range("Byte", "LWORD", 0..7, SourceRange::new(174..177,Some(7),Some(19),Some(7),Some(22))),
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
            Diagnostic::incompatible_directaccess_range("Word", "DWORD", 0..1, SourceRange::new(118..121,Some(5),Some(19),Some(5),Some(21))),
            Diagnostic::incompatible_directaccess_range("Word", "LWORD", 0..3, SourceRange::new(141..144,Some(6),Some(19),Some(6),Some(21))),
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
        vec![Diagnostic::incompatible_directaccess_range(
            "DWord",
            "LWORD",
            0..1,
            SourceRange::new(107..110,Some(5),Some(19),Some(5),Some(21))
        ),]
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
            Diagnostic::incompatible_directaccess_variable("LREAL", SourceRange::new(160..163,Some(6),Some(19),Some(6),Some(22))),
            Diagnostic::incompatible_directaccess_variable("REAL", SourceRange::new(183..186,Some(7),Some(19),Some(7),Some(22))),
        ]
    );
}
