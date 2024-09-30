use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

#[test]
fn uninitialized_constants_fall_back_to_the_default() {
    let diagnostics = parse_and_validate_buffered(
        "
        VAR_GLOBAL
            gX : INT;
            gXi : INT := 7;
        END_VAR

        VAR_GLOBAL CONSTANT
            cgX : INT;
            cgXi : INT := 7;
        END_VAR

        PROGRAM prg
            VAR
                x : INT;
                xi : INT := 7;
            END_VAR

            VAR CONSTANT
                cx : INT;
                cxi : INT := 7;
            END_VAR
        END_PROGRAM
       ",
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn unresolvable_variables_are_reported() {
    let diagnostics = parse_and_validate_buffered(
        "
        VAR_GLOBAL
            gX : INT := 7 + cgX;
            gXi : INT := 7;
        END_VAR

        VAR_GLOBAL CONSTANT
            cgX : INT;  //default
            cgXi : INT := 7;
        END_VAR

        PROGRAM prg
            VAR
                x : INT;
                xi : INT := 7;
            END_VAR

            VAR CONSTANT
                cx : INT := cx;  //unresolvable
                cxi : INT := 7;
                cai : INT := a;  //unresolvable
            END_VAR
        END_PROGRAM
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn constant_on_illegal_var_blocks_cause_validation_issue() {
    // GIVEN different variable block types with the CONSTANT modifier
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        VAR_GLOBAL CONSTANT //OK
        END_VAR

        PROGRAM prg
            VAR_INPUT CONSTANT //illegal
            END_VAR

            VAR_OUTPUT CONSTANT //illegal
            END_VAR

            VAR_IN_OUT CONSTANT //illegal
            END_VAR

            VAR CONSTANT //ok
            END_VAR

        END_PROGRAM

        CLASS cls
            VAR CONSTANT //ok
            END_VAR

             METHOD testMethod
                VAR_INPUT CONSTANT //illegal
                END_VAR

                VAR_OUTPUT CONSTANT //illegal
                END_VAR

                VAR_IN_OUT CONSTANT //illegal
                END_VAR

                VAR CONSTANT //ok
                END_VAR
            END_METHOD
        END_CLASS
       ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn constant_fb_instances_are_illegal() {
    // GIVEN a couple of constants, including FB instances and class-instances
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK MyFb
            ;
        END_FUNCTION_BLOCK

        CLASS cls
            METHOD testMethod : INT
                VAR_INPUT myMethodArg : INT; END_VAR
                testMethod := 1;
            END_METHOD
        END_CLASS

        VAR_GLOBAL CONSTANT
            x : INT := 1;
            y : MyFb;
            z : cls;
        END_VAR
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn sized_varargs_require_type() {
    // GIVEN a function with a untyped sized variadic argument
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION f_with_var : INT
        VAR_INPUT
            in   : INT;
            args : {sized}...;
        END_VAR
        END_FUNCTION
      ",
    );

    assert_snapshot!(&diagnostics);
}

mod overflows {
    use crate::test_utils::tests::parse_and_validate_buffered;
    use insta::assert_snapshot;

    #[test]
    fn overflows_with_literals() {
        let diagnostics = parse_and_validate_buffered(
            "
        FUNCTION main : DINT
            VAR
                // 8
                min_sint    : SINT  := -129;    // -128
                max_sint    : SINT  :=  128;    //  127
                min_usint   : USINT  := -1;     // 0
                max_usint   : USINT  := 257;    // 256

                // 16
                min_int     : INT  := -32_769;  // -32768
                max_int     : INT  := 32_768;   //  32767
                min_uint    : UINT  := -1;      // 0
                max_uint    : UINT  := 65_537;  // 65536

                // 32
                min_dint    : DINT  := -2_147_483_649;  // -2_147_483_648
                max_dint    : DINT  :=  2_147_483_648;  //  2_147_483_647
                min_udint   : UDINT  := -1;             // 0
                max_udint   : UDINT  := 4_294_967_296;  // 4_294_967_296

                // 64
                min_lint    : LINT  := -9_223_372_036_854_775_809;  // -9_223_372_036_854_775_808
                max_lint    : LINT  :=  9_223_372_036_854_775_808;  //  9_223_372_036_854_775_807
                min_ulint   : ULINT  := -1;                         // 0
                max_ulint   : ULINT  := 18_446_744_073_709_551_616; // 18_446_744_073_709_551_615

                // f32
                min_real : REAL := -3.50282347E+38; // -3.40282347E+38
                max_real : REAL :=  3.50282347E+38; //  3.40282347E+38

                // f64
                min_lreal : LREAL := -1.8076931348623157E+308; // -1.7976931348623157E+308
                max_lreal : LREAL :=  1.8076931348623157E+308; //  1.7976931348623157E+308
            END_VAR
        END_FUNCTION
        ",
        );

        assert_snapshot!(diagnostics);
    }

    #[test]
    fn overflows_with_expressions() {
        let diagnostics = parse_and_validate_buffered(
            "
        FUNCTION main : DINT
            VAR
                // 8
                min_sint    : SINT  := ((-128 * 1) * 2);    // -128
                max_sint    : SINT  :=  ((127 * 1) * 2);    //  127
                min_usint   : USINT  := ((1 * 1) * -2);     // 0
                max_usint   : USINT  := ((256 * 1) * 2);    // 256

                // 16
                min_int     : INT  := ((-32_768 * 1) * 2);   // -32768
                max_int     : INT  :=  ((32_767 * 1) * 2);   //  32767
                min_uint    : UINT  := ((1 * 1) * -2);      // 0
                max_uint    : UINT  := ((65_536 * 1) * 2);  // 65536

                // 32
                min_dint    : DINT  := ((-2_147_483_649 * 1) * 2);  // -2_147_483_648
                max_dint    : DINT  := (( 2_147_483_648 * 1) * 2);  //  2_147_483_647
                min_udint   : UDINT  := ((1 * 1) * -2);             // 0
                max_udint   : UDINT  := ((4_294_967_296 * 1) * 2);  // 4_294_967_296

                // 64
                min_lint    : LINT  := ((-9_223_372_036_854_775_808 * 1) * 2);  // -9_223_372_036_854_775_808
                max_lint    : LINT  := (( 9_223_372_036_854_775_807 * 1) * 2);  //  9_223_372_036_854_775_807
                min_ulint   : ULINT  := ((1 * 1) * -2);                         // 0
                max_ulint   : ULINT  := ((18_446_744_073_709_551_615 * 1) * 2); // 18_446_744_073_709_551_615

                // f32
                min_real : REAL := ((-3.40282347E+38 * 1) * 2); // -3.40282347E+38
                max_real : REAL := (( 3.40282347E+38 * 1) * 2); //  3.40282347E+38

                // f64
                min_lreal : LREAL := ((-1.7976931348623157E+308 * 1) * 2); // -1.7976931348623157E+308
                max_lreal : LREAL := (( 1.7976931348623157E+308 * 1) * 2); //  1.7976931348623157E+308
            END_VAR
        END_FUNCTION
        ",
        );

        assert_snapshot!(diagnostics);
    }

    #[test]
    fn overflows_with_globals() {
        let diagnostics = parse_and_validate_buffered(
            "
        VAR_GLOBAL
            a : INT := 32768;
            b : INT := 32767 + 1;
        END_VAR
        ",
        );

        assert_snapshot!(diagnostics);
    }

    #[test]
    fn overflows_with_aliases() {
        let diagnostics = parse_and_validate_buffered(
            "
        TYPE MyINT      : INT   := 60000; END_TYPE
        TYPE MyREAL     : REAL  := 3.50282347E+38; END_TYPE
        TYPE MyLREAL    : LREAL := 1.8076931348623157E+308; END_TYPE
        ",
        );

        assert_snapshot!(diagnostics);
    }

    #[test]
    fn overflows_with_constants() {
        let diagnostics = parse_and_validate_buffered(
            "
        VAR_GLOBAL CONSTANT
            a : INT := 16384; // OK
            b : INT := 16384; // OK
            c : INT := a + b; // Will overflow
        END_VAR
        ",
        );

        assert_snapshot!(diagnostics);
    }

    #[test]
    fn overflows_with_non_global_constants() {
        let diagnostics = parse_and_validate_buffered(
            "
        VAR_GLOBAL
            a : INT := 16384; // OK
            b : INT := 16384; // OK
            c : INT := a + b; // Will overflow
        END_VAR
        ",
        );

        // As of now we do not evaluate `c` because the variable block isn't defined to be constant.
        // If at some point we support evaluation such cases, this test should fail. See also:
        // https://github.com/PLC-lang/rusty/issues/847
        assert_snapshot!(diagnostics)
    }

    #[test]
    fn overflows_with_array_initializer() {
        // TODO(volsa): We currently only detect the first overflow value inside an array-initalizer because
        // the `evaluate_with_target_hint` method will return an error after it first detected such a value (i.e.
        // after `-1`).
        let diagnostics = parse_and_validate_buffered(
            "
        VAR_GLOBAL
            arr : ARRAY[0..5] OF UINT := [0, -1, -2, -3, -4, -5];
        END_VAR
        ",
        );

        assert_snapshot!(diagnostics)
    }

    #[test]
    fn overflows_with_not() {
        let diagnostics = parse_and_validate_buffered(
            "
        VAR_GLOBAL
            x : UINT := 1234;       // OK
            y : UINT := NOT -1234;  // Not OK (because of -1234)
        END_VAR
        ",
        );

        assert_snapshot!(diagnostics);
    }

    #[test]
    fn overflows_with_hex() {
        let diagnostics = parse_and_validate_buffered(
            "
        VAR_GLOBAL
            x : UINT := WORD#16#ffff;   // OK
            y : UINT := WORD#16#fffff;  // Not OK, should have been `ffff` not `ffff_f_`
        END_VAR
        ",
        );

        assert_snapshot!(diagnostics);
    }
}

#[test]
fn type_initializers_in_structs_are_validated() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE foo : STRUCT
            x : DINT;
        END_STRUCT END_TYPE

        TYPE MyStruct: STRUCT
            unknown_reference : foo := (xxx := 1);
            invalid_array_assignment : ARRAY[0..1] OF INT := 0;
        END_STRUCT END_TYPE
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn assignment_suggestion_for_equal_operation_with_no_effect() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM main
            VAR
                value       : DINT;
                condition   : BOOL;

                // These Should work
                arr_dint : ARRAY[0..5] OF DINT := [1 = 1, 2, 3, 4, 5 = 5];
                arr_bool : ARRAY[1..5] OF BOOL := [1 = 1, 2 = 2, 3 = 3, 4 = 4, 5 = 10];
            END_VAR

            // These should work
            value := (condition = TRUE);

            IF   condition = TRUE   THEN (* ... *) END_IF
            IF  (condition = TRUE)  THEN (* ... *) END_IF
            IF ((condition = TRUE)) THEN (* ... *) END_IF

            IF   condition = TRUE  AND  condition = TRUE    THEN (* ... *) END_IF
            IF  (condition = TRUE) AND (condition = TRUE)   THEN (* ... *) END_IF
            IF ((condition = TRUE) AND (condition = TRUE))  THEN (* ... *) END_IF

            // These should NOT work
            value = 1;
            value = condition AND condition;
            value = condition AND (condition = TRUE);

            IF TRUE THEN value = 1; END_IF
            WHILE TRUE DO value = 1; END_WHILE
        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn invalid_initial_constant_values_in_pou_variables() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL CONSTANT
            MAX_LEN : INT := 99;
        END_VAR
        VAR_GLOBAL
            LEN : DINT := MAX_LEN - 2;
        END_VAR
        PROGRAM prg
          VAR_INPUT
            my_len: INT := LEN + 4;  //cannot be evaluated at compile time!
          END_VAR
        END_PROGRAM
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E033]: Unresolved constant `my_len` variable: `LEN` is no const reference
       ┌─ <internal>:10:28
       │
    10 │             my_len: INT := LEN + 4;  //cannot be evaluated at compile time!
       │                            ^^^^^^^ Unresolved constant `my_len` variable: `LEN` is no const reference

    "###);
}

#[test]
#[ignore = "no validation for non-leap-year date literals yet"]
fn date_invalid_declaration() {
    let diagnostics = parse_and_validate_buffered(
        r#"PROGRAM prg
        VAR
          a : DATE := D#2001-02-29; (* feb29 on non-leap year should not pass *)
        END_VAR
        END_PROGRAM"#,
    );

    assert!(!diagnostics.is_empty());

    assert_snapshot!(diagnostics, @r###""###);
}

#[test]
fn var_conf_template_variable_does_not_exist() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo.qux AT %IX1.0 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E101]: Template variable `qux` does not exist
       ┌─ <internal>:15:13
       │
    15 │             main.foo.qux AT %IX1.0 : BOOL;
       │             ^^^^^^^^^^^^ Template variable `qux` does not exist

    "###);
}

#[test]
fn var_conf_config_and_template_variable_types_differ() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : DINT;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo.bar AT %IX1.0 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E001]: Config and Template variable types differ (BOOL and : DINT)
       ┌─ <internal>:15:13
       │
     4 │                 bar AT %I* : DINT;
       │                 --- see also
       ·
    15 │             main.foo.bar AT %IX1.0 : BOOL;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Config and Template variable types differ (BOOL and : DINT)

    "###);
}

#[test]
fn var_conf_config_variable_has_incomplete_address() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo.bar AT %I* : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E104]: Variables defined in a VAR_CONFIG block must have a complete address
       ┌─ <internal>:15:26
       │
    15 │             main.foo.bar AT %I* : BOOL;
       │                          ^^^^^^ Variables defined in a VAR_CONFIG block must have a complete address

    "###);
}

#[test]
fn var_conf_template_address_has_complete_address() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %IX1.0 : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo.bar AT %IX1.0 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E103]: The configured variable is not a template, overriding non-template hardware addresses is not allowed
       ┌─ <internal>:15:13
       │
     4 │                 bar AT %IX1.0 : BOOL;
       │                 --- see also
       ·
    15 │             main.foo.bar AT %IX1.0 : BOOL;
       │             ^^^^^^^^^^^^ The configured variable is not a template, overriding non-template hardware addresses is not allowed

    "###);
}

#[test]
fn var_conf_template_variable_is_no_template() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo.bar AT %IX1.0 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E102]: `foo` is missing a hardware binding
       ┌─ <internal>:4:17
       │
     4 │                 bar : BOOL;
       │                 ^^^ `foo` is missing a hardware binding
       ·
    15 │             main.foo.bar AT %IX1.0 : BOOL;
       │             ----------------------------- see also

    "###);
}

#[test]
fn unconfigured_template_variables_are_validated() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
                qux AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo.bar AT %IX1.0 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E001]: Template not configured
      ┌─ <internal>:5:17
      │
    5 │                 qux AT %I* : BOOL;
      │                 ^^^ Template not configured

    "###);
}

#[test]
fn all_array_elements_configured_causes_no_errors() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : ARRAY[0..1] OF foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo[0].bar AT %IX1.0 : BOOL;
            main.foo[1].bar AT %IX1.1 : BOOL;
        END_VAR
        "#,
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn arrays() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : ARRAY[0..1] OF foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo[0].bar AT %IX1.0 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E001]: Not all template instances in array are configured
      ┌─ <internal>:4:17
      │
    4 │                 bar AT %I* : BOOL;
      │                 ^^^ Not all template instances in array are configured

    "###);
}

#[test]
fn arrays_with_multi_dim() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : ARRAY[0..1, 2..3] OF foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo[0, 2].bar AT %IX1.0 : BOOL;
            main.foo[1, 2].bar AT %IX1.1 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E001]: Not all template instances in array are configured
      ┌─ <internal>:4:17
      │
    4 │                 bar AT %I* : BOOL;
      │                 ^^^ Not all template instances in array are configured

    error[E001]: Not all template instances in array are configured
      ┌─ <internal>:4:17
      │
    4 │                 bar AT %I* : BOOL;
      │                 ^^^ Not all template instances in array are configured

    "###);
}

#[test]
fn arrays_with_const_dim() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL CONSTANT
            START: DINT := 0;
        END_VAR

        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : ARRAY[START..1] OF foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo[START].bar AT %IX1.0 : BOOL;
            main.foo[1].bar AT %IX1.1 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E001]: Non-literal VAR_CONFIG array access is not validated
       ┌─ <internal>:19:22
       │
    19 │             main.foo[START].bar AT %IX1.0 : BOOL;
       │                      ^^^^^ Non-literal VAR_CONFIG array access is not validated

    error[E001]: Not all template instances in array are configured
      ┌─ <internal>:8:17
      │
    8 │                 bar AT %I* : BOOL;
      │                 ^^^ Not all template instances in array are configured

    "###);
}

#[test]
fn multi_dim_arrays_with_consts() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL CONSTANT
            START: DINT := 0;
        END_VAR

        FUNCTION_BLOCK foo_fb
            VAR
                bar AT %I* : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                foo : ARRAY[START..1, 0..1] OF foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo[START + 3, 0].bar AT %IX1.0 : BOOL;
            main.foo[START - 23, 1].bar AT %IX1.1 : BOOL;
            main.foo[1, START * 2].bar AT %IX1.2 : BOOL;
            main.foo[1, 1].bar AT %IX1.3 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r###"
    error[E001]: Non-literal VAR_CONFIG array access is not validated
       ┌─ <internal>:19:22
       │
    19 │             main.foo[START + 3, 0].bar AT %IX1.0 : BOOL;
       │                      ^^^^^^^^^ Non-literal VAR_CONFIG array access is not validated

    error[E001]: Non-literal VAR_CONFIG array access is not validated
       ┌─ <internal>:20:22
       │
    20 │             main.foo[START - 23, 1].bar AT %IX1.1 : BOOL;
       │                      ^^^^^^^^^^ Non-literal VAR_CONFIG array access is not validated

    error[E001]: Non-literal VAR_CONFIG array access is not validated
       ┌─ <internal>:21:25
       │
    21 │             main.foo[1, START * 2].bar AT %IX1.2 : BOOL;
       │                         ^^^^^^^^^ Non-literal VAR_CONFIG array access is not validated

    error[E001]: Not all template instances in array are configured
      ┌─ <internal>:8:17
      │
    8 │                 bar AT %I* : BOOL;
      │                 ^^^ Not all template instances in array are configured

    error[E001]: Not all template instances in array are configured
      ┌─ <internal>:8:17
      │
    8 │                 bar AT %I* : BOOL;
      │                 ^^^ Not all template instances in array are configured

    error[E001]: Not all template instances in array are configured
      ┌─ <internal>:8:17
      │
    8 │                 bar AT %I* : BOOL;
      │                 ^^^ Not all template instances in array are configured

    "###);
}
