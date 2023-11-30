// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::compile_and_run;

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug)]
struct MainType {
    x: i16,
    y: i16,
    z: i16,
    int_array: [i16; 5],
    matrix: [[i16; 5]; 5],    //5x5 array
    cube: [[[i32; 5]; 5]; 5], //5x5x5 array
}

fn new() -> MainType {
    MainType {
        x: 0,
        y: 0,
        z: 0,
        int_array: [1, 2, 3, 4, 5],
        matrix: [[0; 5]; 5],    //initialize with 0
        cube: [[[0; 5]; 5]; 5], //initialize with 0
    }
}
#[test]
fn array_assignments() {
    let function = r"
        PROGRAM main
        VAR
            x : INT;
            y : INT;
            z : INT;
            int_array : ARRAY[0..4] OF INT;
        END_VAR
            int_array[0] := 0 + 10;
            int_array[1] := 1 + 10;
            int_array[2] := 2 + 10;
            int_array[3] := 3 + 10;
            int_array[4] := 4 + 10;
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    for index in 0..5 {
        assert_eq!((index + 10) as i16, maintype.int_array[index]);
    }
}

#[test]
fn array_declaration_using_constants() {
    let function = r"
        VAR_GLOBAL CONSTANT
            ONE : INT := 1;
            LEN : INT := 2 * ONE + ONE;
            ARRAY_LEN : INT := LEN + 1;
        END_VAR

        PROGRAM main
        VAR
            x : INT;
            y : INT;
            z : INT;
            int_array : ARRAY[ ONE-1 .. ARRAY_LEN] OF INT;
        END_VAR
            int_array[0] := 0 + 10;
            int_array[1] := 1 + 10;
            int_array[2] := 2 + 10;
            int_array[3] := 3 + 10;
            int_array[4] := 4 + 10;
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    for index in 0..5 {
        assert_eq!((index + 10) as i16, maintype.int_array[index]);
    }
}

#[test]
fn matrix_array_assignments() {
    let function = r"
        PROGRAM main
        VAR
            x: INT;
            y: INT;
            z: INT;
            int_array   : ARRAY[0..4] OF INT;
            matrix      : ARRAY[0..4, 0..4] OF INT;
        END_VAR

            FOR x := 0 TO 4 DO
                FOR y := 0 TO 4 DO
                    matrix[x, y] := x*y;
                END_FOR
            END_FOR
        END_PROGRAM
        ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    for x in 0..5 {
        for y in 0..5 {
            assert_eq!((x * y) as i16, maintype.matrix[x][y]);
        }
    }
}

#[test]
fn two_dim_array_math() {
    let function = "
        FUNCTION main : INT
        VAR
            x,y,z : INT;
            int_array : ARRAY[0..4, 0..4] OF INT;
        END_VAR
        x := int_array[0,1];
        y := int_array[0,2];
        z := int_array[4,4];
        x := 10;
        y := 20;
        z := 5;
            main := x+y-z*z/z;
        END_FUNCTION
        ";

    let mut maintype = new();
    let res: i16 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(res, 25);
}

#[test]
fn three_dim_array_math() {
    let function = "
        FUNCTION main : INT
        VAR
            x,y,z : INT;
            int_array : ARRAY[0..4, 0..4, 0..4] OF INT;
        END_VAR
        x := int_array[0,1,0];
        y := int_array[0,2,3];
        z := int_array[4,4,0];
        x := 10;
        y := 20;
        z := 5;
            main := x+y-z*z/z+x-y;
        END_FUNCTION
        ";

    let mut maintype = crate::MainType::default();
    let res: i16 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(res, 15);
}

#[test]
fn matrix_array_assignments2() {
    let function = r"
    PROGRAM main
    VAR
    x: INT;
    y: INT;
    z: INT;
    int_array   : ARRAY[0..4] OF INT;
    matrix      : ARRAY[0..4] OF ARRAY[0..4] OF INT;
    END_VAR

            FOR x := 0 TO 4 DO
                FOR y := 0 TO 4 DO
                    matrix[x][y] := x*y;
                END_FOR
            END_FOR
            END_PROGRAM
            ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    for x in 0..5 {
        for y in 0..5 {
            assert_eq!((x * y) as i16, maintype.matrix[x][y]);
        }
    }
}

#[test]
fn cube_array_assignments_array_of_array_of_array() {
    let function = r"
            PROGRAM main
            VAR
            x: INT;
            y: INT;
            z: INT;
            int_array   : ARRAY[0..4] OF INT;
            matrix      : ARRAY[0..4] OF ARRAY[0..4] OF INT;
            cube        : ARRAY[0..4] OF ARRAY[0..4] OF ARRAY[0..4] OF DINT;
            END_VAR

            FOR x := 0 TO 4 DO
               FOR y := 0 TO 4 DO
                   FOR z := 0 TO 4 DO
                       cube[x][y][z] := x*y*z;
                   END_FOR
               END_FOR
            END_FOR
            END_PROGRAM
            ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                assert_eq!((x * y * z) as i32, maintype.cube[x][y][z]);
            }
        }
    }
}

#[test]
fn simple_cube_array_assignments() {
    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Debug, Default)]
    struct MainType {
        x: i32,
        y: i32,
        z: i32,
        cube: [[[i32; 5]; 5]; 5], //5x5x5 array
    }
    let function = r"
            PROGRAM main
            VAR
            x: DINT;
            y: DINT;
            z: DINT;
            cube        : ARRAY[0..4, 0..4, 0..4] OF DINT;
            END_VAR

            x := 0; y := 0; z:= 0;
            cube[x, y, z] := 1;

            x := 4; y := 4; z:= 4;
            cube[x, y, z] := 77;

           END_PROGRAM
            ";

    let mut maintype = MainType::default();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(1, maintype.cube[0][0][0]);
    assert_eq!(77, maintype.cube[4][4][4]);
}

#[test]
fn cube_array_assignments2() {
    let function = r"
            PROGRAM main
            VAR
            x: INT;
            y: INT;
            z: INT;
            int_array   : ARRAY[0..4] OF INT;
            matrix      : ARRAY[0..4] OF ARRAY[0..4] OF INT;
            cube        : ARRAY[0..4, 0..4, 0..4] OF DINT;
            END_VAR

            FOR x := 0 TO 4 DO
               FOR y := 0 TO 4 DO
                  FOR z := 0 TO 4 DO
                    cube[x, y, z] := (x+1)*(y+1)*(z+1);
                   END_FOR
               END_FOR
            END_FOR
            END_PROGRAM
            ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                assert_eq!(((x + 1) * (y + 1) * (z + 1)) as i32, maintype.cube[x][y][z]);
            }
        }
    }
}

#[test]
fn two_dim_array_if() {
    let function = "
        FUNCTION main : INT
        VAR
            x,y,z : INT;
            int_array : ARRAY[0..4, 0..4] OF INT;
        END_VAR
            int_array[0,1] := 10;
            y := 20;

            IF y > 21 THEN
                int_array[4,4] := 40;
            ELSIF y < 21 THEN
                int_array[4,4] := 20;
            END_IF;
            main := int_array[4,4];
        END_FUNCTION
        ";

    let mut maintype = new();
    let res: i16 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(res, 20);
}

#[test]
fn two_dim_array_while() {
    let function = "
        FUNCTION main : INT
        VAR
            x,y,z,counter : INT;
            int_array : ARRAY[0..4, 0..4] OF INT;
        END_VAR
            int_array[0,1] := 10;
            y := 20;

            WHILE counter = 0 DO
                int_array[4,4] := 1;
                counter := counter +1;
            END_WHILE
            main := counter;
        END_FUNCTION
        ";

    let mut maintype = new();
    let res: i16 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(res, 1);
}

#[test]
fn initialize_multi_dim_array() {
    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Debug, Default)]
    struct MainType {
        arr: [i16; 27], //3x3x3 array
    }
    let function = "
        PROGRAM target
        VAR
            int_array : ARRAY[0..2, 0..2, 0..2] OF INT := [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26];
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            int_array : ARRAY[0..2, 0..2, 0..2] OF INT;
        END_VAR
        //lets see if target.int_array got initialized properly
        int_array := target.int_array;
        END_PROGRAM

        ";

    let mut maintype = MainType::default();
    let _: i16 = compile_and_run(function.to_string(), &mut maintype);
    (0..27i16).for_each(|i| {
        assert_eq!(i, maintype.arr[i as usize]);
    })
}

#[test]
fn bool_array_assignments() {
    #[repr(C)]
    struct MainType {
        x: i16,
        b_array1: [u8; 8], // i reserve 8 bytes here! BOOL is stored as i8
        y: i16,
        b_array2: [u8; 8], // i reserve 8 bytes here! BOOL is stored as i8
        z: i16,
    }

    // GIVEN some boolean arrays
    // WHEN I write the array-elements

    let function = r"
        PROGRAM main
        VAR
            x : INT;
            bArray : ARRAY[0..7] OF BOOL := [8(FALSE)];
            y : INT;
            bArray2 : ARRAY[0..7] OF BOOL := [8(FALSE)];
            z : INT;
        END_VAR
            x := 111;
            y := 222;
            z := 333;
            //write forwards
            bArray[0] := TRUE;
            bArray[1] := FALSE;
            bArray[2] := TRUE;
            bArray[3] := FALSE;
            bArray[4] := TRUE;
            bArray[5] := FALSE;
            bArray[6] := TRUE;
            bArray[7] := FALSE;

            //write backwards
            bArray2[7] := TRUE;
            bArray2[6] := FALSE;
            bArray2[5] := TRUE;
            bArray2[4] := FALSE;
            bArray2[3] := TRUE;
            bArray2[2] := FALSE;
            bArray2[1] := TRUE;
            bArray2[0] := FALSE;
        END_PROGRAM
        ";

    let mut maintype = MainType { x: 0, b_array1: [0; 8], y: 0, b_array2: [0; 8], z: 0 };
    //Then i expect the correct array-values without leaking into neighbour segments
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(maintype.b_array1, [1, 0, 1, 0, 1, 0, 1, 0]);
    assert_eq!(maintype.b_array2, [0, 1, 0, 1, 0, 1, 0, 1]);

    //check the magic numbers to spot some alignment issues
    assert_eq!(maintype.x, 111);
    assert_eq!(maintype.y, 222);
    assert_eq!(maintype.z, 333);
}

#[test]
fn assigning_global_arrays_in_function_by_passing_references() {
    #[repr(C)]
    struct MainType {
        arr_in: [u16; 4],
        arr_in_ref: [u16; 4],
        arr_inout: [u16; 4],
    }
    let function = r#"
        FUNCTION foo : DINT
        VAR_INPUT
            in : ARRAY[0..3] OF INT;
        END_VAR
        VAR_INPUT {ref}
            in_ref : ARRAY[0..3] OF INT;
        END_VAR
        VAR_IN_OUT
            inout : ARRAY[0..3] OF INT;
        END_VAR
            glob_arr_in     := in;
            glob_arr_in_ref := in_ref;
            glob_arr_inout  := inout;
        END_FUNCTION

        PROGRAM main
        VAR
            arr_in      : ARRAY[0..3] OF INT;
            arr_in_ref  : ARRAY[0..3] OF INT;
            arr_inout   : ARRAY[0..3] OF INT;
        END_VAR
        VAR_TEMP
            a : ARRAY[0..3] OF INT := (1, 2, 3, 4);
            b : ARRAY[0..3] OF INT := (5, 6, 7, 8);
            c : ARRAY[0..3] OF INT := (9, 10, 11, 12);
        END_VAR
            foo(a, b, c);

            arr_in      := glob_arr_in;
            arr_in_ref  := glob_arr_in_ref;
            arr_inout   := glob_arr_inout;
        END_PROGRAM

        VAR_GLOBAL
            glob_arr_in     : ARRAY[0..3] OF INT;
            glob_arr_in_ref : ARRAY[0..3] OF INT;
            glob_arr_inout  : ARRAY[0..3] OF INT;
        END_VAR
    "#;

    let mut maintype = MainType { arr_in: [0; 4], arr_in_ref: [0; 4], arr_inout: [0; 4] };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(maintype.arr_in, [1, 2, 3, 4]);
    assert_eq!(maintype.arr_in_ref, [5, 6, 7, 8]);
    assert_eq!(maintype.arr_inout, [9, 10, 11, 12]);
}

#[test]
fn assigning_global_arrays_in_program_by_passing_references() {
    #[repr(C)]
    struct MainType {
        arr_in: [u16; 4],
        arr_in_ref: [u16; 4],
        arr_inout: [u16; 4],
    }
    let function = r#"
        PROGRAM prog
        VAR_INPUT
            in : ARRAY[0..3] OF INT;
        END_VAR
        VAR_INPUT {ref}
            in_ref : ARRAY[0..3] OF INT;
        END_VAR
        VAR_IN_OUT
            inout : ARRAY[0..3] OF INT;
        END_VAR
            glob_arr_in     := in;
            glob_arr_in_ref := in_ref;
            glob_arr_inout  := inout;
        END_PROGRAM

        PROGRAM main
        VAR
            arr_in      : ARRAY[0..3] OF INT;
            arr_in_ref  : ARRAY[0..3] OF INT;
            arr_inout   : ARRAY[0..3] OF INT;
        END_VAR
        VAR_TEMP
            a : ARRAY[0..3] OF INT := (1, 2, 3, 4);
            b : ARRAY[0..3] OF INT := (5, 6, 7, 8);
            c : ARRAY[0..3] OF INT := (9, 10, 11, 12);
        END_VAR
            prog(a, b, c);

            arr_in      := glob_arr_in;
            arr_in_ref  := glob_arr_in_ref;
            arr_inout   := glob_arr_inout;
        END_PROGRAM

        VAR_GLOBAL
            glob_arr_in     : ARRAY[0..3] OF INT;
            glob_arr_in_ref : ARRAY[0..3] OF INT;
            glob_arr_inout  : ARRAY[0..3] OF INT;
        END_VAR
    "#;

    let mut maintype = MainType { arr_in: [0; 4], arr_in_ref: [0; 4], arr_inout: [0; 4] };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(maintype.arr_in, [1, 2, 3, 4]);
    assert_eq!(maintype.arr_in_ref, [5, 6, 7, 8]);
    assert_eq!(maintype.arr_inout, [9, 10, 11, 12]);
}

#[test]
fn access_arrays_by_ref() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        a0: u16,
        a1: u16,
        b0: u16,
        b1: u16,
    }
    let function = r#"
        FUNCTION foo : DINT
        VAR_INPUT {ref}
            in_ref : ARRAY[0..1] OF INT;
        END_VAR
        VAR_IN_OUT
            inout : ARRAY[0..1] OF INT;
        END_VAR
			glob_a0 := in_ref[0];
			glob_a1 := in_ref[1];
			glob_b0 := inout[0];
			glob_b1 := inout[1];
        END_FUNCTION

        PROGRAM main
        VAR
			a0, a1, b0, b1 : INT;
        END_VAR
        VAR_TEMP
            a : ARRAY[0..1] OF INT := (1, 2);
            b : ARRAY[0..1] OF INT := (5, 6);
        END_VAR
            foo(a, b);

			a0 := glob_a0;
			a1 := glob_a1;
			b0 := glob_b0;
			b1 := glob_b1;
        END_PROGRAM

        VAR_GLOBAL
			glob_a0, glob_a1, glob_b0, glob_b1 : INT;
        END_VAR
    "#;

    let mut maintype = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(maintype.a0, 1);
    assert_eq!(maintype.a1, 2);
    assert_eq!(maintype.b0, 5);
    assert_eq!(maintype.b1, 6);
}

#[test]
fn initialize_array_with_struct_elements() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        a0: u16,
        b0: u16,
        c0: u16,
        d0: u16,
    }

    let function = r#"
        TYPE myStruct : STRUCT
            x : INT;
            y : INT;
        END_STRUCT END_TYPE

        PROGRAM target
            VAR
                bracket : ARRAY[0..1] OF myStruct := [(x := 2, y := 4), (x := 6, y := 8)];
            END_VAR
        END_PROGRAM

        PROGRAM main
            VAR
                a0, b0, c0, d0 : INT;
            END_VAR

            a0 := target.bracket[0].x;
            b0 := target.bracket[0].y;
            c0 := target.bracket[1].x;
            d0 := target.bracket[1].y;
        END_PROGRAM
    "#;

    let mut maintype = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert_eq!(maintype.a0, 2);
    assert_eq!(maintype.b0, 4);
    assert_eq!(maintype.c0, 6);
    assert_eq!(maintype.d0, 8);
}

#[test]
fn struct_initialization_with_array_initializer_using_multiplied_statement() {
    #[repr(C)]
    struct MainType {
        arr: [u16; 64],
        idx: u16,
    }

    let source = "
		TYPE myStruct : STRUCT
			arr : ARRAY[0..63] OF INT;
			idx : INT;
		END_STRUCT END_TYPE
        PROGRAM target
			VAR
				val : myStruct := (arr := [64(111)], idx := 222);
            END_VAR
        END_PROGRAM
		PROGRAM main
            VAR
                arr : ARRAY[0..63] OF INT;
                idx : INT := 0;
			END_VAR
            arr := target.val.arr;
            idx := target.val.idx;
		END_PROGRAM
		"
    .to_string();

    let mut maintype = MainType { arr: [0; 64], idx: 0 };
    let _: i32 = compile_and_run(source, &mut maintype);
    assert_eq!(maintype.arr, [111; 64]);
    assert_eq!(maintype.idx, 222);
}
