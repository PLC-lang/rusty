use insta::assert_snapshot;

use crate::{
    test_utils::tests::{
        codegen, codegen_debug_without_unwrap, codegen_without_unwrap, parse_and_validate_buffered,
    },
    DebugLevel,
};

#[test]
fn initial_values_in_struct_types() {
    let result = codegen(
        "
        TYPE MyStruct:
        STRUCT
          x : INT := 7;
          xx : INT;
          y : BOOL := TRUE;
          yy : BOOL;
          z : REAL := 3.1415;
          zz : REAL;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL x : MyStruct; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn struct_initial_values_different_data_types() {
    let result = codegen(
        "
        TYPE MyStruct:
        STRUCT
          b  : BYTE   := 7;
          s  : SINT   := 7;
          us : USINT  := 7;
          w  : WORD   := 7;
          i  : INT    := 7;
          ui : UINT   := 7;
          dw : DWORD  := 7;
          di : DINT   := 7;
          udi: UDINT  := 7;
          lw : LWORD  := 7;
          li : LINT   := 7;
          uli: ULINT  := 7;
          r  : REAL   := 7.7;
          lr : LREAL  := 7.7;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL x : MyStruct; END_VAR
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_type_alias() {
    let result = codegen(
        "
        TYPE MyInt: INT := 7; END_TYPE
        VAR_GLOBAL x : MyInt; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_sub_range_type() {
    let result = codegen(
        "
        TYPE MyInt: INT(0..1000) := 7; END_TYPE
        VAR_GLOBAL x : MyInt; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn expression_list_as_array_initilization() {
    let result = codegen(
        "
        VAR_GLOBAL
            arr : ARRAY[-1..3] OF INT := 1, 2, 3;
            b_exp : ARRAY[-1..4] OF DINT := 1+3, 2*3, 7-1, 10;
            str : ARRAY[-1..2] OF STRING := 'first', 'second';
        END_VAR
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn incomplete_array_initialization() {
    let result = codegen(
        "
        VAR_GLOBAL
            arr : ARRAY[0..5] OF INT := 0, 1, 2;
        END_VAR
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn incomplete_array_initialization_with_custom_init_value() {
    let result = codegen(
        "
        TYPE MyInt : INT := 7; END_TYPE

        VAR_GLOBAL
            arr : ARRAY[0..5] OF MyInt := 0, 1, 2;
        END_VAR
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn alias_chain_with_lots_of_initializers() {
    let result = codegen(
        "
        TYPE MyInt: MyOtherInt1; END_TYPE
        VAR_GLOBAL
          x0 : MyInt;
          x1 : MyOtherInt1;
          x2 : MyOtherInt2;
          x3 : MyOtherInt3;
        END_VAR
        TYPE MyOtherInt3 : DINT := 3; END_TYPE
        TYPE MyOtherInt1 : MyOtherInt2 := 1; END_TYPE
        TYPE MyOtherInt2 : MyOtherInt3 := 2; END_TYPE
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_single_dimension_array_variable() {
    let result = codegen(
        "
        VAR_GLOBAL
          a : ARRAY[0..2] OF SINT  := [1, 2, 3];
          b : ARRAY[0..2] OF INT  := [1, 2, 3];
          c : ARRAY[0..2] OF DINT  := [1, 2, 3];
          d : ARRAY[0..2] OF LINT  := [1, 2, 3];
          e : ARRAY[0..2] OF USINT  := [1, 2, 3];
          f : ARRAY[0..2] OF UINT  := [1, 2, 3];
          g : ARRAY[0..2] OF ULINT := [1, 2, 3];
          h : ARRAY[0..2] OF BOOL := [TRUE, FALSE, TRUE];
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_single_dimension_array_type() {
    let result = codegen(
        "
        TYPE MyArray : ARRAY[0..2] OF INT := [1, 2, 3]; END_TYPE
        VAR_GLOBAL x : MyArray; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_multi_dimension_array_variable() {
    let result = codegen(
        "
         VAR_GLOBAL
           a : ARRAY[0..1, 0..1] OF BYTE  := [1,2,3,4];
         END_VAR
         ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_array_variable_using_multiplied_statement() {
    let result = codegen(
        "
         VAR_GLOBAL
           a : ARRAY[0..3] OF BYTE  := [4(7)];
           b : ARRAY[0..3] OF BYTE  := [2, 2(7), 3];
           c : ARRAY[0..9] OF BYTE  := [5(0,1)];
           d : ARRAY[0..9] OF BYTE  := [2(2(0), 2(1), 2)];
         END_VAR
         ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_struct_variable() {
    let result = codegen(
        "
        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
        END_STRUCT
        END_TYPE

         VAR_GLOBAL
           a : MyStruct  := (a:=3, b:=5);
           b : MyStruct  := (b:=3, a:=5);
         END_VAR
         ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_struct_variable_missing_init() {
    let result = codegen(
        "
        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
          c: DINT;
        END_STRUCT
        END_TYPE

         VAR_GLOBAL
           a : MyStruct  := (a:=5, c := 10);
           b : MyStruct  := (b:=3, c := 10);
         END_VAR
         ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn unresolvable_types_validation() {
    let msg = codegen_debug_without_unwrap(
        "
        VAR_GLOBAL
            a : MyStruct2  := (a := (c:=5, b:= 7), b := (a:=3, b:=2));
            b : MyStruct2  := (b := (a:= 9));
        END_VAR

        TYPE MyStruct2: STRUCT
            a : MyStruct  := (a:=5, b:=3);
            b : MyStruct  := (c:=7);
        END_STRUCT
        END_TYPE

        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
        END_STRUCT
        END_TYPE
     ",
        DebugLevel::None,
    )
    .expect_err("should fail");
    assert_snapshot!(msg);
}

#[test]
fn initial_nested_struct_delayed_init() {
    let result = codegen(
        "
        VAR_GLOBAL
            a : MyStruct2  := (a := (a:=5, b:= 7), b := (a:=3, b:=2));
            b : MyStruct2  := (b := (a:= 9));
        END_VAR

        TYPE MyStruct2: STRUCT
            a : MyStruct  := (a:=5, b:=3);
            b : MyStruct  := (b:=7);
        END_STRUCT
        END_TYPE

        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
        END_STRUCT
        END_TYPE
     ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn struct_init_with_wrong_types_does_not_trigger_codegen_validation() {
    let msg = codegen_debug_without_unwrap(
        "
        VAR_GLOBAL
            a : MyType;
            b : MyStruct;
        END_VAR

        TYPE MyType : INT := 'hello'; END_TYPE

        TYPE MyStruct: STRUCT
          a: DINT := 'hello';
          b: DINT := 8;
        END_STRUCT
        END_TYPE
     ",
        DebugLevel::None,
    )
    .expect_err("Should fail");

    assert_snapshot!(msg);
}

#[test]
fn initial_values_in_fb_variable() {
    let result = codegen(
        "FUNCTION_BLOCK TON
        VAR_INPUT
            a: INT;
            b: INT;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main

        VAR CONSTANT
            TEN : INT := 10;
        END_VAR
        VAR
            struct1 : TON := (a := 10, b := TEN + 7);
            struct2 : TON := (b := 10, a := TEN + 7);
        END_VAR
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn complex_initial_values_in_struct_variable_using_multiplied_statement() {
    let result = codegen(
        "
        TYPE MyPoint: STRUCT
          x: DINT;
          y: DINT;
        END_STRUCT
        END_TYPE

        TYPE MyStruct: STRUCT
          point: MyPoint;
          my_array: ARRAY[0..3] OF INT;
          f : DINT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
          a : MyStruct  := (
              point := (x := 1, y:= 2),
              my_array := [0,1,2,3],
              f := 7
            );
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn struct_with_one_field_can_be_initialized() {
    let result = codegen(
        "
        TYPE MyPoint: STRUCT
          x: DINT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
          a : MyPoint := ( x := 7);
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn struct_initializer_needs_assignments() {
    let source = "
            TYPE Point: STRUCT
              x: DINT;
              y: DINT;
            END_STRUCT
            END_TYPE

            VAR_GLOBAL
                x : Point := (x := 1, 2);
            END_VAR
           ";
    let msg = codegen_without_unwrap(source).unwrap_err();
    assert_snapshot!(msg);
}

#[test]
fn struct_initialization_uses_types_default_if_not_provided() {
    // GIVEN a custom dataType MyDINT with initial value of 7
    // AND a struct point that uses it for member z
    // AND a global instance that does not initializes z
    let source = "
            TYPE MyDINT : DINT := 7; END_TYPE

            TYPE Point: STRUCT
              x: DINT;
              y: DINT;
              z: MyDINT;
            END_STRUCT
            END_TYPE

            VAR_GLOBAL
                x : Point := (x := 1, y := 2);
            END_VAR
           ";

    //WHEN it is generated
    let result = codegen(source);

    //THEN we expect z to be 7
    insta::assert_snapshot!(result);
}

#[test]
fn struct_initializer_uses_fallback_to_field_default() {
    let source = "
            TYPE MyOtherDINT : DINT := 2 ; END_TYPE
            TYPE MyDINT      : MyOtherDINT; END_TYPE

            TYPE Point: STRUCT
              x: DINT;
              y: MyDINT;
              z: DINT := 3;
            END_STRUCT
            END_TYPE

            VAR_GLOBAL
                x : Point := (x := 1);
            END_VAR
           ";
    let result = codegen(source);

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_struct_initialization() {
    let source = "
    TYPE myStruct : STRUCT
            a, b : DINT;
            c : ARRAY[0..1] OF DINT;
        END_STRUCT
    END_TYPE

    TYPE AliasMyStruct : myStruct; END_TYPE

    VAR_GLOBAL CONSTANT
        str : myStruct := (a := 50, b := 60, c := [70, 80]);
        alias_str : AliasMyStruct := (a := 50, b := 60, c := [70, 80]);
        global_arr : ARRAY[0..1] OF DINT := [30, 40];
    END_VAR

    PROGRAM main
    VAR
        arr : ARRAY[0..1] OF myStruct := [(a := 10, b := 20, c := [30, 40]), str];
        alias_arr : ARRAY[0..1] OF AliasMyStruct := [(a := 10, b := 20, c := global_arr), alias_str];
    END_VAR
    END_PROGRAM
    ";
    let result = codegen(source);

    insta::assert_snapshot!(result);
}

#[test]
fn type_defaults_are_used_for_uninitialized_constants() {
    let result = codegen_without_unwrap(
        r#"
        TYPE MyOtherDINT : DINT := 2 ; END_TYPE
        TYPE MyDINT      : MyOtherDINT; END_TYPE

        TYPE MyInt : INT := 7; END_TYPE

        VAR_GLOBAL CONSTANT
            a : MyInt;
            b : INT := a + 2*a;
            c : MyDINT;
            d : MyDINT := a + b + c;
        END_VAR
        "#,
    );
    // we expect some initial values:
    // a := default(MyInt) = 7;
    // b := 7 + 14 = 21;
    // c := default(MyDINT) = Default(MyOtherDINT) = 2;
    // d := 7 + 21 + 2 = 30
    insta::assert_snapshot!(result.unwrap());
}

#[test]
fn partly_uninitialized_const_struct_will_get_default_values() {
    let result = codegen_without_unwrap(
        r#"
            TYPE MyOtherDINT : DINT := 2 ; END_TYPE
            TYPE MyDINT      : MyOtherDINT; END_TYPE

            TYPE Point: STRUCT
              x: DINT;
              y: MyDINT;
              z: DINT := 3;
            END_STRUCT
            END_TYPE

            VAR_GLOBAL CONSTANT
                x : Point := (x := 1);
                empty: Point;
            END_VAR
        "#,
    );

    insta::assert_snapshot!(result.unwrap());
}

#[test]
fn partly_uninitialized_const_struct_will_not_report_errors() {
    let diagnostics = parse_and_validate_buffered(
        r#"
            TYPE MyOtherDINT : DINT := 2 ; END_TYPE
            TYPE MyDINT      : MyOtherDINT; END_TYPE

            TYPE Point: STRUCT
              x: DINT;
              y: MyDINT;
              z: DINT := 3;
            END_STRUCT
            END_TYPE

            VAR_GLOBAL CONSTANT
                x : Point := (x := 1);
                empty: Point;
            END_VAR
        "#,
    );
    assert!(diagnostics.is_empty())
}

#[test]
fn enums_with_inline_initializer_do_not_report_errors() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
              x : (red, yellow, green) := red;
        END_VAR

        FUNCTION main : DINT
            VAR
                y : (redy := 1, yellowy := 2, greeny := 3) := greeny;
            END_VAR
            VAR
                var1 : (x1 := 1, x2 := 2, x3 := 3) := x1;
                // or
                var2 : (x5, x6, x7) := x7;
            END_VAR
        END_FUNCTION
        "#,
    );
    assert!(diagnostics.is_empty())
}

#[test]
fn enums_with_inline_initializer_are_initialized() {
    let res = codegen(
        r#"
        VAR_GLOBAL
              x : (red, yellow, green) := 2;
        END_VAR

        FUNCTION main : DINT
            VAR
                y : (redy := 1, yellowy := 2, greeny := 3) := 2;
            END_VAR
            VAR
                var1 : (x1 := 1, x2 := 2, x3 := 3) := x1;
                // or
                var2 : (x5, x6, x7) := x7;

                var3 : (x8, x9) := yellow;
            END_VAR
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(res);
}

#[test]
fn skipped_field_members_for_array_of_structs_are_zero_initialized() {
    let res = codegen(
        r#"
        TYPE STRUCT1 : STRUCT
            idx: DINT;
            arr: ARRAY[1..2] OF STRUCT2;
        END_STRUCT END_TYPE

        TYPE STRUCT2 : STRUCT
            x: DINT;
            y: DINT;
        END_STRUCT END_TYPE

        PROGRAM main
            VAR
                var_init1 : ARRAY[1..3] OF STRUCT1 := [
                    (idx := 0, arr := [(x := 1)]),
                    (idx := 2, arr := [(x := 1, y := 1)]),
                    (idx := 1, arr := [(x := 1), (y := 2)])
                ];
            END_VAR
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(res);
}