/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

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

    compile_and_run(function.to_string(), &mut maintype);

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

    compile_and_run(function.to_string(), &mut maintype);
    for x in 0..5 {
        for y in 0..5 {
            assert_eq!((x * y) as i16, maintype.matrix[x][y]);
        }
    }
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

    compile_and_run(function.to_string(), &mut maintype);
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

    compile_and_run(function.to_string(), &mut maintype);
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

    compile_and_run(function.to_string(), &mut maintype);
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                assert_eq!(((x+1) * (y+1) * (z+1)) as i32, maintype.cube[x][y][z]);
            }
        }
    }
}
