use crate::{
    test_utils::tests::parse_and_validate,
    typesystem::{DATE_AND_TIME_TYPE, DATE_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE},
    Diagnostic,
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
            Diagnostic::literal_out_of_range("256", "BYTE", (67..75).into()),
            Diagnostic::literal_out_of_range("65536", "UINT", (123..134).into()),
            Diagnostic::literal_out_of_range("4294967296", "UDINT", (190..209).into()),
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
            Diagnostic::literal_out_of_range("2", "BOOL", (120..126).into()),
            Diagnostic::incompatible_literal_cast("BOOL", "2.3", (140..148).into()),
            Diagnostic::incompatible_literal_cast("BOOL", "'abc'", (162..172).into()),
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
            Diagnostic::incompatible_literal_cast("STRING", "true", (102..113).into()),
            Diagnostic::incompatible_literal_cast("WSTRING", "false", (127..140).into()),
            Diagnostic::incompatible_literal_cast("STRING", "22", (155..164).into()),
            Diagnostic::incompatible_literal_cast("WSTRING", "33", (178..188).into()),
            Diagnostic::incompatible_literal_cast("STRING", "3.14", (203..214).into()),
            Diagnostic::incompatible_literal_cast("WSTRING", "1.0", (228..239).into()),
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
            Diagnostic::incompatible_literal_cast("REAL", "'3.14'", (180..191).into()),
            Diagnostic::incompatible_literal_cast("LREAL", r#""3.14""#, (252..264).into())
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
        vec![Diagnostic::literal_expected((25..32).into())],
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
            Diagnostic::incompatible_literal_cast("LINT", DATE_AND_TIME_TYPE, (33..63).into()),
            Diagnostic::incompatible_literal_cast("LINT", TIME_OF_DAY_TYPE, (77..106).into()),
            Diagnostic::incompatible_literal_cast("LINT", TIME_TYPE, (120..136).into()),
            Diagnostic::incompatible_literal_cast("LINT", DATE_TYPE, (150..170).into()),
            Diagnostic::incompatible_literal_cast("ULINT", DATE_AND_TIME_TYPE, (185..216).into()),
            Diagnostic::incompatible_literal_cast("ULINT", TIME_OF_DAY_TYPE, (230..260).into()),
            Diagnostic::incompatible_literal_cast("ULINT", TIME_TYPE, (274..291).into()),
            Diagnostic::incompatible_literal_cast("ULINT", DATE_TYPE, (305..326).into()),
            Diagnostic::incompatible_literal_cast("INT", DATE_AND_TIME_TYPE, (341..370).into()),
            Diagnostic::incompatible_literal_cast("INT", TIME_OF_DAY_TYPE, (384..412).into()),
            Diagnostic::incompatible_literal_cast("INT", TIME_TYPE, (426..441).into()),
            Diagnostic::incompatible_literal_cast("INT", DATE_TYPE, (455..474).into()),
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
            Diagnostic::literal_out_of_range(r#""XY""#, "CHAR", (83..92).into()),
            Diagnostic::literal_out_of_range("'YZ'", "WCHAR", (97..107).into())
        ]
    );
}
