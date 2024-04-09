use insta::assert_snapshot;

use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn assign_pointer_to_too_small_type_result_in_an_error() {
    //GIVEN assignment statements to DWORD
    //WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM FOO
            VAR
                ptr : REF_TO INT;
                address : DWORD;
            END_VAR

            address := 16#DEAD_BEEF;
            address := ptr;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN assignment with different type sizes are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn assign_too_small_type_to_pointer_result_in_an_error() {
    //GIVEN assignment statements to pointer
    //WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM FOO
            VAR
                ptr : REF_TO INT;
                address : DWORD;
            END_VAR

            address := 16#DEAD_BEEF;
            ptr := address;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN assignment with different type sizes are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn assign_pointer_to_lword() {
    //GIVEN assignment statements to lword
    //WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM FOO
            VAR
                ptr : REF_TO INT;
                address : LWORD;
            END_VAR

            address := 16#DEAD_BEEF;
            address := ptr;
        END_PROGRAM
        ",
    );

    //THEN every assignment is valid
    assert!(diagnostics.is_empty());
}

#[test]
fn assignment_to_constants_result_in_an_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        VAR_GLOBAL CONSTANT
            ci: INT := 1;
        END_VAR

        VAR_GLOBAL
            i : INT;
        END_VAR

        PROGRAM prg
            VAR CONSTANT
                cl : INT := 1;
            END_VAR

            VAR
                l : INT := 1;
            END_VAR

            l   := 7;
            cl  := 4;
            i   := 1;
            ci  := 4;
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn assignment_to_enum_literals_results_in_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE Color: (red, yellow, green); END_TYPE

        VAR_GLOBAL
            g_enum: (A, B, C);
        END_VAR

        PROGRAM prg
            VAR
                state: (OPEN, CLOSED);
            END_VAR

            OPEN := 3;
            B := A;
            red := green;
       END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn invalid_char_assignments() {
    // GIVEN invalid assignments to CHAR/WCHAR
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM mainProg
        VAR
            c : CHAR;
            c2 : CHAR;
            wc : WCHAR;
            wc2 : WCHAR;
            i : INT;
            s : STRING;
        END_VAR
            c := 'AJK%&/231'; // invalid
            wc := "898JKAN"; // invalid

            c := wc; // invalid
            wc := c; // invalid

            i := 54;
            c := i; // invalid
            c := 42; // invalid

            s := 'ABC';
            c := s; // invalid
            wc := s; // invalid

            i := c; // invalid
            s := c; // invalid

            c := 'A';
            c2 := 'B';
            c := c2;

            wc := "A";
            wc2 := "B";
            wc := wc2;
        END_PROGRAM"#,
    );

    // THEN every assignment should be reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn missing_string_compare_function_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM prg
            'a' =  'b'; // missing compare function :-(
            'a' <> 'b'; // missing compare function :-(
            'a' <  'b'; // missing compare function :-(
            'a' >  'b'; // missing compare function :-(
            'a' <= 'b'; // missing compare function :-(
            'a' >= 'b'; // missing compare function :-(
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn string_compare_function_cause_no_error_if_functions_exist() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION STRING_EQUAL : BOOL VAR_INPUT a,b : STRING; END_VAR END_FUNCTION
        FUNCTION STRING_GREATER : BOOL VAR_INPUT a,b : STRING; END_VAR END_FUNCTION
        FUNCTION STRING_LESS : BOOL VAR_INPUT a,b : STRING; END_VAR END_FUNCTION

        PROGRAM prg
            'a' =  'b'; // missing compare function :-(
            'a' <> 'b'; // missing compare function :-(
            'a' <  'b'; // missing compare function :-(
            'a' >  'b'; // missing compare function :-(
            'a' <= 'b'; // missing compare function :-(
            'a' >= 'b'; // missing compare function :-(
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert!(diagnostics.is_empty());
}

#[test]
fn string_compare_function_with_wrong_signature_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION STRING_EQUAL : BOOL VAR_INPUT a : STRING; END_VAR END_FUNCTION

        PROGRAM prg
            'a' =  'b'; // missing compare function :-(
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn missing_wstring_compare_function_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prg
            "a" =  "b"; // missing compare function :-(
            "a" <> "b"; // missing compare function :-(
            "a" <  "b"; // missing compare function :-(
            "a" >  "b"; // missing compare function :-(
            "a" <= "b"; // missing compare function :-(
            "a" >= "b"; // missing compare function :-(
        END_PROGRAM
      "#,
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn wstring_compare_function_cause_no_error_if_functions_exist() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION WSTRING_EQUAL : BOOL VAR_INPUT a,b : WSTRING; END_VAR END_FUNCTION
        FUNCTION WSTRING_GREATER : BOOL VAR_INPUT a,b : WSTRING; END_VAR END_FUNCTION
        FUNCTION WSTRING_LESS : BOOL VAR_INPUT a,b : WSTRING; END_VAR END_FUNCTION

        PROGRAM prg
            "a" =  "b"; // missing compare function :-(
            "a" <> "b"; // missing compare function :-(
            "a" <  "b"; // missing compare function :-(
            "a" >  "b"; // missing compare function :-(
            "a" <= "b"; // missing compare function :-(
            "a" >= "b"; // missing compare function :-(
        END_PROGRAM
      "#,
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert!(diagnostics.is_empty());
}

#[test]
fn switch_case() {
    // GIVEN switch case statement
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL CONSTANT
            BASE : DINT := 1;
        END_VAR

        TYPE myType: ( MYTYPE_A := BASE+1 ); END_TYPE

        PROGRAM prog
        VAR
            input, res : DINT;
        END_VAR

            CASE input OF
                BASE:
                    res := 1;
                MYTYPE_A:
                    res := 2;
                MYTYPE_A+1:
                    res := 3;
                4:
                    res := 4;
                2*2+1:
                    res := 5;
        (BASE*5)..(BASE*10):
                    res := 6;
            END_CASE
        END_PROGRAM
      "#,
    );

    // THEN no errors should occure
    assert!(diagnostics.is_empty());
}

#[test]
fn switch_case_duplicate_integer_non_const_var_reference() {
    // GIVEN switch case with non constant variables
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL CONSTANT
            CONST : DINT := 8;
        END_VAR

        PROGRAM prog
        VAR
            input, res, x, y : DINT;
        END_VAR
            x := 2;
            y := x;

            CASE input OF
                x: // x is no constant => error
                    res := 1;
                y: // y is no constant => error
                    res := 2;
                2+x: // x is no constant => error
                    res := 3;
                CONST:
                    res := 4;
                CONST+x: // x is no constant => error
                    res := 5;
            END_CASE
        END_PROGRAM
      "#,
    );

    // THEN the non constant variables are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn switch_case_duplicate_integer() {
    // GIVEN switch case with duplicate constant conditions
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        VAR_GLOBAL CONSTANT
            BASE : DINT := 2;
            GLOB : DINT := 2;
        END_VAR

        TYPE myType: ( MYTYPE_A := BASE*2 ); END_TYPE

        PROGRAM prog
        VAR
            input, res : DINT;
        END_VAR
            CASE input OF
                4:
                    res := 1;
                BASE*2:
                    res := 2;
                BASE+GLOB:
                    res := 3;
                MYTYPE_A:
                    res := 4;
                2+2:
                    res := 5;
            END_CASE
        END_PROGRAM
      "#,
    );

    // THEN the non constant variables are reported
    assert_snapshot!(&diagnostics);
}

#[test]
fn switch_case_invalid_case_conditions() {
    // GIVEN switch case statement
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION foo : DINT
        END_FUNCTION

        PROGRAM main
        VAR
            input, res : DINT;
        END_VAR

            CASE input OF
                foo():
                    res := 1;
                res := 2:
                    res := 2;
            END_CASE
        END_PROGRAM
      "#,
    );

    // THEN
    assert_snapshot!(&diagnostics);
}

#[test]
fn case_condition_used_outside_case_statement() {
    // GIVEN switch case statement
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM main
        VAR
            var1 : TOD;
        END_VAR
            var1 := TOD#20:15:30:123;
            23:
            var1 := TOD#20:15:30;
        END_PROGRAM
      "#,
    );

    // THEN
    assert_snapshot!(&diagnostics);
}

#[test]
fn subrange_compare_function_causes_no_error() {
    // GIVEN comparison of subranges
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM main
        VAR
            a, b, c, d, e, f : BOOL;
        END_VAR
        VAR_TEMP
            x,y : INT(0..500);
        END_VAR
            a := x < y;
            b := x = y;
            c := x = 3;
            d := y = 500;
            e := x >= 0 AND x <= 500;
            f := x < 0 OR x > 500;
        END_PROGRAM
        "#,
    );

    // THEN the validator does not throw an error
    assert!(diagnostics.is_empty());
}

#[test]
fn aliased_subrange_compare_function_causes_no_error() {
    // GIVEN comparison of aliased subranges
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        TYPE MyInt: INT(0..500); END_TYPE
        PROGRAM main
        VAR
            a, b, c, d, e, f : BOOL;
        END_VAR
        VAR_TEMP
            x,y : MyInt;
        END_VAR
            a := x < y;
            b := x = y;
            c := x = 3;
            d := y = 500;
            e := x >= 0 AND x <= 500;
            f := x < 0 OR x > 500;
        END_PROGRAM
        "#,
    );

    // THEN the validator does not throw an error
    assert!(diagnostics.is_empty());
}

#[test]
fn aliased_int_compare_function_causes_no_error() {
    // GIVEN comparison of aliased integers
    // WHEN it is validated
    let diagnostics = parse_and_validate_buffered(
        r#"
        TYPE MyInt: INT; END_TYPE
        PROGRAM main
        VAR
            a, b, c, d, e, f : BOOL;
        END_VAR
        VAR_TEMP
            x,y : MyInt;
        END_VAR
            a := x < y;
            b := x = y;
            c := x = 3;
            d := y = 500;
            e := x >= 0 AND x <= 500;
            f := x < 0 OR x > 500;
        END_PROGRAM
        "#,
    );

    // THEN the validator does not throw an error
    assert!(diagnostics.is_empty());
}

#[test]
fn program_missing_inout_assignment() {
    // GIVEN
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(input1 := var1, output1 => var2);
            prog(var1, var2);
            prog(var1);
            prog();
        END_PROGRAM
        ",
    );
    // THEN
    assert_snapshot!(&diagnostics);
}

#[test]
fn function_call_parameter_validation() {
    // GIVEN
    // WHEN
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1 : DINT;
            var2 : STRING;
            var3 : REF_TO WSTRING;
            var4 : REAL;
        END_VAR
            foo(input1 := var1, inout1 := var1, output1 => var1); // valid

            foo(output1 => var1, var1, var1); // invalid cannot mix explicit and implicit

            foo(input1 := var2, inout1 := var3, output1 => var4); // invalid types assigned
            //                                  ^^^^^^^^^^^^^^^ REAL assignment to DINT is valid
            foo(var2, var3, var4); // invalid types assigned
            //              ^^^^ REAL assignment to DINT is valid
        END_PROGRAM
        "#,
    );

    // THEN
    assert_snapshot!(&diagnostics);
}

#[test]
fn program_call_parameter_validation() {
    // GIVEN
    // WHEN
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1 : DINT;
            var2 : STRING;
            var3 : REF_TO WSTRING;
            var4 : REAL;
        END_VAR
            prog(input1 := var1, inout1 := var1, output1 => var1); // valid

            prog(output1 => var1, var1, var1); // invalid cannot mix explicit and implicit

            prog(input1 := var2, inout1 := var3, output1 => var4); // invalid types assigned
            //                                   ^^^^^^^^^^^^^^^ REAL assignment to DINT is valid
            prog(var2, var3, var4); // invalid types assigned
            //               ^^^^ REAL assignment to DINT is valid
        END_PROGRAM
        "#,
    );

    // THEN
    assert_snapshot!(&diagnostics);
}

#[test]
fn reference_to_reference_assignments_in_function_arguments() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    VAR_GLOBAL
        global1 : STRUCT_params;
        global2 : STRUCT_params;
        global3 : STRUCT_params;

        global4 : INT := 1;
        global5 : REAL := 1.1;
        global6 : String[6] := 'foobar';
    END_VAR

    TYPE STRUCT_params :
        STRUCT
            param1 : BOOL;
            param2 : BOOL;
            param3 : BOOL;
        END_STRUCT
    END_TYPE

    PROGRAM prog
        VAR_INPUT
            input1 : REF_TO STRUCT_params;
            input2 : REF_TO STRUCT_params;
            input3 : REF_TO STRUCT_params;
        END_VAR
    END_PROGRAM

    PROGRAM main
        prog(
            // ALL of these should be valid
            input1 := ADR(global1),
            input2 := REF(global2),
            input3 := &global3
        );

        prog(
            // ALL of these should be valid because ADR(...) returns no type information and instead
            // only a LWORD is returned which we allow to be assigned to any pointer
            input1 := ADR(global4),
            input2 := ADR(global5),
            input3 := ADR(global6),
        );

        prog(
            // NONE of these should be valid because REF(...) returns type information and we
            // explicitly check if pointer assignments are of the same type
            input1 := REF(global4),
            input2 := REF(global5),
            input3 := REF(global6),
        );

        prog(
            // NONE of these should be valid because &(...) returns type information and we
            // explicitly check if pointer assignments are of the same type
            input1 := &(global4),
            input2 := &(global5),
            input3 := &(global6),
        );
    END_PROGRAM
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn ref_builtin_function_reports_invalid_param_count() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            x: ARRAY[0..1] OF INT;
        END_VAR
            REF(x); // valid
            REF();
            REF(x, 1, 2, 'abc');
        END_FUNCTION
    ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn address_of_operations() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE MyStruct: STRUCT
            a : SubStruct;
        END_STRUCT
        END_TYPE

        TYPE SubStruct: STRUCT
            b : INT;
        END_STRUCT
        END_TYPE

        PROGRAM main
            VAR
                a: INT;
                b: ARRAY[0..5] OF INT;
                c: MyStruct;
            END_VAR

            // Should work
            &(a);
            &b[1];
            &c.a.b;

            // Should not work
            &&a;
            &100;
            &(a+3);
        END_PROGRAM
        ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn validate_call_by_ref() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION func : DINT
            VAR_INPUT
                byValInput : INT;
            END_VAR

            VAR_IN_OUT
                byRefInOut : INT;
            END_VAR

            VAR_OUTPUT
                byRefOutput : INT;
            END_VAR
        END_FUNCTION

        PROGRAM main
            VAR
                x : INT := 1;
            END_VAR

            // The second and third arguments are expected to be references, as such
            // any call to `func` where these two arguments are literals will fail
            func(1, 2, 3);
            func(1, 2, x);
            func(1, x, 3);
            func(1, x, x); // Valid
            func(x, 2, 3);
            func(x, 2, x);
            func(x, x, 3);
            func(x, x, x); // Valid
        END_PROGRAM
        ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn validate_call_by_ref_explicit() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION func : DINT
            VAR_INPUT
                byValInput : INT;
            END_VAR

            VAR_IN_OUT
                byRefInOut : INT;
            END_VAR

            VAR_OUTPUT
                byRefOutput : INT;
            END_VAR
        END_FUNCTION

        PROGRAM main
            VAR
                x : INT := 1;
            END_VAR

            // The second and third arguments are expected to be references, as such
            // any call to `func` where these two arguments are literals will fail
            func(byValInput := 1, byRefInOut := 2, byRefOutput =>  );
            func(byValInput := 1, byRefInOut := x, byRefOutput =>  ); // Valid (Output assignments are optional)
            func(byValInput := 1, byRefInOut := 2, byRefOutput => 3);
            func(byValInput := 1, byRefInOut := 2, byRefOutput => x);
            func(byValInput := 1, byRefInOut := x, byRefOutput => 3);
            func(byValInput := 1, byRefInOut := x, byRefOutput => x); // Valid
        END_PROGRAM
        ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn exlicit_param_unknown_reference() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK func
            VAR_INPUT
                byValInput : INT;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                fb: func;
            END_VAR

            fb(unknown := 2);
            fb(byVALInput := 2); //different case but valid
            fb(byValInput := 2); //valid

        END_PROGRAM
        ",
    );
    assert_snapshot!(diagnostics)
}

#[test]
fn exlicit_param_different_casing() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK func
            VAR_INPUT
                IN : INT;
                IN2 : INT;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                fb: func;
            END_VAR

            fb(in := 2);
            fb(in := 2, IN2 := 3);
            fb(IN := 2, in2 := 3);
            fb(in := 2, in2 := 3);
        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn implicit_param_downcast_in_function_call() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM main
        VAR
            var1_lint, var2_lint : LINT;
            var_lreal            : LREAL;
            var_lword            : LWORD;
            // trying to implicitly cast arrays gives an invalid assignment error, it shouldn't also give a downcast warning
            var_in_out_wstr       : WSTRING;
            var_arr              : ARRAY[1..3] OF LINT;
        END_VAR
            foo(
                var1_lint, // downcast
                var_lword, // downcast
                var_lreal, // downcast
                INT#var1_lint, // downcast
                var2_lint, // downcast
                var_in_out_wstr, // invalid
                var1_lint // downcast
            );
        END_PROGRAM

        FUNCTION foo : DINT
        VAR_INPUT {ref}
            in_ref_int      : INT;
            in_ref_dword    : DWORD;
        END_VAR
        VAR_INPUT
            in_real         : REAL;
            in_sint         : SINT;
        END_VAR
        VAR_IN_OUT
            in_out          : INT;
            in_out_str      : STRING;
        END_VAR
        VAR_OUTPUT
            out_var         : DINT;
        END_VAR
        END_FUNCTION
        ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn function_block_implicit_downcast() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM main
        VAR
            fb: fb_t;
            var1_lint, var2_lint : LINT;
            var_real             : REAL;
            var_lword            : LWORD;
            var_wstr             : WSTRING;
        END_VAR
            fb(
                var1_lint, // downcast
                var_lword, // downcast
                var_real, // ok
                INT#var1_lint, // downcast
                var2_lint, // downcast
                var_wstr, // invalid
                var1_lint // downcast
            );
        END_PROGRAM

        FUNCTION_BLOCK fb_t
        VAR_INPUT {ref}
            in_ref_int      : INT;
            in_ref_dword    : DWORD;
        END_VAR
        VAR_INPUT
            in_real         : LREAL;
            in_sint         : SINT;
        END_VAR
        VAR_IN_OUT
            in_out          : INT;
            in_out_arr      : STRING;
        END_VAR
        VAR_OUTPUT
            out_var         : DINT;
        END_VAR
        END_FUNCTION_BLOCK
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn program_implicit_downcast() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM main
        VAR
            fb: fb_t;
            var1_lint, var2_lint : LINT;
            var_real             : REAL;
            var_lword            : LWORD;
            var_wstr             : WSTRING;
        END_VAR
            prog(
                var1_lint, // downcast
                var_lword, // downcast
                var_real, // ok
                INT#var1_lint, // downcast
                var2_lint, // downcast
                var_wstr, // invalid
                var1_lint // downcast
            );
        END_PROGRAM

        PROGRAM prog
        VAR_INPUT {ref}
            in_ref_int      : INT;
            in_ref_dword    : DWORD;
        END_VAR
        VAR_INPUT
            in_real         : LREAL;
            in_sint         : SINT;
        END_VAR
        VAR_IN_OUT
            in_out          : INT;
            in_out_arr      : STRING;
        END_VAR
        VAR_OUTPUT
            out_var         : DINT;
        END_VAR
        END_PROGRAM
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn action_implicit_downcast() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM main
        VAR
            var_lint : LINT;
            var_wstr : WSTRING;
            var_arr  : ARRAY[1..3] OF LINT;
            fb       : fb_t;
        END_VAR
            fb.foo(var_lint, var_wstr); // downcast, invalid
            prog.bar(var_lint, var_arr); // downcast, invalid
        END_PROGRAM

        FUNCTION_BLOCK fb_t
        VAR_INPUT
            in1 : DINT;
            in2 : STRING;
        END_VAR
        END_FUNCTION_BLOCK

        ACTIONS fb_t
        ACTION foo
        END_ACTION
        END_ACTIONS

        PROGRAM prog
        VAR_INPUT
            in1 : INT;
            in2 : STRING;
        END_VAR
        END_PROGRAM

        ACTIONS prog
        ACTION bar
        END_ACTION
        END_ACTIONS
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn method_implicit_downcast() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    PROGRAM main
    VAR
        cl : MyClass;
        var_lint : LINT;
        var_arr : ARRAY[1..3] OF DINT;
    END_VAR
        cl.testMethod(var_lint, var_arr, ADR(var_arr)); // downcast, invalid, ok
    END_PROGRAM

    CLASS MyClass
    VAR
        x, y : DINT;
    END_VAR

    METHOD testMethod
    VAR_INPUT
        val : INT;
        arr : ARRAY[1..3] OF SINT;
        ref : REF_TO ARRAY[1..3] OF DINT;
    END_VAR
    END_METHOD
    END_CLASS
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn validate_array_elements_passed_to_functions_by_ref() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION func : DINT
            VAR_IN_OUT
                byRefInOut : INT;
            END_VAR

            VAR_OUTPUT
                byRefOutput : INT;
            END_VAR
        END_FUNCTION

        PROGRAM main
            VAR
                x : ARRAY[0..1] OF INT;
            END_VAR

            func(x, x);                                    // Invalid because we pass a whole array
            func(x[0], x[1]);                              // Valid because we pass a variable by array access
            func(byRefInOut := x[0], byRefOutput := x[1]); // Valid because we pass a variable by array access
        END_PROGRAM
        ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn validate_arrays_passed_to_functions() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION func : DINT
            VAR_INPUT
                arr_dint  : ARRAY[0..1] OF DINT;
            END_VAR
        END_FUNCTION

        PROGRAM main
            VAR
                arr_sint   : ARRAY[0..1] OF   SINT;
                arr_int    : ARRAY[0..1] OF    INT;
                arr_dint   : ARRAY[0..1] OF   DINT;
                arr_lint   : ARRAY[0..1] OF   LINT;
                arr_real   : ARRAY[0..1] OF   REAL;
                arr_lreal  : ARRAY[0..1] OF  LREAL;

                arr_dint_1_2            : ARRAY[1..2]       OF DINT;
                arr_dint_3_4            : ARRAY[3..4]       OF DINT;
                arr_dint_1_10           : ARRAY[1..10]      OF DINT;
                arr_dint_10_100         : ARRAY[10..100]    OF DINT;

                arr_dint_2d : ARRAY[0..1] OF ARRAY[0..1] OF DINT;
            END_VAR

            // Check if datatypes are correctly checked; only `arr_dint` should work
            func(arr_sint);
            func(arr_int);
            func(arr_dint);
            func(arr_lint);
            func(arr_real);
            func(arr_lreal);

            // Check if dimensions are correctly checked
            func(arr_dint_1_2); // Should work (but why would you write this)
            func(arr_dint_3_4); // ^
            func(arr_dint_1_10);
            func(arr_dint_10_100);

            // Check if 2D arrays are correctly checked
            func(arr_dint_2d);
        END_PROGRAM
        ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn assigning_to_rvalue() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION func : DINT
        VAR_INPUT
            x : INT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            i : INT;
        END_VAR
            1 := 1;
            1 := i;
            func(1 := 1);
        END_PROGRAM
        "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn assigning_to_qualified_references_allowed() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM prg
        VAR_INPUT
            x : INT;
        END_VAR
        END_PROGRAM

        PROGRAM main
            prg.x := 1;
        END_PROGRAM
        "#,
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn assigning_to_rvalue_allowed_for_directaccess() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM main
        VAR
            x : INT;
        END_VAR
            x.%X1 := 1;
            x.%B1 := 1;
            x.1 := 1;
        END_PROGRAM
        "#,
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn allowed_assignable_types() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        PROGRAM main
        VAR
            v : INT;
            x : ARRAY[0..1] OF INT;
            y : REF_TO INT;
            z : REF_TO ARRAY[0..1] OF INT;
        END_VAR
            v := 0;
            x[0] := 1;
            y^ := 2;
            y^.1 := 3;
            z^[0] := 4;
            z^[1].1 := 5;
        END_PROGRAM
        "#,
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn assignment_of_incompatible_types_is_reported() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    PROGRAM prog
    VAR
        dint_ : DINT;
        string_ : STRING := 'Hello, world!';
        array_ : ARRAY[0..3] OF LWORD;
    END_VAR
        string_ := dint_;           // invalid
        string_ := array_;          // invalid
        dint_ := string_;           // invalid
        array_ := string_;          // invalid
    END_PROGRAM
    "#,
    );
    assert_snapshot!(diagnostics);
}

#[test]
fn passing_compatible_numeric_types_to_functions_is_allowed() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    PROGRAM prog
    VAR
        dint_ : DINT;
        lreal_ : LREAL;
    END_VAR
        foo(dint_);
        bar(lreal_);
    END_PROGRAM

    FUNCTION foo : DINT
    VAR_INPUT r : REAL; END_VAR
    END_FUNCTION

    FUNCTION bar : DINT
    VAR_INPUT i : LINT; END_VAR
    END_FUNCTION
    "#,
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn bit_access_with_incorrect_operator_causes_warning() {
    let diagnostics = parse_and_validate_buffered(
        "PROGRAM mainProg
        VAR_INPUT
            Input : STRUCT1;
        END_VAR
        VAR
            access : STRUCT2;
        END_VAR
        VAR_OUTPUT
            Output : STRUCT1;
        END_VAR
            Output.var1.%Wn1.%Bn1.%Xn1 := Input.var1; // OK
            Output.var1.n1             := Input.var1; // bitaccess without %X -> Warning
        END_PROGRAM

        TYPE STRUCT1 :
        STRUCT
            var1 : DWORD;
        END_STRUCT
        END_TYPE

        TYPE ENUM1 :
        (
            n1 := 1,
            n2 := 2
        );
        END_TYPE

        TYPE STRUCT2 :
        STRUCT
            var1 : BOOL;
        END_STRUCT
        END_TYPE",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn invalid_cast_statement_causes_error() {
    let diagnostics = parse_and_validate_buffered(
        "PROGRAM mainProg
            VAR_INPUT
                s : STRUCT1;
                i : INT;
            END_VAR

                i := INT#s.var1; // illegal, var1 cannot be found in INT
                i := INT#i;      // ok
                i := INT#4;      // ok
        END_PROGRAM

        TYPE STRUCT1 :
        STRUCT
            var1 : DWORD;
        END_STRUCT
        END_TYPE
       ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn for_loop_conditions_are_numerical() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM main
            VAR
                i : STRING;
                x : BOOL;
                y : DINT;
            END_VAR

        FOR i := 100000 TO x BY y DO
        END_FOR

        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics, @r###"
    error[E094]: Expected an integer value, got `STRING`
      ┌─ <internal>:9:13
      │
    9 │         FOR i := 100000 TO x BY y DO
      │             ^ Expected an integer value, got `STRING`

    error[E094]: Expected an integer value, got `BOOL`
      ┌─ <internal>:9:28
      │
    9 │         FOR i := 100000 TO x BY y DO
      │                            ^ Expected an integer value, got `BOOL`

    "###);
}

#[test]
fn for_loop_conditions_are_real_and_trigger_error() {
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM main
            VAR
                i : STRING;
                x : REAL;
                y : REAL;
            END_VAR

        FOR i := 10.0 TO x BY y DO
        END_FOR

        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics, @r###"
    error[E094]: Expected an integer value, got `STRING`
      ┌─ <internal>:9:13
      │
    9 │         FOR i := 10.0 TO x BY y DO
      │             ^ Expected an integer value, got `STRING`

    error[E094]: Expected an integer value, got `REAL`
      ┌─ <internal>:9:18
      │
    9 │         FOR i := 10.0 TO x BY y DO
      │                  ^^^^ Expected an integer value, got `REAL`

    error[E094]: Expected an integer value, got `REAL`
      ┌─ <internal>:9:26
      │
    9 │         FOR i := 10.0 TO x BY y DO
      │                          ^ Expected an integer value, got `REAL`

    error[E094]: Expected an integer value, got `REAL`
      ┌─ <internal>:9:31
      │
    9 │         FOR i := 10.0 TO x BY y DO
      │                               ^ Expected an integer value, got `REAL`

    "###);
}

#[test]
fn if_statement_triggers_error_if_condition_is_not_boolean() {
    let diagnostic = parse_and_validate_buffered(
        "
        FUNCTION main
            VAR
                x : BOOL;
                y : DINT;
                z : STRING;
            END_VAR

            IF      y THEN // Returns a warning, because we're dealing with an integer
            ELSIF   z THEN // Returns an error, because we're dealing with neither integers nor booleans

            // These are OK
            ELSIF   0 THEN
            ELSIF   1 THEN
            ELSIF   x               THEN
            ELSIF   (0 < 1)         THEN
            ELSIF   (y < 0)         THEN
            ELSIF   ((1 = 2) = 3)   THEN
            END_IF
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostic, @r###"
    warning[E096]: Expected a boolean, got `DINT`, consider adding an `=` or `<>` operator for better clarity
      ┌─ <internal>:9:21
      │
    9 │             IF      y THEN // Returns a warning, because we're dealing with an integer
      │                     ^ Expected a boolean, got `DINT`, consider adding an `=` or `<>` operator for better clarity

    error[E094]: Expected a boolean, got `STRING`
       ┌─ <internal>:10:21
       │
    10 │             ELSIF   z THEN // Returns an error, because we're dealing with neither integers nor booleans
       │                     ^ Expected a boolean, got `STRING`

    "###);
}

#[test]
fn while_loop_triggers_error_if_condition_is_not_boolean() {
    let diagnostic = parse_and_validate_buffered(
        "
        FUNCTION main
            VAR
                x : BOOL;
                y : DINT;
                z : STRING;
            END_VAR

            WHILE y DO END_WHILE // Returns a warning, because we're dealing with an integer
            WHILE z DO END_WHILE // Returns an error, because we're dealing with neither integers nor booleans

            // These are OK
            WHILE 0 DO END_WHILE
            WHILE 1 DO END_WHILE
            WHILE x             DO END_WHILE
            WHILE (0 < 1)       DO END_WHILE
            WHILE (y < 0)       DO END_WHILE
            WHILE ((1 = 2) = 3) DO END_WHILE
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostic, @r###"
    warning[E096]: Expected a boolean, got `DINT`, consider adding an `=` or `<>` operator for better clarity
      ┌─ <internal>:9:19
      │
    9 │             WHILE y DO END_WHILE // Returns a warning, because we're dealing with an integer
      │                   ^ Expected a boolean, got `DINT`, consider adding an `=` or `<>` operator for better clarity

    error[E094]: Expected a boolean, got `STRING`
       ┌─ <internal>:10:19
       │
    10 │             WHILE z DO END_WHILE // Returns an error, because we're dealing with neither integers nor booleans
       │                   ^ Expected a boolean, got `STRING`

    "###);
}

#[test]
fn action_calls_without_parentheses() {
    // Given a POU with defined actions,
    // when trying to call them without parentheses
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb1
            FOO;
            BAR;
        END_FUNCTION_BLOCK

        ACTIONS
            ACTION FOO
            END_ACTION

            ACTION BAR
            END_ACTION
        END_ACTIONS
        ",
    );

    // we expect a validation error for each "call"-statement
    assert_snapshot!(diagnostics, @r###"
    error[E095]: A reference to fb1.FOO exists, but it is an ACTION. If you meant to call it, add `()` to the statement: `fb1.FOO()`
      ┌─ <internal>:3:13
      │
    3 │             FOO;
      │             ^^^ A reference to fb1.FOO exists, but it is an ACTION. If you meant to call it, add `()` to the statement: `fb1.FOO()`

    error[E095]: A reference to fb1.BAR exists, but it is an ACTION. If you meant to call it, add `()` to the statement: `fb1.BAR()`
      ┌─ <internal>:4:13
      │
    4 │             BAR;
      │             ^^^ A reference to fb1.BAR exists, but it is an ACTION. If you meant to call it, add `()` to the statement: `fb1.BAR()`

    "###);
}

#[test]
fn action_as_reference_does_not_cause_parentheses_diagnostic() {
    // Given a POU with defined actions,
    // when getting the address of a qualified action reference
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM prog
        VAR
            fb : fb1;
            address: LWORD;
        END_VAR
            address := ADR(fb1.FOO);
        END_PROGRAM

        FUNCTION_BLOCK fb1
        END_FUNCTION_BLOCK

        ACTIONS
            ACTION FOO
            END_ACTION
        END_ACTIONS
        ",
    );

    // we expect no missing parentheses diagnostic
    assert_snapshot!(diagnostics, @r###"
    error[E037]: Invalid assignment: cannot assign 'FOO' to 'fb1'
      ┌─ <internal>:7:28
      │
    7 │             address := ADR(fb1.FOO);
      │                            ^^^^^^^ Invalid assignment: cannot assign 'FOO' to 'fb1'

    "###);
    // XXX: change assertion to `assert!(diagnostics.is_empty())` once
    // https://github.com/PLC-lang/rusty/issues/1165 is resolved
}

#[test]
fn action_assignment_attempt_does_not_report_missing_parentheses() {
    // Given a qualified action reference,
    // when trying to assign it to a variable
    let diagnostics = parse_and_validate_buffered(
        "
        PROGRAM prog
        VAR
            fb : fb1;
            address: LWORD;
        END_VAR
            address := fb1.FOO;
        END_PROGRAM

        FUNCTION_BLOCK fb1
        END_FUNCTION_BLOCK

        ACTIONS
            ACTION FOO
            END_ACTION
        END_ACTIONS
        ",
    );

    // we expect no missing parentheses diagnostic
    assert_snapshot!(diagnostics, @r###"
    error[E095]: A reference to fb1.FOO exists, but it is an ACTION. If you meant to call it, add `()` to the statement: `fb1.FOO()`
      ┌─ <internal>:7:28
      │
    7 │             address := fb1.FOO;
      │                            ^^^ A reference to fb1.FOO exists, but it is an ACTION. If you meant to call it, add `()` to the statement: `fb1.FOO()`

    error[E037]: Invalid assignment: cannot assign 'FOO' to 'LWORD'
      ┌─ <internal>:7:13
      │
    7 │             address := fb1.FOO;
      │             ^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'FOO' to 'LWORD'

    "###);
}
