use crate::assert_validation_snapshot;
use crate::test_utils::tests::parse_and_validate;

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

    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn array_initialization_validation() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION main : DINT
		VAR
			arr			: ARRAY[1..2] OF DINT;
			arr2		: ARRAY[1..2] OF DINT := 1, 2; // our parser can handle this, should we validate this ?
			arr3		: ARRAY[1..2] OF myStruct := ((var1 := 1), (var1 := 2, var2 := (1, 2))); // valid
			arr4		: ARRAY[1..2] OF myStruct := ((var1 := 1), (var1 := 2, var2 := 1, 2)); // var2 missing `(`
			arr_init	: ARRAY[1..2] OF DINT := (1, 2);
			x	 		: myStruct;
			y	 		: myStruct := (var1 := 1, var2 := 3, 4); // var2 missing `(`
		END_VAR
			arr	:= 1, 2; // missing `(`
			arr	:= (1, 2); // valid
			arr	:= (arr_init); // valid
			x	:= (var1 := 1, var2 := 3, 4); // var2 missing `(`
			x	:= (var1 := 1, var2 := arr_init); // valid
		END_FUNCTION
		
		TYPE myStruct : STRUCT
				var1 : DINT;
				var2 : ARRAY[1..2] OF DINT;
			END_STRUCT
		END_TYPE
       ",
    );

    assert_validation_snapshot!(&diagnostics);
}
