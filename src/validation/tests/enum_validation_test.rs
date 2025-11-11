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

#[test]
fn enum_with_invalid_type() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE MyIntAlias : INT; END_TYPE
        TYPE MyRealAlias : REAL; END_TYPE
        TYPE MyStruct : STRUCT x: INT; END_STRUCT; END_TYPE
        TYPE MyArray : ARRAY[1..10] OF INT; END_TYPE
        TYPE MyStringAlias : STRING; END_TYPE

        // Invalid: REAL
        TYPE InvalidEnum1 : REAL (red := 1, green := 2, blue := 3); END_TYPE

        // Invalid: STRING
        TYPE InvalidEnum2 : STRING (a := 1, b := 2); END_TYPE

        // Invalid: REAL alias
        TYPE InvalidEnum3 : MyRealAlias (x := 1, y := 2); END_TYPE

        // Invalid: Non-existent type
        TYPE InvalidEnum4 : NonExistentType (p := 1, q := 2); END_TYPE

        // Invalid: WSTRING
        TYPE InvalidEnum5 : WSTRING (a := 1, b := 2); END_TYPE

        // Invalid: Struct type
        TYPE InvalidEnum6 : MyStruct (red := 1, blue := 2); END_TYPE

        // Invalid: Array type
        TYPE InvalidEnum7 : MyArray (a := 1, b := 2); END_TYPE

        // Invalid: String alias
        TYPE InvalidEnum8 : MyStringAlias (p := 1, q := 2); END_TYPE

        // Invalid: LREAL (floating point)
        TYPE InvalidEnum9 : LREAL (low := 1, high := 2); END_TYPE

        // Valid: INT (61131 standard syntax)
        TYPE ValidEnum1 : INT (red := 1, green := 2); END_TYPE

        // Valid: INT (Codesys syntax)
        TYPE ValidEnum2 : (red := 1, green := 2) INT; END_TYPE

        // Valid: DWORD (61131 standard syntax)
        TYPE ValidEnum3 : DWORD (a := 1, b := 2); END_TYPE

        // Valid: DWORD (Codesys syntax)
        TYPE ValidEnum4 : (a := 1, b := 2) DWORD; END_TYPE

        // Valid: BYTE
        TYPE ValidEnum5 : BYTE (x := 1, y := 2); END_TYPE

        // Valid: BYTE (Codesys syntax)
        TYPE ValidEnum6 : (x := 1, y := 2) BYTE; END_TYPE

        // Valid: INT alias (61131 syntax)
        TYPE ValidEnum7 : MyIntAlias (z := 1, w := 2); END_TYPE

        // Valid: INT alias (Codesys syntax)
        TYPE ValidEnum8 : (z := 1, w := 2) MyIntAlias; END_TYPE

        // Valid: BOOL
        TYPE ValidEnum9 : BOOL (false_val := 0, true_val := 1); END_TYPE

        // Valid: BOOL (Codesys syntax)
        TYPE ValidEnum10 : (false_val := 0, true_val := 1) BOOL; END_TYPE

        // Valid: All integer types (61131 syntax)
        TYPE ValidEnum11 : LINT (a := 1, b := 2); END_TYPE
        TYPE ValidEnum12 : ULINT (a := 1, b := 2); END_TYPE
        TYPE ValidEnum13 : USINT (a := 1, b := 2); END_TYPE
        TYPE ValidEnum14 : UINT (a := 1, b := 2); END_TYPE
        TYPE ValidEnum15 : UDINT (a := 1, b := 2); END_TYPE
        TYPE ValidEnum16 : WORD (a := 1, b := 2); END_TYPE
        TYPE ValidEnum17 : LWORD (a := 1, b := 2); END_TYPE
        TYPE ValidEnum18 : SINT (a := 1, b := 2); END_TYPE
        TYPE ValidEnum19 : DINT (a := 1, b := 2); END_TYPE

        // Valid: Some integer types (Codesys syntax)
        TYPE ValidEnum20 : (a := 1, b := 2) LINT; END_TYPE
        TYPE ValidEnum21 : (a := 1, b := 2) ULINT; END_TYPE
        TYPE ValidEnum22 : (a := 1, b := 2) WORD; END_TYPE
        TYPE ValidEnum23 : (a := 1, b := 2) LWORD; END_TYPE
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E122]: Invalid type 'REAL' for enum. Only integer types are allowed
      ┌─ <internal>:9:14
      │
    9 │         TYPE InvalidEnum1 : REAL (red := 1, green := 2, blue := 3); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'REAL' for enum. Only integer types are allowed

    error[E122]: Invalid type 'STRING' for enum. Only integer types are allowed
       ┌─ <internal>:12:14
       │
    12 │         TYPE InvalidEnum2 : STRING (a := 1, b := 2); END_TYPE
       │              ^^^^^^^^^^^^ Invalid type 'STRING' for enum. Only integer types are allowed

    error[E122]: Invalid type 'MyRealAlias' for enum. Only integer types are allowed
       ┌─ <internal>:15:14
       │
    15 │         TYPE InvalidEnum3 : MyRealAlias (x := 1, y := 2); END_TYPE
       │              ^^^^^^^^^^^^ Invalid type 'MyRealAlias' for enum. Only integer types are allowed

    error[E052]: Unknown type: NonExistentType
       ┌─ <internal>:18:14
       │
    18 │         TYPE InvalidEnum4 : NonExistentType (p := 1, q := 2); END_TYPE
       │              ^^^^^^^^^^^^ Unknown type: NonExistentType

    error[E122]: Invalid type 'WSTRING' for enum. Only integer types are allowed
       ┌─ <internal>:21:14
       │
    21 │         TYPE InvalidEnum5 : WSTRING (a := 1, b := 2); END_TYPE
       │              ^^^^^^^^^^^^ Invalid type 'WSTRING' for enum. Only integer types are allowed

    error[E122]: Invalid type 'MyStruct' for enum. Only integer types are allowed
       ┌─ <internal>:24:14
       │
    24 │         TYPE InvalidEnum6 : MyStruct (red := 1, blue := 2); END_TYPE
       │              ^^^^^^^^^^^^ Invalid type 'MyStruct' for enum. Only integer types are allowed

    error[E122]: Invalid type 'MyArray' for enum. Only integer types are allowed
       ┌─ <internal>:27:14
       │
    27 │         TYPE InvalidEnum7 : MyArray (a := 1, b := 2); END_TYPE
       │              ^^^^^^^^^^^^ Invalid type 'MyArray' for enum. Only integer types are allowed

    error[E122]: Invalid type 'MyStringAlias' for enum. Only integer types are allowed
       ┌─ <internal>:30:14
       │
    30 │         TYPE InvalidEnum8 : MyStringAlias (p := 1, q := 2); END_TYPE
       │              ^^^^^^^^^^^^ Invalid type 'MyStringAlias' for enum. Only integer types are allowed

    error[E122]: Invalid type 'LREAL' for enum. Only integer types are allowed
       ┌─ <internal>:33:14
       │
    33 │         TYPE InvalidEnum9 : LREAL (low := 1, high := 2); END_TYPE
       │              ^^^^^^^^^^^^ Invalid type 'LREAL' for enum. Only integer types are allowed
    ");
}

#[test]
fn enum_with_time_types_should_be_invalid() {
    let diagnostics = parse_and_validate_buffered(
        "
        // Time types should not be allowed as enum base types
        TYPE InvalidEnum1 : TIME (a := 1, b := 2); END_TYPE
        TYPE InvalidEnum2 : DATE (x := 1, y := 2); END_TYPE
        TYPE InvalidEnum3 : TOD (morning := 1, evening := 2); END_TYPE
        TYPE InvalidEnum4 : DT (start := 1, end := 2); END_TYPE
        TYPE InvalidEnum5 : DATE_AND_TIME (t1 := 1, t2 := 2); END_TYPE
        TYPE InvalidEnum6 : TIME_OF_DAY (t1 := 1, t2 := 2); END_TYPE
        TYPE InvalidEnum7 : LTIME (a := 1, b := 2); END_TYPE

        // Valid: Regular integer types should still work
        TYPE ValidEnum1 : INT (a := 1, b := 2); END_TYPE
        TYPE ValidEnum2 : LINT (a := 1, b := 2); END_TYPE
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E122]: Invalid type 'TIME' for enum. Only integer types are allowed
      ┌─ <internal>:3:14
      │
    3 │         TYPE InvalidEnum1 : TIME (a := 1, b := 2); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'TIME' for enum. Only integer types are allowed

    error[E122]: Invalid type 'DATE' for enum. Only integer types are allowed
      ┌─ <internal>:4:14
      │
    4 │         TYPE InvalidEnum2 : DATE (x := 1, y := 2); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'DATE' for enum. Only integer types are allowed

    error[E122]: Invalid type 'TOD' for enum. Only integer types are allowed
      ┌─ <internal>:5:14
      │
    5 │         TYPE InvalidEnum3 : TOD (morning := 1, evening := 2); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'TOD' for enum. Only integer types are allowed

    error[E122]: Invalid type 'DT' for enum. Only integer types are allowed
      ┌─ <internal>:6:14
      │
    6 │         TYPE InvalidEnum4 : DT (start := 1, end := 2); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'DT' for enum. Only integer types are allowed

    error[E122]: Invalid type 'DATE_AND_TIME' for enum. Only integer types are allowed
      ┌─ <internal>:7:14
      │
    7 │         TYPE InvalidEnum5 : DATE_AND_TIME (t1 := 1, t2 := 2); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'DATE_AND_TIME' for enum. Only integer types are allowed

    error[E122]: Invalid type 'TIME_OF_DAY' for enum. Only integer types are allowed
      ┌─ <internal>:8:14
      │
    8 │         TYPE InvalidEnum6 : TIME_OF_DAY (t1 := 1, t2 := 2); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'TIME_OF_DAY' for enum. Only integer types are allowed

    error[E122]: Invalid type 'LTIME' for enum. Only integer types are allowed
      ┌─ <internal>:9:14
      │
    9 │         TYPE InvalidEnum7 : LTIME (a := 1, b := 2); END_TYPE
      │              ^^^^^^^^^^^^ Invalid type 'LTIME' for enum. Only integer types are allowed
    ");
}

#[test]
fn enum_variants_initialized_with_other_enum_values() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE SubEnum : INT (a := 10, b := 20, c := 30); END_TYPE

        TYPE MainEnum : INT (
            x := SubEnum.a,
            y := SubEnum.b,
            z := SubEnum.c
        ); END_TYPE

        VAR_GLOBAL
            myMain : MainEnum;
            mySub : SubEnum;
        END_VAR

        PROGRAM main
            myMain := MainEnum.x;  // Should be ok
            myMain := SubEnum.a; 
            mySub := MainEnum.x;
        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics, @r"
    note[E092]: Replace `SubEnum.a` with `x`
       ┌─ <internal>:17:23
       │
     4 │         TYPE MainEnum : INT (
       │              -------- see also
       ·
    17 │             myMain := SubEnum.a; 
       │                       ^^^^^^^^^ Replace `SubEnum.a` with `x`

    note[E092]: Replace `MainEnum.x` with `a`
       ┌─ <internal>:18:22
       │
     2 │         TYPE SubEnum : INT (a := 10, b := 20, c := 30); END_TYPE
       │              ------- see also
       ·
    18 │             mySub := MainEnum.x;
       │                      ^^^^^^^^^^ Replace `MainEnum.x` with `a`
    ");
}

#[test]
#[ignore = "currently fails during codegen, tracked in #1546"]
fn enum_type_assigned_without_qualifier() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE Color : INT (red := 1, green := 2, blue := 3); END_TYPE

        VAR_GLOBAL
            myColor : Color;
        END_VAR

        PROGRAM main
            myColor := red;      // Unqualified variant - ok
            myColor := Color;    // Type itself - should be unresolvable
        END_PROGRAM
        ",
    );

    assert!(diagnostics.len() > 0);
    assert_snapshot!(diagnostics, @r#""#);
}

#[test]
#[ignore = "currently fails during codegen, tracked in #1546"]
fn type_name_used_as_value() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE MyInt : INT; END_TYPE
        TYPE Color : INT (red := 1, green := 2, blue := 3); END_TYPE
        TYPE MyStruct : STRUCT x: INT; y: INT; END_STRUCT; END_TYPE

        PROGRAM main
            VAR
                a : INT;
                b : MyInt;
                c : Color;
                d : MyStruct;
            END_VAR
            a := INT;        // Type name as value - generic type
            b := MyInt;      // Type name as value - alias type
            c := Color;      // Type name as value - enum type
            d := MyStruct;   // Type name as value - struct type
        END_PROGRAM
        ",
    );

    assert!(diagnostics.len() > 0);
    assert_snapshot!(diagnostics, @r#""#);
}
