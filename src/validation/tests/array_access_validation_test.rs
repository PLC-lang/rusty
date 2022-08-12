use crate::ast::SourceRange;
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
            Diagnostic::incompatible_array_access_range(2..3, SourceRange::new(557..558,Some(28),Some(21),Some(28),Some(24))),
            Diagnostic::incompatible_array_access_range(2..3, SourceRange::new(590..591,Some(29),Some(23),Some(29),Some(24))),
            Diagnostic::incompatible_array_access_range(0..1, SourceRange::new(617..618,Some(30),Some(17),Some(30),Some(18))),
            Diagnostic::incompatible_array_access_range(-2..2, SourceRange::new(655..657,Some(31),Some(28),Some(31),Some(30))),
            Diagnostic::incompatible_array_access_range(-3..-1, SourceRange::new(688..690,Some(32),Some(22),Some(32),Some(24))),
            Diagnostic::incompatible_array_access_range(1..2, SourceRange::new(718..719,Some(33),Some(19),Some(33),Some(20))),
            Diagnostic::incompatible_array_access_type("STRING", SourceRange::new(745..755,Some(34),Some(17),Some(34),Some(27))),
            Diagnostic::incompatible_array_access_variable("INT", SourceRange::new(802..803,Some(35),Some(21),Some(35),Some(22))),
        ]
    );
}
