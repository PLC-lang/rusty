// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests initialization of multi-dimensional arrays and arrays of arrays,
// including struct element types.

TYPE str1 : STRUCT
    a : INT;
END_STRUCT END_TYPE

PROGRAM mainProg
VAR
    x2 : ARRAY [0..1, 0..1] OF INT := [11,12,21,22];
    x3 : ARRAY [0..1] OF ARRAY[0..1] OF INT := [[11,12],[21,22]];

    x2_1 : ARRAY [0..1, 0..1] OF str1 := [(a:=11),(a:=12),(a:=21),(a:=22)];
    x3_1 : ARRAY [0..1] OF ARRAY[0..1] OF str1 := [[(a:=11),(a:=12)],[(a:=21),(a:=22)]];
END_VAR
    ;
END_PROGRAM

FUNCTION main : DINT
    mainProg();

    // Multi-dimensional array of INT
    // CHECK: x2=11,12,21,22
    printf('x2=%d,%d,%d,%d$N', mainProg.x2[0,0], mainProg.x2[0,1], mainProg.x2[1,0], mainProg.x2[1,1]);

    // Array of array of INT
    // CHECK: x3=11,12,21,22
    printf('x3=%d,%d,%d,%d$N', mainProg.x3[0][0], mainProg.x3[0][1], mainProg.x3[1][0], mainProg.x3[1][1]);

    // Multi-dimensional array of struct
    // CHECK: x2_1=11,12,21,22
    printf('x2_1=%d,%d,%d,%d$N', mainProg.x2_1[0,0].a, mainProg.x2_1[0,1].a, mainProg.x2_1[1,0].a, mainProg.x2_1[1,1].a);

    // Array of array of struct
    // CHECK: x3_1=11,12,21,22
    printf('x3_1=%d,%d,%d,%d$N', mainProg.x3_1[0][0].a, mainProg.x3_1[0][1].a, mainProg.x3_1[1][0].a, mainProg.x3_1[1][1].a);

    main := 0;
END_FUNCTION
