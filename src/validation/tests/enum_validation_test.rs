use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

#[test]
fn enum_variants_mismatch() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE State : (Idle := 0, Working := 100); END_TYPE
        FUNCTION foo : DINT END_FUNCTION
        FUNCTION bar : State END_FUNCTION

        PROGRAM main
                VAR
                    color       : (red := 0, green := 1, blue := 2);
                    localState  : State;

                    validReferenceForEnum   : DINT := 0;
                    invalidReferenceForEnum : DINT := 99; // ...problems but a ~~bi-~~ validation ain't one
                END_VAR

                // These are Ok
                localState := State.Idle;
                localState := State.Working;
                localState := 0;
                localState := 100;

                color := red;
                color := green;
                color := blue;
                color := 0;
                color := 1;
                color := 2;

                color := State.Idle;    // State.Idle == 0 == Color.Red
                localState := red;      // Color.Red  == 0 == State.Idle

                color := validReferenceForEnum; // We still want an error here, because no const-expr
                localState := validReferenceForEnum;

                localState := bar(); // Value of `bar` unknown, BUT return type is `State` which is Ok

                // These are NOT Ok
                // -> valid values for color are [0, 1, 2] and for localState are [0, 100]
                color := 99;
                color := State.Working;
                color := invalidReferenceForEnum;
                color := foo(); // Value of `foo()` unknown, might be outside of variants range

                localState := 99;
                localState := green;
                localState := blue;
                localState := invalidReferenceForEnum;
        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn enum_mismatch_error_with_many_variants_truncates_values_for_better_readability() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE Color : (red, green, blue, yellow, cyan, orange, purple, violet); END_TYPE
        PROGRAM main
            VAR
                myColor : Color;
            END_VAR
            myColor := 1000;
        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn validate_enum_variant_initializer() {
    let diagnostics = parse_and_validate_buffered(
        "VAR_GLOBAL
                x : (red, yellow, green) := 2; // error
            END_VAR

            PROGRAM  main
            VAR
                y : (metallic := 1, matte := 2, neon := 3) := red; // error
            END_VAR
            VAR
                var1 : (x1 := 1, x2 := 2, x3 := 3) := yellow;   // warning
                var2 : (x5, x6, x7) := neon;                    // error
                var3 : (a, b, c) := 7;                          // error
            END_VAR
            END_PROGRAM",
    );
    assert_snapshot!(diagnostics);
}

#[test]
fn enum_variants_mismatch_but_values_are_identical() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE Animal: (Dog, Cat, Horse); END_TYPE

        PROGRAM main
        VAR
            color: (red, green, blue);
            water: (still, medium, sparkling);
        END_VAR
            color := green;     // ok
            water := sparkling; // ok
            color := Dog;       // warning
            water := blue;      // warning
            color := sparkling; // warning
            color := 2;         // warning
        END_PROGRAM",
    );

    assert_snapshot!(diagnostics);
}
