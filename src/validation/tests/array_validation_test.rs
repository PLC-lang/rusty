use insta::assert_snapshot;

use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn array_access_validation() {
    let diagnostics = parse_and_validate_buffered(
        "
            VAR_GLOBAL CONSTANT
                start : INT := 1;
                end : INT := 2;
            END_VAR

            PROGRAM prg
            VAR
                multi : ARRAY[0..1,2..3] OF INT;
                nested : ARRAY[0..1] OF ARRAY[2..3] OF INT;
                arr : ARRAY[0..1] OF INT;
                negative_start : ARRAY[-2..2] OF INT;
                negative : ARRAY[-3..-1] OF INT;
                const : ARRAY[start..end] OF INT;
                int_ref : INT;
                string_ref : STRING;
            END_VAR

            // valid
            multi[0,3];
            nested[1][3];
            arr[1];
            negative_start[-1];
            negative[-2];
            const[1];
            arr[int_ref];

            // invalid
            multi[1,4]; // out of range
            nested[1][4]; // out of range
            arr[3]; // out of range
            negative_start[-4]; // out of range
            negative[-4]; // out of range
            const[3]; // out of range
            arr[string_ref]; // invalid type for array access
            int_ref[1]; // not an array
            END_PROGRAM
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn array_initialization_validation() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            arr         : ARRAY[1..2] OF DINT;
            arr2        : ARRAY[1..2] OF DINT := 1, 2;                                              // Missing `[`
            arr3        : ARRAY[1..2] OF myStruct := ((var1 := 1), (var1 := 2, var2 := (1, 2)));    // Missing `[`
            arr4        : ARRAY[1..2] OF myStruct := ((var1 := 1), (var1 := 2, var2 := 1, 2));      // Missing `[`
            arr_init    : ARRAY[1..2] OF DINT := (1, 2);                                            // Missing `[`
            x           : myStruct;
            y           : myStruct := (var1 := 1, var2 := 3, 4);                                    // Missing `[`
        END_VAR
            arr := 1, 2;                                                                            // Missing `[`
            arr := (1, 2);                                                                          // Missing `[`
            arr := (arr_init); // valid
            x   := (var1 := 1, var2 := 3, 4);                                                       // Missing `[`
            x   := (var1 := 1, var2 := arr_init); // valid
        END_FUNCTION

        TYPE myStruct : STRUCT
                var1 : DINT;
                var2 : ARRAY[1..2] OF DINT;
            END_STRUCT
        END_TYPE
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn array_access_dimension_mismatch() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION fn : DINT
            VAR_INPUT {ref}
                arr : ARRAY[0..5] OF DINT;
                vla : ARRAY[*] OF DINT;
            END_VAR

            // Valid
            arr[0] := 1;
            vla[0] := 1;

            // Invalid
            arr[0, 1] := 1;
            vla[0, 1] := 1;
            arr[0, 1, 2] := 1;
            vla[0, 1, 2] := 1;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn assignment_1d() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
            VAR
                arr         : ARRAY[1..5] OF DINT := [1, 2, 3, 4, 5, 6];
                arr_alt     : ARRAY[1..5] OF DINT := (1, 2, 3, 4, 5, 6);
            END_VAR

            arr := [1, 2, 3, 4];    // Valid
            arr := [1, 2, 3, 4, 5]; // Valid

            arr := (1, 2, 3, 4);
            arr := (1, 2, 3, 4, 5);
            arr := (1, 2, 3, 4, 5, 6);
            arr := [1, 2, 3, 4, 5, 6];
            arr := [1, 2, 3, 4, 5);
            arr := (1, 2, 3, 4, 5];
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn assignment_2d() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
            VAR
                arr             : ARRAY[1..2, 1..5] OF DINT := [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
                arr_alt         : ARRAY[1..2, 1..5] OF DINT := (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

                arr_nested      : ARRAY[1..2] OF ARRAY[1..5] OF DINT := [ [1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15] ];
                arr_nested_alt  : ARRAY[1..2] OF ARRAY[1..5] OF DINT := ( [1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15] );
            END_VAR

            arr := [1, 2, 3, 4, 5, 6, 7, 8, 9];     // Valid
            arr := [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Valid

            arr := (1, 2, 3, 4, 5, 6, 7, 8, 9);
            arr := (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
            arr := (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
            arr := [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

            arr_nested      := [ [1, 2, 3, 4, 5], [6, 7, 8, 9, 10] ]; // Valid
            arr_nested      := [ [1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15] ];
            arr_nested      := ( [1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15] );
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn assignment_3d() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
            VAR
                arr         : ARRAY[1..2, 1..2, 1..2] OF DINT := [1, 2, 3, 4, 5, 6, 7, 8, 9];
                arr_alt     : ARRAY[1..2, 1..2, 1..2] OF DINT := (1, 2, 3, 4, 5, 6, 7, 8, 9);

                arr_nested      : ARRAY[1..2] OF ARRAY[1..2] OF ARRAY[1..2] OF DINT := [ [[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]] ];
                arr_nested_alt  : ARRAY[1..2] OF ARRAY[1..2] OF ARRAY[1..2] OF DINT := ( [[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]] );
            END_VAR

            arr := [1, 2, 3, 4, 5, 6, 7];       // Valid
            arr := [1, 2, 3, 4, 5, 6, 7, 8];    // Valid

            arr := (1, 2, 3, 4, 5, 6, 7, 8);
            arr := (1, 2, 3, 4, 5, 6, 7, 8, 9);
            arr := [1, 2, 3, 4, 5, 6, 7, 8, 9];

            arr_nested := [ [[1, 2], [3, 4]], [[5, 6], [7, 8]] ]; // Valid
            arr_nested := [ [[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]] ];
            arr_nested := ( [[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]] );
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn assignment_structs() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE FOO : STRUCT
            idx : DINT;
            arr : ARRAY[1..2] OF BAR;
        END_STRUCT END_TYPE

        TYPE BAR : STRUCT
            arr : ARRAY[1..5] OF DINT;
        END_STRUCT END_TYPE

        FUNCTION main : DINT
            VAR
                foo_valid_0     : FOO := (idx := 0, arr := [(arr := [1, 2, 3, 4, 5])]);
                foo_valid_1     : FOO := (idx := 0, arr := [(arr := [1, 2, 3, 4, 5]), (arr := [1, 2, 3])]);
                foo_valid_2     : FOO := (idx := 0, arr := [(arr := [1, 2, 3, 4, 5]), (arr := [1, 2, 3, 4, 5])]);
                foo_valid_3     : FOO := (arr := [(arr := [1, 2, 3, 4, 5]), (arr := [1, 2, 3, 4, 5])], idx := 0);

                foo_invalid_0   : FOO := (idx := 0, arr := ((arr := (1, 2, 3, 4, 5)), (arr := (1, 2, 3, 4, 5))));
                foo_invalid_1   : FOO := (idx := 0, arr := ((arr := (1, 2, 3, 4, 5)), (arr := (1, 2, 3, 4, 5))));
            END_VAR
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
#[ignore = "Needs to be re-checked"]
fn exceeding_size_structs() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE Foo : STRUCT
            idx: DINT;
            arr : ARRAY[1..2] OF Bar;
        END_STRUCT END_TYPE

        TYPE Bar : STRUCT
            arr : ARRAY[1..2] OF DINT;
        END_STRUCT END_TYPE

        FUNCTION main : DINT
            VAR
                arr_a : Foo := (
                    idx := 1,
                    arr := [(arr := [1, 2]), (arr := [3, 4]), (arr := [5, 6])]          // Invalid, because the outter `arr` can only store 2 elements
                );

                arr_b : ARRAY[1..2] OF Foo := (
                    [idx := 2, arr := [(arr := [1, 2, 3]),  (arr := [4, 5])]],          // Invalid because of the first inner `arr`
                    [idx := 3, arr := [(arr := [1, 2]),     (arr := [3, 4, 5])]],       // Invalid because of the second inner `arr`
                );

                arr_c : ARRAY[1..2] OF Foo := (                                         // Invalid because only 2 elements can be stored, but 3 are provided
                    (idx := 4, arr := [(arr := [1, 2, 3]),  (arr := [4, 5])]),          // Invalid because of the first inner `arr`
                    (idx := 5, arr := [(arr := [1, 2]),     (arr := [3, 4, 5])]),       // Invalid because of the second inner `arr`
                    (idx := 6, arr := [(arr := 2(0)),       (arr := 4(0))]),            // Invalid ebcause of the second inner `arr`
                );
            END_VAR
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn assignment_multiplied_statement() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
            VAR
                arr_1d : ARRAY[1..5] OF DINT := [6(0)];

                arr_2d : ARRAY[1..2, 1..5] OF DINT := [11(0)];
                arr_2d_nested : ARRAY[1..2] OF ARRAY[1..5] OF DINT := [11(0)];

                arr_3d : ARRAY[1..2, 1..2, 1..2] OF DINT := [9(0)];
                arr_3d_nested : ARRAY[1..2] OF ARRAY[1..2] OF ARRAY[1..2] OF DINT := [9(0)];
            END_VAR

            // Valid
            arr_1d := [5(0)];
            arr_2d := [10(0)];
            arr_2d_nested := [10(0)];
            arr_3d = [8(0)];
            arr_3d_nested := [8(0)];

            // Invalid
            arr_1d := [6(0)];
            arr_2d := [11(0)];
            arr_2d_nested := [11(0)];
            arr_3d := [9(0)];
            arr_3d_nested := [9(0)];
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn parenthesized_struct_initializers() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE foo : STRUCT
            idx : DINT;
            val : DINT;
        END_STRUCT END_TYPE

        FUNCTION main : DINT
            VAR
                foo_valid : ARRAY[1..2] OF foo := [(idx := 0, val := 0), (idx := 1, val := 1)];
                foo_invalid_a : ARRAY[1..2] OF foo := [idx := 0, val := 0, idx := 1, val := 1];     // Both initializers missing parens
                foo_invalid_b : ARRAY[1..2] OF foo := [idx := 0, val := 0, (idx := 1, val := 1)];   // First initializer missing parens
                foo_invalid_c : ARRAY[1..2] OF foo := [(idx := 0, val := 0), idx := 1, val := 1];   // Second initializer missing parens
                foo_invalid_d : ARRAY[1..2] OF foo := [idx := 0];
            END_VAR
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn array_assignment_function_call() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION foo : ARRAY [0..3] OF USINT
            foo[0] := 0;
            foo[1] := 1;
            foo[2] := 2;
            foo[3] := 3;
        END_FUNCTION

        FUNCTION main : DINT
            VAR
                arr                 : ARRAY[0..3] OF USINT;
                arr_incorrect_size  : ARRAY[0..1] OF USINT;
            END_VAR

            arr                 := foo(); // We don't want a `... must be wrapped by []` error here
            arr_incorrect_size  := foo(); // We want a invalid size array error here
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn validate_ranges() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION foo
            VAR
                arr_A : ARRAY[1..-5] OF DINT;
                arr_B : ARRAY[1..5] OF ARRAY[1..-10] OF DINT;
                arr_C : ARRAY[1..-5] OF ARRAY[1..-10] OF DINT;
                arr_D : ARRAY[1..5, 1..-5] OF DINT;
            END_VAR
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E097]: Invalid range `1..-5`, did you mean `-5..1`?
      ┌─ <internal>:4:17
      │
    4 │                 arr_A : ARRAY[1..-5] OF DINT;
      │                 ^^^^^ Invalid range `1..-5`, did you mean `-5..1`?

    error[E097]: Invalid range `1..-10`, did you mean `-10..1`?
      ┌─ <internal>:5:17
      │
    5 │                 arr_B : ARRAY[1..5] OF ARRAY[1..-10] OF DINT;
      │                 ^^^^^ Invalid range `1..-10`, did you mean `-10..1`?

    error[E097]: Invalid range `1..-5`, did you mean `-5..1`?
      ┌─ <internal>:6:17
      │
    6 │                 arr_C : ARRAY[1..-5] OF ARRAY[1..-10] OF DINT;
      │                 ^^^^^ Invalid range `1..-5`, did you mean `-5..1`?

    error[E097]: Invalid range `1..-10`, did you mean `-10..1`?
      ┌─ <internal>:6:17
      │
    6 │                 arr_C : ARRAY[1..-5] OF ARRAY[1..-10] OF DINT;
      │                 ^^^^^ Invalid range `1..-10`, did you mean `-10..1`?

    error[E097]: Invalid range `1..-5`, did you mean `-5..1`?
      ┌─ <internal>:7:17
      │
    7 │                 arr_D : ARRAY[1..5, 1..-5] OF DINT;
      │                 ^^^^^ Invalid range `1..-5`, did you mean `-5..1`?
    ");
}
