// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;
#[allow(dead_code)]
#[repr(C)]
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

    let _ : i32 = compile_and_run(function.to_string(), &mut maintype);

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

    let _ : i32 = compile_and_run(function.to_string(), &mut maintype);

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

    let _ : i32 = compile_and_run(function.to_string(), &mut maintype);
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

    let mut maintype = new();
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

    let _ : i32 = compile_and_run(function.to_string(), &mut maintype);
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

    let _ : i32 = compile_and_run(function.to_string(), &mut maintype);
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                assert_eq!((x * y * z) as i32, maintype.cube[x][y][z]);
            }
        }
    }
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

    let _ : i32 = compile_and_run(function.to_string(), &mut maintype);
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
