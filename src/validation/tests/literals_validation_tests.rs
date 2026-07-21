use insta::assert_snapshot;

use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn int_literal_casts_max_values_are_validated() {
    let diagnostics = parse_and_validate_buffered(
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

    assert_snapshot!(&diagnostics);
}

#[test]
fn bool_literal_casts_are_validated() {
    let diagnostics = parse_and_validate_buffered(
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

    assert_snapshot!(&diagnostics);
}

#[test]
fn string_literal_casts_are_validated() {
    let diagnostics = parse_and_validate_buffered(
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

    assert_snapshot!(&diagnostics);
}

#[test]
fn real_literal_casts_are_validated() {
    let diagnostics = parse_and_validate_buffered(
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

    assert_snapshot!(&diagnostics);
}

#[test]
fn literal_cast_with_non_literal() {
    let diagnostics = parse_and_validate_buffered(
        "PROGRAM exp
            INT#[x];
        END_PROGRAM

        VAR_GLOBAL x : INT; END_VAR",
    );
    assert_snapshot!(&diagnostics);
}

#[test]
fn literal_enum_elements_validate_without_errors() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE Animal: (Dog, Cat, Horse); END_TYPE
        TYPE Color: (Red, Yellow, Green); END_TYPE

        PROGRAM exp
            Animal#Dog;
            Color#Yellow;
        END_PROGRAM",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn date_literal_casts_are_validated() {
    let diagnostics = parse_and_validate_buffered(
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

    assert_snapshot!(&diagnostics);
}

#[test]
fn char_cast_validate() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prg

            CHAR#"A";
            WCHAR#'B';

            CHAR#"XY";
            WCHAR#'YZ';

        END_PROGRAM
       "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn there_should_be_no_downcast_warning_for_literal_assignment_to_integer_types() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prg
        VAR
            varSINT : SINT;
            varINT : INT;
            varDINT : DINT;
            varLINT : LINT;

            varUSINT : USINT;
            varUINT : UINT;
            varUDINT : UDINT;
            varULINT : ULINT;

            varBOOL : BOOL;
        END_VAR

        varSINT := 0;
        varINT := 0;
        varDINT := 0;
        varLINT := 0;

        varUSINT := 0;
        varUINT := 0;
        varUDINT := 0;
        varULINT := 0;

        // Prove the counter-case, this should still be valid
        varBOOL := 0;

        END_PROGRAM
       "#,
    );

    assert_snapshot!(&diagnostics, @r"");
}

#[test]
fn short_temporal_literals_overflow_and_underflow_produce_warnings() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prg
        VAR
            d_underflow : DATE := DATE#1969-12-31;
            d_overflow : DATE := DATE#2500-01-01;
            dt_underflow : DT := DT#1969-12-31-23:59:59;
            dt_overflow : DT := DT#2500-01-01-00:00:00;
            t_underflow : TIME := TIME#-1ms;
            t_overflow : TIME := TIME#4294967296ms;
            tod_overflow : TOD := TOD#24:00:00;
        END_VAR
        END_PROGRAM
       "#,
    );

    let normalized = diagnostics.lines().map(str::trim_start).collect::<Vec<_>>().join("\n");
    assert_snapshot!(normalized, @r"
    warning[E146]: DATE literal underflow detected
    ┌─ <internal>:4:35
    │
    4 │             d_underflow : DATE := DATE#1969-12-31;
    │                                   ^^^^^^^^^^^^^^^ DATE literal underflow detected

    warning[E146]: DATE literal out-of-range detected
    ┌─ <internal>:5:34
    │
    5 │             d_overflow : DATE := DATE#2500-01-01;
    │                                  ^^^^^^^^^^^^^^^ DATE literal out-of-range detected

    warning[E146]: DATE_AND_TIME literal underflow detected
    ┌─ <internal>:6:34
    │
    6 │             dt_underflow : DT := DT#1969-12-31-23:59:59;
    │                                  ^^^^^^^^^^^^^^^^^^^^^^ DATE_AND_TIME literal underflow detected

    warning[E146]: DATE_AND_TIME literal out-of-range detected
    ┌─ <internal>:7:33
    │
    7 │             dt_overflow : DT := DT#2500-01-01-00:00:00;
    │                                 ^^^^^^^^^^^^^^^^^^^^^^ DATE_AND_TIME literal out-of-range detected

    warning[E146]: TIME literal underflow detected
    ┌─ <internal>:8:35
    │
    8 │             t_underflow : TIME := TIME#-1ms;
    │                                   ^^^^^^^^^ TIME literal underflow detected

    warning[E146]: TIME literal overflow detected
    ┌─ <internal>:9:34
    │
    9 │             t_overflow : TIME := TIME#4294967296ms;
    │                                  ^^^^^^^^^^^^^^^^^ TIME literal overflow detected

    warning[E146]: TIME_OF_DAY literal out-of-range detected
    ┌─ <internal>:10:35
    │
    10 │             tod_overflow : TOD := TOD#24:00:00;
    │                                   ^^^^^^^^^^^^ TIME_OF_DAY literal out-of-range detected
    ");
}

#[test]
fn long_temporal_literals_do_not_produce_short_range_warnings() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prg
        VAR
            d : LDATE := LDATE#2100-01-01;
            dt : LDT := LDT#2100-01-01-00:00:00;
            t : LTIME := LTIME#-1ms;
            tod : LTOD := LTOD#23:59:59;
        END_VAR
        END_PROGRAM
       "#,
    );

    let normalized = diagnostics.lines().map(str::trim_start).collect::<Vec<_>>().join("\n");
    assert_snapshot!(normalized, @r"");
}
