use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

static SOURCE: &str = "
    <POU_TYPE> fn : DINT
        VAR_<VAR_TYPE>
            arr : ARRAY[*] OF DINT;
        END_VAR
    END_<POU_TYPE>

    FUNCTION main : DINT
        VAR
            local : ARRAY[-5..5] OF DINT;
        END_VAR

        fn(local);
    END_FUNCTION
";

#[test]
fn variable_length_array_defined_as_a_global_variable() {
    let src = "
        VAR_GLOBAL
            arr : ARRAY[*] OF DINT;
        END_VAR
    ";

    assert_validation_snapshot!(parse_and_validate(src));
}

mod functions {
    use crate::{
        assert_validation_snapshot, test_utils::tests::parse_and_validate,
        validation::tests::variable_length_array_test::SOURCE,
    };

    #[test]
    fn variable_length_array_function_with_input_output_and_inout() {
        let function = SOURCE.replace("<POU_TYPE>", "FUNCTION");
        assert!(parse_and_validate(&function.replace("<VAR_TYPE>", "INPUT {ref}")).is_empty());
        assert!(parse_and_validate(&function.replace("<VAR_TYPE>", "OUTPUT")).is_empty());
        assert!(parse_and_validate(&function.replace("<VAR_TYPE>", "IN_OUT")).is_empty());
    }

    #[test]
    fn variable_length_array_function_input() {
        let function = SOURCE.replace("<POU_TYPE>", "FUNCTION");
        assert_validation_snapshot!(parse_and_validate(&function.replace("<VAR_TYPE>", "INPUT")));
    }
}

mod program {
    use crate::{
        assert_validation_snapshot, test_utils::tests::parse_and_validate,
        validation::tests::variable_length_array_test::SOURCE,
    };

    #[test]
    fn variable_length_array_program_input() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_input = parse_and_validate(&program.replace("<VAR_TYPE>", "INPUT"));
        assert_validation_snapshot!(program_input);
    }

    #[test]
    fn variable_length_array_program_input_ref() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_input = parse_and_validate(&program.replace("<VAR_TYPE>", "INPUT {ref}"));
        assert_validation_snapshot!(program_input);
    }

    #[test]
    fn variable_length_array_program_output() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_output = parse_and_validate(&program.replace("<VAR_TYPE>", "OUTPUT"));
        assert_validation_snapshot!(program_output);
    }

    #[test]
    fn variable_length_array_program_inout() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_inout = parse_and_validate(&program.replace("<VAR_TYPE>", "IN_OUT"));
        assert_validation_snapshot!(program_inout);
    }
}

mod access {
    use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

    #[test]
    fn variable_length_array_access() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION fn : DINT
                VAR_INPUT {ref}
                    arr : ARRAY[*] OF DINT;
                END_VAR

                arr[0]      := 1;
                arr[0, 0]   := 1; // This should fail (arr is defined as a 1D array)
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    local_a : ARRAY[0..10] OF DINT;
                    local_b : ARRAY[0..5, 5..10] OF DINT;
                END_VAR

                fn(local_a);
                fn(local_b); // This call should fail, because we expect a 1D array
            END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }

    #[test]
    fn variable_length_array_incompatible_datatypes() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION fn : DINT
                VAR_INPUT {ref}
                    arr : ARRAY[*] OF DINT;
                END_VAR
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    local_int       : ARRAY[0..10] OF INT;
                    local_float     : ARRAY[0..10] OF REAL;
                    local_string    : ARRAY[0..10] OF STRING;
                END_VAR

                fn(local_int);
                fn(local_float);
                fn(local_string);
            END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }
}

mod assignment {
    use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

    #[test]
    fn function_calls() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION fn : DINT
                VAR_TEMP
                    a : ARRAY[0..10] OF DINT;
                END_VAR

                VAR_IN_OUT
                    vla : ARRAY[*] OF DINT;
                END_VAR

                // Invalid
                a   := vla;
                vla := a;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    arr : ARRAY[0..1] OF DINT;
                END_VAR

                // Valid (fn.vla <- main.arr assignment)
                fn(arr);
            END_FUNCTION
            ",
        );

        assert_validation_snapshot!(diagnostics);
    }
}

mod naming {
    use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};
    #[test]
    fn two_identical_vlas_in_same_pou_arent_duplicated_in_symbol_map() {
        let diag = parse_and_validate(
            r#"
        FUNCTION foo : INT
        VAR_INPUT{ref}
            vla1 : ARRAY[ * , * ] OF INT;
            vla2 : ARRAY[ * , * ] OF INT;
        END_VAR
        END_FUNCTION

        FUNCTION bar : DINT
        VAR_INPUT{ref}
            vla1 : ARRAY[ * , * ] OF LINT;
            vla2 : ARRAY[ * , * ] OF SINT;
            vla3 : ARRAY[ * , *, * ] OF SINT;
        END_VAR
        END_FUNCTION
    "#,
        );

        assert_eq!(diag.len(), 0);
    }

    #[test]
    fn global_vla_does_not_cause_name_conflict() {
        let diag = parse_and_validate(
            r#"
        VAR_GLOBAL
            vla : ARRAY[*, *] OF DINT;
        END_VAR

        FUNCTION foo : DINT
        VAR_IN_OUT
            arr : ARRAY[*, *] OF DINT;
        END_VAR
        END_FUNCTION
    "#,
        );
        assert_validation_snapshot!(diag);
    }
}

mod builtins {
    use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

    #[test]
    fn builtins_called_with_invalid_type() {
        let diagnostics = parse_and_validate(
            "
        FUNCTION main : DINT
        VAR CONSTANT
            MY_CONST : DINT := 10;
        END_VAR
        VAR
            arr : ARRAY[0..1] OF DINT;
            duration: TIME;
        END_VAR
            LOWER_BOUND(arr, MY_CONST + 1);
            LOWER_BOUND(duration, 1);
            LOWER_BOUND('i am a string', 1);

            UPPER_BOUND(arr, MY_CONST + 1);
            UPPER_BOUND(duration, 1);
            UPPER_BOUND('i am a string', 1);
        END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }

    #[test]
    fn builtins_called_with_invalid_index() {
        let diagnostics = parse_and_validate(
            "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..1] OF DINT;
        END_VAR
            foo(arr);
        END_FUNCTION

        TYPE MyType : INT; END_TYPE

        FUNCTION foo : DINT
        VAR_IN_OUT
            vla: ARRAY[*] OF DINT;
        END_VAR
        VAR
            x: MyType := 1;
        END_VAR
            LOWER_BOUND(vla, 3.1415); // invalid
            LOWER_BOUND(vla, TIME#3s); // invalid
            LOWER_BOUND(vla, 0); // index out of bounds

            UPPER_BOUND(vla, 3.1415); // invalid
            UPPER_BOUND(vla, TIME#3s); // invalid
            UPPER_BOUND(vla, 0); // index out of bounds

            UPPER_BOUND(vla, vla[LOWER_BOUND(vla, INT#1)]); // valid
        END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }

    #[test]
    fn builtins_called_with_aliased_type() {
        let diagnostics = parse_and_validate(
            "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..1] OF DINT;
        END_VAR
            foo(arr);
        END_FUNCTION

        TYPE MyType : INT; END_TYPE

        FUNCTION foo : DINT
        VAR_IN_OUT
            vla: ARRAY[*] OF DINT;
        END_VAR
        VAR
            x: MyType := 1;
        END_VAR

            LOWER_BOUND(vla, x); // valid
            UPPER_BOUND(vla, MyType#1); // valid
        END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }

    #[test]
    fn builtins_called_with_invalid_number_of_params() {
        let diagnostics = parse_and_validate(
            "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..1] OF DINT;
        END_VAR
            foo(arr);
        END_FUNCTION

        FUNCTION foo : DINT
        VAR_IN_OUT
            vla: ARRAY[*] OF DINT;
        END_VAR
            LOWER_BOUND();
            LOWER_BOUND(vla);
            LOWER_BOUND(1);
            LOWER_BOUND(vla, 1, 2, 3);

            UPPER_BOUND();
            UPPER_BOUND(vla);
            UPPER_BOUND(1);
            UPPER_BOUND(vla, 1, 2, 3);
        END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }
}
