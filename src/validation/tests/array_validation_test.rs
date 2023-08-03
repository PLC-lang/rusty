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

#[test]
fn array_access_dimension_mismatch() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION fn : DINT
			VAR_INPUT {ref}
				arr : ARRAY[0..5] OF DINT;
				vla : ARRAY[*] OF DINT;
			END_VAR

			// Valid
			arr[0] := 1;
			vla[0] := 1;

			// Invalid
			arr[0, 1] := 1;
			vla[0, 1] := 1;
			arr[0, 1, 2] := 1;
			vla[0, 1, 2] := 1;
		END_FUNCTION
		",
    );

    assert_eq!(diagnostics.len(), 4);
    assert_validation_snapshot!(diagnostics);
}

#[test]
fn exceeding_size_1d() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION main : DINT
			VAR
				arr 		: ARRAY[1..5] OF DINT;
				arr_init	: ARRAY[1..5] OF DINT := [1, 2, 3, 4, 5, 6];
			END_VAR

			arr := [1, 2, 3, 4, 5, 6];
			arr := (1, 2, 3, 4, 5, 6);
		END_FUNCTION
		",
    );

    assert_validation_snapshot!(diagnostics);
}

#[test]
fn exceeding_size_2d() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION main : DINT
			VAR
				arr 		: ARRAY[1..2, 1..5] OF DINT;
				arr_nested 	: ARRAY[1..2] OF ARRAY[1..5] OF DINT;

				arr_init 		: ARRAY[1..2, 1..5] OF DINT := [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
				arr_nested_init : ARRAY[1..2] OF ARRAY[1..5] OF DINT := [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10, 11]];
			END_VAR

			arr := [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
			arr := (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
			
			arr_nested := [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10, 11]];
			// arr_nested := ((1, 2, 3, 4, 5), (6, 7, 8, 9, 10, 11)); // TODO: Nested Arrays can only be initialized with a bracket symbol?
		END_FUNCTION
		",
    );

    assert_validation_snapshot!(diagnostics);
}

#[test]
fn exceeding_size_3d() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION main : DINT
			VAR
				arr 		: ARRAY[1..2, 1..2, 1..2] OF DINT;
				arr_nested 	: ARRAY[1..2] OF ARRAY[1..2] OF ARRAY[1..2] OF DINT;

				arr_init 		: ARRAY[1..2, 1..2, 1..2] OF DINT := [1, 2, 3, 4, 5, 6, 7, 8, 9];
				arr_nested_init	: ARRAY[1..2] OF ARRAY[1..2] OF ARRAY[1..2] OF DINT := [[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]]];
			END_VAR

			arr := [1, 2, 3, 4, 5, 6, 7, 8, 9];
			arr := (1, 2, 3, 4, 5, 6, 7, 8, 9);
			
			arr_nested := [
				[[1, 2], [3, 4]],
				[[5, 6], [7, 8]],
				[[9, 10], [11, 12]],
			];
		END_FUNCTION
		",
    );

    assert_validation_snapshot!(diagnostics);
}

#[test]
fn exceeding_size_structs() {
    let diagnostics = parse_and_validate(
        "
		TYPE Foo : STRUCT
			idx: DINT;
			arr : ARRAY[1..2] OF Bar;
		END_STRUCT END_TYPE

		TYPE Bar : STRUCT
			arr : ARRAY[1..2] OF DINT;
		END_STRUCT END_TYPE

		FUNCTION main : DINT
			VAR
				arr_a : Foo := (
					idx := 1, 
					arr := ((arr := [1, 2]), (arr := [3, 4]), (arr := [5, 6])) 			// Invalid, because the outter `arr` can only store 2 elements
				);

				arr_b : ARRAY[1..2] OF Foo := (
					(idx := 2, arr := ((arr := [1, 2, 3]),  (arr := [4, 5]))),			// Invalid because of the first inner `arr`
					(idx := 3, arr := ((arr := [1, 2]),     (arr := [3, 4, 5]))),		// Invalid because of the second inner `arr`
				);

				arr_c : ARRAY[1..2] OF Foo := (											// Invalid because only 2 elements can be stored, but 3 are provided
					(idx := 4, arr := ((arr := [1, 2, 3]),  (arr := [4, 5]))),			// Invalid because of the first inner `arr`
					(idx := 5, arr := ((arr := [1, 2]),     (arr := [3, 4, 5]))),		// Invalid because of the second inner `arr`
					(idx := 6, arr := ((arr := 2(0)),  		(arr := 4(0)))),			// Invalid ebcause of the second inner `arr`
				);
			END_VAR
		END_FUNCTION
		",
    );

    assert_validation_snapshot!(diagnostics)
}

#[test]
fn struct_initialization_with_array_initializer_using_multiplied_statement() {
    let diagnostics = parse_and_validate(
        "
		TYPE myStruct : STRUCT
			arr : ARRAY[0..63] OF BYTE;
			idx : DINT;
		END_STRUCT END_TYPE
		PROGRAM mainProg
			VAR
				val : myStruct := (arr := 64(0), idx := 0);
			END_VAR
		END_PROGRAM
		",
    );

    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn exceeding_size_multiplied_statement() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION main : DINT
			VAR
				arr 		: ARRAY[1..5] OF DINT;
				arr_init	: ARRAY[1..5] OF DINT := 6(0);
			END_VAR

			arr := 6(0);
		END_FUNCTION
		",
    );

    assert_validation_snapshot!(diagnostics);
}
