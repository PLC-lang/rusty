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
                min_real : REAL := -3.50282347E+38; // -3.40282347E+38 // -inf is no overflow
                max_real : REAL :=  3.50282347E+38; //  3.40282347E+38 //  inf is no overflow

                // f64
                min_lreal : LREAL := -1.8076931348623157E+308; // -1.7976931348623157E+308 inf is no overflow
                max_lreal : LREAL :=  1.8076931348623157E+308; //  1.7976931348623157E+308 inf is no overflow
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
                min_real : REAL := ((-3.40282347E+38 * 1) * 2); // -inf is no overflow
                max_real : REAL := (( 3.40282347E+38 * 1) * 2); //  inf is no overflow

                // f64
                min_lreal : LREAL := ((-1.7976931348623157E+308 * 1) * 2); // -inf is no overflow
                max_lreal : LREAL := (( 1.7976931348623157E+308 * 1) * 2); //  -inf is no overflow
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
        TYPE MyREAL     : REAL  := 3.50282347E+38; END_TYPE // Not an overflow, but inf
        TYPE MyLREAL    : LREAL := 1.8076931348623157E+308; END_TYPE // Not an overflow, but inf
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

    assert_snapshot!(diagnostics, @r"
    error[E033]: Unresolved constant `my_len` variable: `LEN` is no const reference
       ┌─ <internal>:10:28
       │
    10 │             my_len: INT := LEN + 4;  //cannot be evaluated at compile time!
       │                            ^^^^^^^ Unresolved constant `my_len` variable: `LEN` is no const reference
    ");
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

    assert_snapshot!(diagnostics, @r"
    error[E107]: Template-variable must have a configuration
      ┌─ <internal>:4:17
      │
    4 │                 bar AT %I* : BOOL;
      │                 ^^^ Template-variable must have a configuration

    error[E101]: Template variable `qux` does not exist
       ┌─ <internal>:15:13
       │
    15 │             main.foo.qux AT %IX1.0 : BOOL;
       │             ^^^^^^^^^^^^ Template variable `qux` does not exist
    ");
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

    assert_snapshot!(diagnostics, @r"
    error[E001]: Config and Template variable types differ (BOOL and : DINT)
       ┌─ <internal>:15:13
       │
     4 │                 bar AT %I* : DINT;
       │                 --- see also
       ·
    15 │             main.foo.bar AT %IX1.0 : BOOL;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Config and Template variable types differ (BOOL and : DINT)
    ");
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

    assert_snapshot!(diagnostics, @r"
    error[E104]: Variables defined in a VAR_CONFIG block must have a complete address
       ┌─ <internal>:15:26
       │
    15 │             main.foo.bar AT %I* : BOOL;
       │                          ^^^^^^ Variables defined in a VAR_CONFIG block must have a complete address
    ");
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

    assert_snapshot!(diagnostics, @r"
    error[E103]: The configured variable is not a template, overriding non-template hardware addresses is not allowed
       ┌─ <internal>:15:13
       │
     4 │                 bar AT %IX1.0 : BOOL;
       │                 --- see also
       ·
    15 │             main.foo.bar AT %IX1.0 : BOOL;
       │             ^^^^^^^^^^^^ The configured variable is not a template, overriding non-template hardware addresses is not allowed
    ");
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

    assert_snapshot!(diagnostics, @r"
    error[E102]: `foo` is missing a hardware binding
       ┌─ <internal>:4:17
       │
     4 │                 bar : BOOL;
       │                 ^^^ `foo` is missing a hardware binding
       ·
    15 │             main.foo.bar AT %IX1.0 : BOOL;
       │             ----------------------------- see also
    ");
}

#[test]
fn only_constant_builtins_are_allowed_in_initializer() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            gb: BOOL;
            gb2: BOOL;
        END_VAR

        {constant}
        FUNCTION AlwaysTrue : BOOL
            AlwaysTrue := TRUE;
        END_FUNCTION

        FUNCTION Negate : BOOL
        VAR_INPUT
            in: BOOL;
        END_VAR
            Negate := NOT in;
        END_FUNCTION

        FUNCTION_BLOCK foo
            VAR
                bar : REF_TO BOOL := REF(gb); // OK
                qux : BOOL := AlwaysTrue(); // is const but no builtin, should err
                quux : BOOL := Negate(gb); // Should err
                corge : LWORD := ADR(gb); // OK
                grault : BOOL := SEL(TRUE, gb, gb2); // is builtin but no const, should err
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @"
    error[E105]: Pragma {constant} is not allowed in POU declarations
      ┌─ <internal>:7:9
      │  
    7 │ ╭         {constant}
    8 │ │         FUNCTION AlwaysTrue : BOOL
      │ ╰────────────────^ Pragma {constant} is not allowed in POU declarations

    error[E033]: Unresolved constant `qux` variable: Call-statement 'AlwaysTrue' in initializer is not constant.
       ┌─ <internal>:22:31
       │
    22 │                 qux : BOOL := AlwaysTrue(); // is const but no builtin, should err
       │                               ^^^^^^^^^^^^^ Unresolved constant `qux` variable: Call-statement 'AlwaysTrue' in initializer is not constant.

    error[E033]: Unresolved constant `quux` variable: Call-statement 'Negate' in initializer is not constant.
       ┌─ <internal>:23:32
       │
    23 │                 quux : BOOL := Negate(gb); // Should err
       │                                ^^^^^^^^^^ Unresolved constant `quux` variable: Call-statement 'Negate' in initializer is not constant.

    error[E033]: Unresolved constant `grault` variable: Call-statement 'SEL' in initializer is not constant.
       ┌─ <internal>:25:34
       │
    25 │                 grault : BOOL := SEL(TRUE, gb, gb2); // is builtin but no const, should err
       │                                  ^^^^^^^^^^^^^^^^^^ Unresolved constant `grault` variable: Call-statement 'SEL' in initializer is not constant.
    ");
}

#[test]
fn unresolved_references_to_const_builtins_in_initializer_are_reported() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo
            VAR
                bar : REF_TO BOOL := REF(gb); // unresolved reference to gb
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E048]: Could not resolve reference to gb
      ┌─ <internal>:4:42
      │
    4 │                 bar : REF_TO BOOL := REF(gb); // unresolved reference to gb
      │                                          ^^ Could not resolve reference to gb
    ");
}

#[test]
fn unknown_types_are_reported() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            a: undefined;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Unknown type: undefined
      ┌─ <internal>:3:16
      │
    3 │             a: undefined;
      │                ^^^^^^^^^ Unknown type: undefined
    ");
}

#[test]
fn aliases_are_not_reported_as_unknown_types() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        TYPE myDeclaredType : DINT; END_TYPE
        VAR_GLOBAL
            a: myDeclaredType;
        END_VAR
        "#,
    );

    assert!(diagnostics.is_empty())
}

#[test]
fn aliasing_to_undeclared_type_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        TYPE myDeclaredType : undeclaredType; END_TYPE
        VAR_GLOBAL
            a: myDeclaredType;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Unknown type: myDeclaredType
      ┌─ <internal>:4:16
      │
    4 │             a: myDeclaredType;
      │                ^^^^^^^^^^^^^^ Unknown type: myDeclaredType
    ");
}

#[test]
fn trying_to_initialize_a_pointer_of_unknown_type_is_reported() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            a: undefined;
        END_VAR
        FUNCTION_BLOCK foo
        VAR
            bar : REF_TO undefined := REF(a);
        END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E052]: Unknown type: undefined
      ┌─ <internal>:7:26
      │
    7 │             bar : REF_TO undefined := REF(a);
      │                          ^^^^^^^^^ Unknown type: undefined

    error[E052]: Unknown type: undefined
      ┌─ <internal>:3:16
      │
    3 │             a: undefined;
      │                ^^^^^^^^^ Unknown type: undefined
    ");
}

#[test]
fn trying_to_initialize_a_pointer_with_builtin_ref_with_type_mismatch_leads_to_error() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            a: DINT;
        END_VAR
        FUNCTION_BLOCK foo
        VAR
            bar : REF_TO STRING := REF(a);
        END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E037]: Invalid assignment: cannot assign 'DINT' to 'REF_TO STRING := REF(a)'
      ┌─ <internal>:7:36
      │
    7 │             bar : REF_TO STRING := REF(a);
      │                                    ^^^^^^ Invalid assignment: cannot assign 'DINT' to 'REF_TO STRING := REF(a)'
    ");
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

    assert_snapshot!(diagnostics, @r"
    error[E107]: Template-variable must have a configuration
      ┌─ <internal>:5:17
      │
    5 │                 qux AT %I* : BOOL;
      │                 ^^^ Template-variable must have a configuration
    ");
}

#[test]
fn variable_configured_multiple_times() {
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
            main.foo.bar AT %IX1.0 : BOOL;
            main.foo.bar AT %IX1.1 : BOOL;
            main.foo.bar AT %IX1.2 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E108]: Template variable configured multiple times
       ┌─ <internal>:15:13
       │
    15 │             main.foo.bar AT %IX1.0 : BOOL;
       │             ^^^^^^^^^^^^ Template variable configured multiple times
    16 │             main.foo.bar AT %IX1.1 : BOOL;
       │             ------------ see also
    17 │             main.foo.bar AT %IX1.2 : BOOL;
       │             ------------ see also
    ");
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
                bar : ARRAY[0..1, 0..1] OF foo_fb;
            END_VAR
        END_PROGRAM

        VAR_CONFIG
            main.foo[0].bar AT %IX1.0 : BOOL;
            main.foo[1].bar AT %IX1.1 : BOOL;
            main.bar[0, 0].bar AT %IX1.2 : BOOL;
            main.bar[0, 1].bar AT %IX1.3 : BOOL;
            main.bar[1, 0].bar AT %IX1.4 : BOOL;
            main.bar[1, 1].bar AT %IX1.5 : BOOL;
        END_VAR
        "#,
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn missing_array_elements_are_reported() {
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

    assert_snapshot!(diagnostics, @r"
    error[E107]: One or more template-elements in array have not been configured
      ┌─ <internal>:4:17
      │
    4 │                 bar AT %I* : BOOL;
      │                 ^^^ One or more template-elements in array have not been configured
    ");
}

#[test]
fn missing_configurations_in_arrays_with_multiple_dimensions_are_validated() {
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

    assert_snapshot!(diagnostics, @r"
    error[E107]: One or more template-elements in array have not been configured
      ┌─ <internal>:4:17
      │
    4 │                 bar AT %I* : BOOL;
      │                 ^^^ One or more template-elements in array have not been configured
    ");
}

#[test]
fn arrays_with_const_expr_access_cause_errors() {
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

    assert_snapshot!(diagnostics, @r"
    error[E001]: VAR_CONFIG array access must be a literal integer
       ┌─ <internal>:19:22
       │
    19 │             main.foo[START].bar AT %IX1.0 : BOOL;
       │                      ^^^^^ VAR_CONFIG array access must be a literal integer

    error[E107]: One or more template-elements in array have not been configured
      ┌─ <internal>:8:17
      │
    8 │                 bar AT %I* : BOOL;
      │                 ^^^ One or more template-elements in array have not been configured
    ");
}

#[test]
fn multi_dim_arrays_with_const_expr_access_cause_errors() {
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

    assert_snapshot!(diagnostics, @r"
    error[E001]: VAR_CONFIG array access must be a literal integer
       ┌─ <internal>:19:22
       │
    19 │             main.foo[START + 3, 0].bar AT %IX1.0 : BOOL;
       │                      ^^^^^^^^^ VAR_CONFIG array access must be a literal integer

    error[E001]: VAR_CONFIG array access must be a literal integer
       ┌─ <internal>:20:22
       │
    20 │             main.foo[START - 23, 1].bar AT %IX1.1 : BOOL;
       │                      ^^^^^^^^^^ VAR_CONFIG array access must be a literal integer

    error[E001]: VAR_CONFIG array access must be a literal integer
       ┌─ <internal>:21:25
       │
    21 │             main.foo[1, START * 2].bar AT %IX1.2 : BOOL;
       │                         ^^^^^^^^^ VAR_CONFIG array access must be a literal integer

    error[E107]: One or more template-elements in array have not been configured
      ┌─ <internal>:8:17
      │
    8 │                 bar AT %I* : BOOL;
      │                 ^^^ One or more template-elements in array have not been configured
    ");
}

#[test]
fn array_access_with_non_integer_literal_causes_error() {
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
            main.foo['hello world'].bar AT %IX1.0 : BOOL;
            main.foo[1.4].bar AT %IX1.1 : BOOL;
        END_VAR
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E001]: VAR_CONFIG array access must be a literal integer
       ┌─ <internal>:15:22
       │
    15 │             main.foo['hello world'].bar AT %IX1.0 : BOOL;
       │                      ^^^^^^^^^^^^^ VAR_CONFIG array access must be a literal integer

    error[E001]: VAR_CONFIG array access must be a literal integer
       ┌─ <internal>:16:22
       │
    16 │             main.foo[1.4].bar AT %IX1.1 : BOOL;
       │                      ^^^ VAR_CONFIG array access must be a literal integer

    error[E107]: One or more template-elements in array have not been configured
      ┌─ <internal>:4:17
      │
    4 │                 bar AT %I* : BOOL;
      │                 ^^^ One or more template-elements in array have not been configured
    ");
}

#[test]
fn use_of_var_external_block_gives_a_warning() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            b: BOOL;
        END_VAR
        FUNCTION_BLOCK foo_fb
            VAR_EXTERNAL
                b : BOOL;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    warning[E106]: VAR_EXTERNAL blocks have no effect
      ┌─ <internal>:6:13
      │
    6 │             VAR_EXTERNAL
      │             ^^^^^^^^^^^^ VAR_EXTERNAL blocks have no effect
    ");
}

#[test]
fn unresolved_var_external_reference_does_not_lead_to_errors() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            a: BOOL;
        END_VAR
        FUNCTION_BLOCK foo_fb
            VAR_EXTERNAL
                b : BOOL;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    warning[E106]: VAR_EXTERNAL blocks have no effect
      ┌─ <internal>:6:13
      │
    6 │             VAR_EXTERNAL
      │             ^^^^^^^^^^^^ VAR_EXTERNAL blocks have no effect
    ");
}

#[test]
fn var_external_with_initializer_does_not_err() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL
            b: BOOL;
        END_VAR
        FUNCTION_BLOCK foo_fb
            VAR_EXTERNAL
                b : BOOL := TRUE;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    warning[E106]: VAR_EXTERNAL blocks have no effect
      ┌─ <internal>:6:13
      │
    6 │             VAR_EXTERNAL
      │             ^^^^^^^^^^^^ VAR_EXTERNAL blocks have no effect
    ");
}

#[test]
fn using_var_external_variable_without_matching_global_will_not_resolve() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo_fb
            VAR_EXTERNAL
                b : BOOL := TRUE;
            END_VAR

            b := FALSE;
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    warning[E106]: VAR_EXTERNAL blocks have no effect
      ┌─ <internal>:3:13
      │
    3 │             VAR_EXTERNAL
      │             ^^^^^^^^^^^^ VAR_EXTERNAL blocks have no effect

    error[E048]: Could not resolve reference to b
      ┌─ <internal>:7:13
      │
    7 │             b := FALSE;
      │             ^ Could not resolve reference to b
    ");
}

#[test]
fn assigning_a_temp_reference_to_stateful_var_is_error() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK foo
            VAR
                s1: REF_TO DINT := REF(t1);     // error
                s2 AT t1 : DINT;                // error
                s3 : REFERENCE TO DINT REF= t1; // error
                s4 AT s2 : DINT;                // OK
                s5 : REF_TO DINT := REF(s4);    // OK
                s6 : REFERENCE TO DINT REF= s5; // OK
            END_VAR
            VAR_TEMP
                t1 : DINT;
                t2 : REF_TO DINT := REF(t1);    // OK
                t3 AT s1 : DINT;                // OK
                t4 : REFERENCE TO DINT REF= t3; // OK
            END_VAR
        END_FUNCTION_BLOCK

        // all of these assignments are okay in a function, since they are all stack-variables
        FUNCTION bar
            VAR
                s1: REF_TO DINT := REF(t1);
                s2 AT t1 : DINT;
                s3 : REFERENCE TO DINT REF= t1;
                s4 AT s2 : DINT;
                s5 : REF_TO DINT := REF(s4);
                s6 : REFERENCE TO DINT REF= s5;
            END_VAR
            VAR_TEMP
                t1 : DINT;
                t2 : REF_TO DINT := REF(t1);
                t3 AT s1 : DINT;
                t4 : REFERENCE TO DINT REF= t3;
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E109]: Cannot assign address of temporary variable to a member-variable
      ┌─ <internal>:4:40
      │
    4 │                 s1: REF_TO DINT := REF(t1);     // error
      │                                        ^^ Cannot assign address of temporary variable to a member-variable

    error[E109]: Cannot assign address of temporary variable to a member-variable
      ┌─ <internal>:5:23
      │
    5 │                 s2 AT t1 : DINT;                // error
      │                       ^^ Cannot assign address of temporary variable to a member-variable

    error[E109]: Cannot assign address of temporary variable to a member-variable
      ┌─ <internal>:6:45
      │
    6 │                 s3 : REFERENCE TO DINT REF= t1; // error
      │                                             ^^ Cannot assign address of temporary variable to a member-variable
    ")
}

#[test]
fn output_variables_must_not_be_assignable_outside_of_their_scope() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK function_block_0
            VAR_INPUT
            END_VAR

            VAR_OUTPUT
                out1 : BOOL;
            END_VAR

            VAR
            END_VAR

            METHOD someMethod
                VAR
                END_VAR

                out1 := 1;
            END_METHOD

            PROPERTY someProperty : DINT
                GET
                    out1 := 1;
                END_GET
                SET END_SET
            END_PROPERTY

            out1 := 1;
        END_FUNCTION_BLOCK

        ACTIONS function_block_0
            ACTION SetOutputAction
                out1 := 1;
            END_ACTION
        END_ACTIONS

        PROGRAM mainProg
            VAR
                fb : function_block_0;
            END_VAR

            fb.out1 := 1;
            fb();
        END_PROGRAM
       ",
    );

    assert_snapshot!(&diagnostics, @"
    error[E037]: VAR_OUTPUT variables cannot be assigned outside of their scope.
       ┌─ <internal>:41:13
       │
    41 │             fb.out1 := 1;
       │             ^^^^^^^^^^^^ VAR_OUTPUT variables cannot be assigned outside of their scope.
    ");
}

#[test]
fn output_variables_must_be_assignable_within_the_scope_of_inheritance() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK function_block_0
            VAR_INPUT
            END_VAR

            VAR_OUTPUT
                out1 : BOOL;
            END_VAR

            VAR
            END_VAR

            ;
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK function_block_1 EXTENDS function_block_0
            VAR_INPUT
            END_VAR

            VAR_OUTPUT
                out2 : BOOL;
            END_VAR

            VAR
            END_VAR

            METHOD someMethod
                VAR
                END_VAR

                out1 := 1;
            END_METHOD

            PROPERTY someProperty : DINT
                GET
                    out1 := 1;
                END_GET
                SET END_SET
            END_PROPERTY

            out1 := 1;
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK function_block_2 EXTENDS function_block_1
            VAR_INPUT
            END_VAR

            VAR_OUTPUT
            END_VAR

            VAR
            END_VAR

            out1 := 1;
            out2 := 2;
        END_FUNCTION_BLOCK

        PROGRAM mainProg
            VAR
                fb1 : function_block_1;
                fb2 : function_block_2;
            END_VAR

            fb1();
            fb2();
        END_PROGRAM
       ",
    );

    assert_snapshot!(&diagnostics, @"");
}
