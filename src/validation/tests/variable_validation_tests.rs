use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

#[test]
fn uninitialized_constants_fall_back_to_the_default() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(diagnostics, vec![]);
}

#[test]
fn unresolvable_variables_are_reported() {
    let diagnostics = parse_and_validate(
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

    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn constant_on_illegal_var_blocks_cause_validation_issue() {
    // GIVEN different variable block types with the CONSTANT modifier
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn constant_fb_instances_are_illegal() {
    // GIVEN a couple of constants, including FB instances and class-instances
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn sized_varargs_require_type() {
    // GIVEN a function with a untyped sized variadic argument
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        FUNCTION f_with_var : INT
        VAR_INPUT
            in   : INT;
            args : {sized}...;
        END_VAR
        END_FUNCTION
      ",
    );

    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn overflow_with_literals() {
    let diagnostics = parse_and_validate(
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

    assert_validation_snapshot!(diagnostics);
}

#[test]
fn overflow_with_expression() {
    let diagnostics = parse_and_validate(
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

    assert_validation_snapshot!(diagnostics);
}

#[test]
fn overflow_with_global_literals_and_expressions() {
    let diagnostics = parse_and_validate(
        "
        VAR_GLOBAL
            a : INT := 65537;
            b : INT := 65536 + 1;
        END_VAR
        ",
    );

    assert_validation_snapshot!(diagnostics);
}
