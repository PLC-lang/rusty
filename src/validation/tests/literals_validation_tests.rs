use plc_diagnostics::diagnostics::Diagnostic;

use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

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

    assert_validation_snapshot!(&diagnostics);
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

    assert_validation_snapshot!(&diagnostics);
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

    assert_validation_snapshot!(&diagnostics);
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

    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn literal_cast_with_non_literal() {
    let diagnostics = parse_and_validate(
        "PROGRAM exp
            INT#[x];
        END_PROGRAM

        VAR_GLOBAL x : INT; END_VAR",
    );
    assert_validation_snapshot!(&diagnostics);
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

    assert_validation_snapshot!(&diagnostics);
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

    assert_validation_snapshot!(&diagnostics);
}
