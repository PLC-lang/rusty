use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

#[test]
fn array_access_validation() {
    let diagnostics = parse_and_validate(
        "
        	PROGRAM prg
        	VAR
			multi : ARRAY[0..1,2..3] OF INT;
			arr : ARRAY[0..1] OF INT;
			int_ref : INT;
			string_ref : STRING;
        	END_VAR

			// valid
			multi[0,3];
			arr[1];
			arr[int_ref];

			// invalid
			multi[1,4]; // out of range
			arr[3]; // out of range
			arr[string_ref]; // invalid type for array access
			int_ref[1]; // not an array
        	END_PROGRAM
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_array_access_range(2..3, (241..242).into()),
            Diagnostic::incompatible_array_access_range(0..1, (268..269).into()),
            Diagnostic::incompatible_array_access_type("STRING", (295..305).into()),
            Diagnostic::incompatible_array_access_variable("INT", (352..353).into()),
        ]
    );
}
