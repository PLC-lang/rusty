use crate::index::VariableType;
use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

#[test]
fn assign_pointer_to_too_small_type_result_in_an_error() {
    //GIVEN assignment statements to DWORD
    //WHEN it is validated
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
        "
        PROGRAM FOO
            VAR
                ptr : POINTER TO INT;
                address : DWORD;
            END_VAR
            
            address := 16#DEAD_BEEF;              
            address := ptr;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN assignment with different type sizes are reported
    assert_eq!(
        diagnostics,
        vec![Diagnostic::incompatible_type_size("DWORD", 32, "hold a", (204..218).into()),]
    );
}

#[test]
fn assign_too_small_type_to_pointer_result_in_an_error() {
    //GIVEN assignment statements to pointer
    //WHEN it is validated
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
        "
        PROGRAM FOO
            VAR
                ptr : POINTER TO INT;
                address : DWORD;
            END_VAR
            
            address := 16#DEAD_BEEF;              
            ptr := address;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN assignment with different type sizes are reported
    assert_eq!(
        diagnostics,
        vec![Diagnostic::incompatible_type_size("DWORD", 32, "to be stored in a", (204..218).into()),]
    );
}

#[test]
fn assign_pointer_to_lword() {
    //GIVEN assignment statements to lword
    //WHEN it is validated
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
        "
        PROGRAM FOO
            VAR
                ptr : POINTER TO INT;
                address : LWORD;
            END_VAR
            
            address := 16#DEAD_BEEF;              
            address := ptr;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN every assignment is valid
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn assignment_to_constants_result_in_an_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::cannot_assign_to_constant("prg.cl", (327..329).into()),
            Diagnostic::cannot_assign_to_constant("ci", (371..373).into()),
        ]
    );
}

#[test]
fn assignment_to_enum_literals_results_in_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::cannot_assign_to_constant("__prg_state.OPEN", (230..234).into()),
            Diagnostic::cannot_assign_to_constant("__global_g_enum.B", (253..254).into()),
            Diagnostic::cannot_assign_to_constant("Color.red", (273..276).into()),
        ]
    );
}

#[test]
fn invalid_char_assignments() {
    // GIVEN invalid assignments to CHAR/WCHAR
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::syntax_error("Value: 'AJK%&/231' exceeds length for type: CHAR", (129..140).into()),
            Diagnostic::syntax_error("Value: '898JKAN' exceeds length for type: WCHAR", (162..171).into()),
            Diagnostic::invalid_assignment("WCHAR", "CHAR", (188..195).into()),
            Diagnostic::invalid_assignment("CHAR", "WCHAR", (211..218).into()),
            Diagnostic::invalid_assignment("INT", "CHAR", (247..253).into()),
            Diagnostic::invalid_assignment("DINT", "CHAR", (269..276).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (308..314).into()),
            Diagnostic::invalid_assignment("STRING", "WCHAR", (330..337).into()),
            Diagnostic::invalid_assignment("CHAR", "INT", (354..360).into()),
            Diagnostic::invalid_assignment("CHAR", "STRING", (376..382).into()),
        ]
    );
}

#[test]
fn missing_string_compare_function_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (33..43).into()),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (89..99).into()),
            Diagnostic::missing_compare_function("STRING_LESS", "STRING", (145..155).into()),
            Diagnostic::missing_compare_function("STRING_GREATER", "STRING", (201..211).into()),
            Diagnostic::missing_compare_function("STRING_LESS", "STRING", (257..267).into()),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (257..267).into()),
            Diagnostic::missing_compare_function("STRING_GREATER", "STRING", (313..323).into()),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (313..323).into()),
        ]
    );
}

#[test]
fn string_compare_function_cause_no_error_if_functions_exist() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        FUNCTION STRING_EQUAL : BOOL VAR_INPUT a,b : STRING END_VAR END_FUNCTION
        FUNCTION STRING_GREATER : BOOL VAR_INPUT a,b : STRING END_VAR END_FUNCTION
        FUNCTION STRING_LESS : BOOL VAR_INPUT a,b : STRING END_VAR END_FUNCTION

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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn string_compare_function_with_wrong_signature_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        FUNCTION STRING_EQUAL : BOOL VAR_INPUT a : STRING END_VAR END_FUNCTION

        PROGRAM prg
            'a' =  'b'; // missing compare function :-(
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (113..123).into()),]
    );
}

#[test]
fn missing_wstring_compare_function_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (33..43).into()),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (89..99).into()),
            Diagnostic::missing_compare_function("WSTRING_LESS", "WSTRING", (145..155).into()),
            Diagnostic::missing_compare_function("WSTRING_GREATER", "WSTRING", (201..211).into()),
            Diagnostic::missing_compare_function("WSTRING_LESS", "WSTRING", (257..267).into()),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (257..267).into()),
            Diagnostic::missing_compare_function("WSTRING_GREATER", "WSTRING", (313..323).into()),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (313..323).into()),
        ]
    );
}

#[test]
fn wstring_compare_function_cause_no_error_if_functions_exist() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION WSTRING_EQUAL : BOOL VAR_INPUT a,b : WSTRING END_VAR END_FUNCTION
        FUNCTION WSTRING_GREATER : BOOL VAR_INPUT a,b : WSTRING END_VAR END_FUNCTION
        FUNCTION WSTRING_LESS : BOOL VAR_INPUT a,b : WSTRING END_VAR END_FUNCTION

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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn switch_case() {
    // GIVEN switch case statement
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
		VAR_GLOBAL CONSTANT
			BASE : DINT := 1;
		END_VAR

		TYPE myType: ( MYTYPE_A := BASE+1 ); END_TYPE

        PROGRAM
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn switch_case_duplicate_integer_non_const_var_reference() {
    // GIVEN switch case with non constant variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
		VAR_GLOBAL CONSTANT
			CONST : DINT := 8;
		END_VAR

        PROGRAM
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::non_constant_case_condition("'x' is no const reference", (160..161).into()),
            Diagnostic::non_constant_case_condition("'y' is no const reference", (211..212).into()),
            Diagnostic::non_constant_case_condition("'x' is no const reference", (262..265).into()),
            Diagnostic::non_constant_case_condition("'x' is no const reference", (341..348).into())
        ]
    );
}

#[test]
fn switch_case_duplicate_integer() {
    // GIVEN switch case with duplicate constant conditions
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
		VAR_GLOBAL CONSTANT
			BASE : DINT := 2;
			GLOB : DINT := 2;
		END_VAR

		TYPE myType: ( MYTYPE_A := BASE*2 ); END_TYPE

        PROGRAM
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::duplicate_case_condition(&4, (222..228).into()),
            Diagnostic::duplicate_case_condition(&4, (249..258).into()),
            Diagnostic::duplicate_case_condition(&4, (279..287).into()),
            Diagnostic::duplicate_case_condition(&4, (308..311).into()),
        ]
    );
}

#[test]
fn switch_case_invalid_case_conditions() {
    // GIVEN switch case statement
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_case_condition((120..126).into()),
            Diagnostic::non_constant_case_condition("Cannot resolve constant: CallStatement {\n    operator: Reference {\n        name: \"foo\",\n    },\n    parameters: None,\n}", (120..126).into()),
            Diagnostic::invalid_case_condition((146..154).into()),
        ]
    );
}

#[test]
fn case_condition_used_outside_case_statement() {
    // GIVEN switch case statement
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::case_condition_used_outside_case_statement((50..70).into()),
            Diagnostic::case_condition_used_outside_case_statement((79..81).into()),
        ]
    );
}

#[test]
fn subrange_compare_function_causes_no_error() {
    // GIVEN comparison of subranges
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn aliased_subrange_compare_function_causes_no_error() {
    // GIVEN comparison of aliased subranges
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn aliased_int_compare_function_causes_no_error() {
    // GIVEN comparison of aliased integers
    // WHEN it is validated
    let diagnostics = parse_and_validate(
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
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn program_missing_inout_assignment() {
    // GIVEN
    let result = parse_and_validate(
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
    assert_eq!(
        vec![
            Diagnostic::missing_inout_parameter("inout1", (216..220).into(),),
            Diagnostic::missing_inout_parameter("inout1", (258..262).into(),),
            Diagnostic::missing_inout_parameter("inout1", (279..283).into(),),
            Diagnostic::missing_inout_parameter("inout1", (294..298).into(),)
        ],
        result
    )
}

#[test]
fn function_call_parameter_validation() {
    // GIVEN
    // WHEN
    let diagnostics = parse_and_validate(
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
			foo(var2, var3, var4); // invalid types assigned
		END_PROGRAM
        "#,
    );

    // THEN
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_parameter_type((360..364).into()),
            Diagnostic::invalid_parameter_type((366..370).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (425..439).into()),
            Diagnostic::invalid_assignment("__main_var3", "DINT", (441..455).into()),
            Diagnostic::incompatible_type_size("DINT", 32, "hold a", (441..455).into()),
            Diagnostic::invalid_assignment("REAL", "DINT", (457..472).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (508..512).into()),
            Diagnostic::invalid_assignment("__main_var3", "DINT", (514..518).into()),
            Diagnostic::invalid_assignment("REAL", "DINT", (520..524).into()),
        ]
    );
}

#[test]
fn program_call_parameter_validation() {
    // GIVEN
    // WHEN
    let diagnostics = parse_and_validate(
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
			prog(var2, var3, var4); // invalid types assigned
		END_PROGRAM
        "#,
    );

    // THEN
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_parameter_type((354..358).into()),
            Diagnostic::invalid_parameter_type((360..364).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (420..434).into()),
            Diagnostic::invalid_assignment("__main_var3", "DINT", (436..450).into()),
            Diagnostic::incompatible_type_size("DINT", 32, "hold a", (436..450).into()),
            Diagnostic::invalid_assignment("REAL", "DINT", (452..467).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (504..508).into()),
            Diagnostic::invalid_assignment("__main_var3", "DINT", (510..514).into()),
            Diagnostic::invalid_assignment("REAL", "DINT", (516..520).into()),
        ]
    );
}

#[test]
fn reference_to_reference_assignments_in_function_arguments() {
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
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

    let types_and_ranges = vec![
        // REF(...)
        ("__POINTER_TO_INT", "__prog_input1", (1286..1308)),
        ("__POINTER_TO_REAL", "__prog_input2", (1322..1344)),
        ("__POINTER_TO_STRING", "__prog_input3", (1358..1380)),
        // &(...)
        ("__POINTER_TO_INT", "__prog_input1", (1596..1615)),
        ("__POINTER_TO_REAL", "__prog_input2", (1630..1649)),
        ("__POINTER_TO_STRING", "__prog_input3", (1664..1683)),
    ];

    assert_eq!(diagnostics.len(), 6);
    assert_eq!(diagnostics.len(), types_and_ranges.len());

    for (idx, diagnostic) in diagnostics.iter().enumerate() {
        assert_eq!(
            diagnostic,
            &Diagnostic::invalid_assignment(
                types_and_ranges[idx].0,
                types_and_ranges[idx].1,
                types_and_ranges[idx].2.to_owned().into()
            )
        );
    }
}

#[test]
fn address_of_operations() {
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
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

    assert_eq!(diagnostics.len(), 3);

    let ranges = vec![(462..465), (479..483), (497..502)];
    for (idx, diagnostic) in diagnostics.iter().enumerate() {
        assert_eq!(
            diagnostic,
            &Diagnostic::invalid_operation("Invalid address-of operation", ranges[idx].to_owned().into())
        );
    }
}

#[test]
fn validate_call_by_ref() {
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
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
            
            // Explicit argument assignments are also valid, IF their right side is a LValue
            func(byValInput := 1, byRefInOut := 2, byRefOutput =>  );
            func(byValInput := 1, byRefInOut := x, byRefOutput =>  ); // Valid (Output assignments are optional)
            func(byValInput := 1, byRefInOut := 2, byRefOutput => 3); 
            func(byValInput := 1, byRefInOut := 2, byRefOutput => x);
            func(byValInput := 1, byRefInOut := x, byRefOutput => 3);
            func(byValInput := 1, byRefInOut := x, byRefOutput => x); // Valid

        END_PROGRAM
        ",
    );

    let ranges = vec![
        ("byRefInOut", VariableType::InOut, (589..590)),
        ("byRefOutput", VariableType::Output, (592..593)),
        ("byRefInOut", VariableType::InOut, (616..617)),
        ("byRefOutput", VariableType::Output, (646..647)),
        ("byRefInOut", VariableType::InOut, (706..707)),
        ("byRefOutput", VariableType::Output, (709..710)),
        ("byRefInOut", VariableType::InOut, (733..734)),
        ("byRefOutput", VariableType::Output, (763..764)),
        ("byRefInOut", VariableType::InOut, (957..958)),
        ("byRefInOut", VariableType::InOut, (1140..1141)),
        ("byRefOutput", VariableType::Output, (1158..1159)),
        ("byRefInOut", VariableType::InOut, (1211..1212)),
        ("byRefOutput", VariableType::Output, (1299..1300)),
    ];

    assert_eq!(diagnostics.len(), 13);
    for (idx, diagnostic) in diagnostics.iter().enumerate() {
        assert_eq!(
            diagnostic,
            &Diagnostic::invalid_argument_type(ranges[idx].0, ranges[idx].1, ranges[idx].2.to_owned().into()),
        );
    }
}

#[test]
fn implicit_param_downcast_in_function_call() {
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
        "
        PROGRAM main
        VAR
            var1_lint, var2_lint : LINT;
            var_lreal            : LREAL;
            var_lword            : LWORD;
            // trying to implicitly cast arrays gives an invalid assignment error, it shouldn't also give a downcast warning
            var_ref_arr          : ARRAY[1..3] OF LTIME;
            var_in_out_arr       : WSTRING;
            var_arr              : ARRAY[1..3] OF LINT;
        END_VAR
            foo(
                var1_lint, 
                var_lword, 
                var_ref_arr,
                var_lreal, 
                INT#var1_lint, 
                var_arr,
                var2_lint, 
                var_in_out_arr,
                var1_lint
            );
        END_PROGRAM

        FUNCTION foo : DINT
        VAR_INPUT {ref}
            in_ref_int      : INT;
            in_ref_dword    : DWORD;
            in_ref_arr      : ARRAY[1..3] OF TIME;
        END_VAR
        VAR_INPUT
            in_real         : REAL;
            in_sint         : SINT;
            in_arr          : ARRAY[1..3] OF TIME;
        END_VAR
        VAR_IN_OUT
            in_out          : INT;
            in_out_arr      : STRING;
        END_VAR
        VAR_OUTPUT
            out_var         : DINT;
        END_VAR
        END_FUNCTION
        ",
    );
    // we are expecting 6 implicit downcast warnings and 3 invalid assignment
    assert_eq!(diagnostics.len(), 9);
    let ranges = &[(490..499), (518..527), (575..584), (603..616), (660..669), (720..729)];
    let passed_types = &["LINT", "LWORD", "LREAL", "INT", "LINT", "LINT"];
    let expected_types = &["INT", "DWORD", "REAL", "SINT", "INT", "DINT"];
    for (idx, diagnostic) in
        diagnostics.iter().filter(|it| matches!(it, Diagnostic::ImprovementSuggestion { .. })).enumerate()
    {
        assert_eq!(
            diagnostic,
            &Diagnostic::implicit_downcast(
                expected_types[idx],
                passed_types[idx],
                ranges[idx].to_owned().into()
            )
        );
    }
    assert_eq!(diagnostics.iter().filter(|it| matches!(it, Diagnostic::SyntaxError { .. })).count(), 3);
}

#[test]
fn function_block_implicit_downcast() {
    let diagnostics = parse_and_validate(
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
                var1_lint, 
                var_lword, 
                var_real, 
                INT#var1_lint, 
                var2_lint, 
                var_wstr,
                var1_lint
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
    // we are expecting 5 implicit downcast warnings and 1 invalid assignment
    assert_eq!(diagnostics.len(), 6);
    let ranges = &[(272..281), (300..309), (355..368), (387..396), (441..450)];
    let passed_types = &["LINT", "LWORD", "INT", "LINT", "LINT"];
    let expected_types = &["INT", "DWORD", "SINT", "INT", "DINT"];
    for (idx, diagnostic) in
        diagnostics.iter().filter(|it| matches!(it, Diagnostic::ImprovementSuggestion { .. })).enumerate()
    {
        assert_eq!(
            diagnostic,
            &Diagnostic::implicit_downcast(
                expected_types[idx],
                passed_types[idx],
                ranges[idx].to_owned().into()
            )
        );
    }
    assert_eq!(diagnostics.iter().filter(|it| matches!(it, Diagnostic::SyntaxError { .. })).count(), 1);
}

#[test]
fn program_implicit_downcast() {
    let diagnostics = parse_and_validate(
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
                var1_lint, 
                var_lword, 
                var_real, 
                INT#var1_lint, 
                var2_lint, 
                var_wstr,
                var1_lint
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
    // we are expecting 5 implicit downcast warnings and 1 invalid assignment
    assert_eq!(diagnostics.len(), 6);
    let ranges = &[(274..283), (302..311), (357..370), (389..398), (443..452)];
    let passed_types = &["LINT", "LWORD", "INT", "LINT", "LINT"];
    let expected_types = &["INT", "DWORD", "SINT", "INT", "DINT"];
    for (idx, diagnostic) in
        diagnostics.iter().filter(|it| matches!(it, Diagnostic::ImprovementSuggestion { .. })).enumerate()
    {
        assert_eq!(
            diagnostic,
            &Diagnostic::implicit_downcast(
                expected_types[idx],
                passed_types[idx],
                ranges[idx].to_owned().into()
            )
        );
    }
    assert_eq!(diagnostics.iter().filter(|it| matches!(it, Diagnostic::SyntaxError { .. })).count(), 1);
}

#[test]
fn action_implicit_downcast() {
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM main
        VAR
            var_lint : LINT;
            var_wstr : WSTRING;
            var_arr  : ARRAY[1..3] OF LINT;
            fb       : fb_t;
        END_VAR
            fb.foo(var_lint, var_wstr);
            prog.bar(var_lint, var_arr);
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

    // we are expecting 2 implicit downcast warnings and 2 invalid assignment errors
    assert_eq!(diagnostics.len(), 4);
    let ranges = &[(203..211), (245..253)];
    let passed_types = &["LINT", "LINT"];
    let expected_types = &["DINT", "INT"];
    for (idx, diagnostic) in
        diagnostics.iter().filter(|it| matches!(it, Diagnostic::ImprovementSuggestion { .. })).enumerate()
    {
        assert_eq!(
            diagnostic,
            &Diagnostic::implicit_downcast(
                expected_types[idx],
                passed_types[idx],
                ranges[idx].to_owned().into()
            )
        );
    }
    assert_eq!(diagnostics.iter().filter(|it| matches!(it, Diagnostic::SyntaxError { .. })).count(), 2);
}

#[test]
fn method_implicit_downcast() {
    let diagnostics = parse_and_validate(
        r#"
    PROGRAM main
    VAR
        cl : MyClass;
        var_lint : LINT;
        var_arr : ARRAY[1..3] OF DINT;
    END_VAR
        cl.testMethod(var_lint, var_arr, ADR(var_arr));
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

    assert_eq!(diagnostics.len(), 2);
    let ranges = &[(146..154)];
    let passed_types = &["LINT"];
    let expected_types = &["INT"];
    for (idx, diagnostic) in
        diagnostics.iter().filter(|it| matches!(it, Diagnostic::ImprovementSuggestion { .. })).enumerate()
    {
        assert_eq!(
            diagnostic,
            &Diagnostic::implicit_downcast(
                expected_types[idx],
                passed_types[idx],
                ranges[idx].to_owned().into()
            )
        );
    }
    assert_eq!(diagnostics.iter().filter(|it| matches!(it, Diagnostic::SyntaxError { .. })).count(), 1);
}

#[test]
fn validate_call_by_ref_arrays() {
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
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

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].get_message(), "Invalid assignment: cannot assign '__main_x' to 'INT'");
    assert_eq!(diagnostics[0].get_affected_ranges(), &[(323..324).into()]);

    assert_eq!(diagnostics[1].get_message(), "Invalid assignment: cannot assign '__main_x' to 'INT'");
    assert_eq!(diagnostics[1].get_affected_ranges(), &[(326..327).into()]);
}
