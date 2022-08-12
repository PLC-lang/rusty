use crate::{
    test_utils::tests::parse_and_validate,
    typesystem::{DATE_AND_TIME_TYPE, DATE_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE},
    Diagnostic, ast::SourceRange,
};

#[test]
fn int_literal_casts_max_values_are_validated() {
    let diagnostics = parse_and_validate(
        "
            PROGRAM prg
                BYTE#255;
                BYTE#256;

                UINT#65_535;
                UINT#65_536;

                UDINT#4_294_967_295;
                UDINT#4_294_967_296;

                //ULINT#16#FFFF_FFFF_FFFF_FFFF;
                //ULINT#16#1_0000_0000_0000_0000;
           END_PROGRAM
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::literal_out_of_range("256", "BYTE", SourceRange::new(67..75,Some(3),Some(17),Some(3),Some(25))),
            Diagnostic::literal_out_of_range("65536", "UINT", SourceRange::new(123..134,Some(6),Some(17),Some(6),Some(28))),
            Diagnostic::literal_out_of_range("4294967296", "UDINT", SourceRange::new(190..209,Some(9),Some(17),Some(9),Some(46))),
        ]
    );
}

#[test]
fn bool_literal_casts_are_validated() {
    let diagnostics = parse_and_validate(
        "
        PROGRAM prg
            BOOL#TRUE;
            BOOL#FALSE;
            BOOL#0;
            BOOL#1;
            BOOL#2;
            BOOL#2.3;
            BOOL#'abc';
        END_PROGRAM
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::literal_out_of_range("2", "BOOL", SourceRange::new(120..126,Some(6),Some(13),Some(6),Some(19))),
            Diagnostic::incompatible_literal_cast("BOOL", "2.3", SourceRange::new(140..148,Some(7),Some(13),Some(7),Some(21))),
            Diagnostic::incompatible_literal_cast("BOOL", "'abc'", SourceRange::new(162..172,Some(8),Some(13),Some(8),Some(2))),
        ]
    );
}

#[test]
fn string_literal_casts_are_validated() {
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM prg
            
            STRING#"TRUE";
            WSTRING#'TRUE';

            STRING#TRUE;
            WSTRING#FALSE;

            STRING#22;
            WSTRING#33;

            STRING#3.14;
            WSTRING#1.0;


        END_PROGRAM
       "#,
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_literal_cast("STRING", "true", SourceRange::new(102..113,Some(6),Some(13),Some(6),Some(24))),
            Diagnostic::incompatible_literal_cast("WSTRING", "false", SourceRange::new(127..140,Some(7),Some(13),Some(7),Some(26))),
            Diagnostic::incompatible_literal_cast("STRING", "22", SourceRange::new(155..164,Some(9),Some(13),Some(9),Some(22))),
            Diagnostic::incompatible_literal_cast("WSTRING", "33", SourceRange::new(178..188,Some(10),Some(13),Some(10),Some(23))),
            Diagnostic::incompatible_literal_cast("STRING", "3.14", SourceRange::new(203..214,Some(12),Some(13),Some(12),Some(24))),
            Diagnostic::incompatible_literal_cast("WSTRING", "1.0", SourceRange::new(228..239,Some(13),Some(13),Some(13),Some(224))),
        ]
    );
}

#[test]
fn real_literal_casts_are_validated() {
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM prg
            
            REAL#3.14;
            LREAL#3.15;
            REAL#10;
            LREAL#-3;

            REAL#TRUE;
            REAL#1;
            REAL#'3.14';
 
            LREAL#TRUE;
            LREAL#1;
            LREAL#"3.14";
        END_PROGRAM
       "#,
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_literal_cast("REAL", "'3.14'", SourceRange::new(180..191,Some(10),Some(13),Some(10),Some(24))),
            Diagnostic::incompatible_literal_cast("LREAL", r#""3.14""#, SourceRange::new(252..264,Some(14),Some(13),Some(14),Some(25)))
        ]
    );
}

#[test]
fn literal_cast_with_non_literal() {
    let diagnostics = parse_and_validate(
        "PROGRAM exp 
            INT#[x]; 
        END_PROGRAM",
    );
    assert_eq!(
        vec![Diagnostic::literal_expected(SourceRange::new(25..32,Some(2),Some(13),Some(2),Some(20)))],
        diagnostics
    );
}

#[test]
fn literal_enum_elements_validate_without_errors() {
    let diagnostics = parse_and_validate(
        "
        TYPE Animal: (Dog, Cat, Horse); END_TYPE
        TYPE Color: (Red, Yellow, Green); END_TYPE
        
        PROGRAM exp 
            Animal#Dog; 
            Color#Yellow; 
        END_PROGRAM",
    );

    let empty: Vec<Diagnostic> = Vec::new();
    assert_eq!(empty, diagnostics);
}

#[test]
fn date_literal_casts_are_validated() {
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM prg
            LINT#DT#1989-06-15-13:56:14.77;
            LINT#TIME_OF_DAY#15:36:30.123;
            LINT#T#12h34m15s;
            LINT#DATE#1996-05-06;

            ULINT#DT#1989-06-15-13:56:14.77;
            ULINT#TIME_OF_DAY#15:36:30.123;
            ULINT#T#12h34m15s;
            ULINT#DATE#1996-05-06;

            INT#DT#1989-06-15-13:56:14.77;
            INT#TIME_OF_DAY#15:36:30.123;
            INT#T#12h34m15s;
            INT#DATE#1996-05-06;
        END_PROGRAM
       "#,
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_literal_cast("LINT", DATE_AND_TIME_TYPE, SourceRange::new(33..63,Some(2),Some(13),Some(2),Some(43))),
            Diagnostic::incompatible_literal_cast("LINT", TIME_OF_DAY_TYPE, SourceRange::new(77..106,Some(3),Some(13),Some(3),Some(42))),
            Diagnostic::incompatible_literal_cast("LINT", TIME_TYPE, SourceRange::new(120..136,Some(4),Some(13),Some(4),Some(29))),
            Diagnostic::incompatible_literal_cast("LINT", DATE_TYPE, SourceRange::new(150..170,Some(5),Some(13),Some(5),Some(33))),
            Diagnostic::incompatible_literal_cast("ULINT", DATE_AND_TIME_TYPE, SourceRange::new(185..216,Some(7),Some(13),Some(7),Some(43))),
            Diagnostic::incompatible_literal_cast("ULINT", TIME_OF_DAY_TYPE, SourceRange::new(230..260,Some(8),Some(13),Some(8),Some(42))),
            Diagnostic::incompatible_literal_cast("ULINT", TIME_TYPE, SourceRange::new(274..291,Some(9),Some(13),Some(9),Some(29))),
            Diagnostic::incompatible_literal_cast("ULINT", DATE_TYPE, SourceRange::new(305..326,Some(10),Some(13),Some(10),Some(33))),
            Diagnostic::incompatible_literal_cast("INT", DATE_AND_TIME_TYPE, SourceRange::new(341..370,Some(12),Some(13),Some(12),Some(43))),
            Diagnostic::incompatible_literal_cast("INT", TIME_OF_DAY_TYPE, SourceRange::new(384..412,Some(13),Some(13),Some(13),Some(42))),
            Diagnostic::incompatible_literal_cast("INT", TIME_TYPE, SourceRange::new(426..441,Some(14),Some(13),Some(14),Some(29))),
            Diagnostic::incompatible_literal_cast("INT", DATE_TYPE, SourceRange::new(455..474,Some(15),Some(13),Some(15),Some(33))),
        ]
    );
}

#[test]
fn char_cast_validate() {
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM prg
            
            CHAR#"A";
            WCHAR#'B';

			CHAR#"XY";
			WCHAR#'YZ';

        END_PROGRAM
       "#,
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::literal_out_of_range(r#""XY""#, "CHAR", SourceRange::new(83..92,Some(6), Some(13), Some(6), Some(22))),
            Diagnostic::literal_out_of_range("'YZ'", "WCHAR", SourceRange::new(97..107,Some(7), Some(13), Some(7), Some(23)))
        ]
    );
}
