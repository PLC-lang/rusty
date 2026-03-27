// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that partial array initialization zero-pads at runtime.

PROGRAM mainProg
VAR
    a1 : ARRAY [0..3] OF INT := [10, 20];
    a2 : ARRAY [0..1, 0..1] OF INT := [10, 20, 30];
    a3 : ARRAY [0..1] OF ARRAY[0..1] OF INT := [[10, 20]];
END_VAR
    ;
END_PROGRAM

FUNCTION main : DINT
    mainProg();

    // CHECK: a1=10,20,0,0
    printf('a1=%d,%d,%d,%d$N', mainProg.a1[0], mainProg.a1[1], mainProg.a1[2], mainProg.a1[3]);

    // CHECK: a2=10,20,30,0
    printf('a2=%d,%d,%d,%d$N', mainProg.a2[0,0], mainProg.a2[0,1], mainProg.a2[1,0], mainProg.a2[1,1]);

    // CHECK: a3=10,20,0,0
    printf('a3=%d,%d,%d,%d$N', mainProg.a3[0][0], mainProg.a3[0][1], mainProg.a3[1][0], mainProg.a3[1][1]);

    main := 0;
END_FUNCTION
