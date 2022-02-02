use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

#[test]
fn array_access_validation() {
    let diagnostics = parse_and_validate(
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_array_access_range(2..3, (557..558).into()),
            Diagnostic::incompatible_array_access_range(2..3, (590..591).into()),
            Diagnostic::incompatible_array_access_range(0..1, (617..618).into()),
            Diagnostic::incompatible_array_access_range(-2..2, (655..657).into()),
            Diagnostic::incompatible_array_access_range(-3..-1, (688..690).into()),
            Diagnostic::incompatible_array_access_range(1..2, (718..719).into()),
            Diagnostic::incompatible_array_access_type("STRING", (745..755).into()),
            Diagnostic::incompatible_array_access_variable("INT", (802..803).into()),
        ]
    );
}
