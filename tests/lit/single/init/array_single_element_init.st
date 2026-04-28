// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that single-element array initialization compiles and zero-pads correctly.

PROGRAM mainProg
VAR
    a1 : ARRAY [0..3] OF INT := [11];
    a2 : ARRAY [0..1, 0..1] OF INT := [42];
    a3 : ARRAY [0..1] OF ARRAY[0..1] OF INT := [[99]];
END_VAR
    ;
END_PROGRAM

FUNCTION main : DINT
    mainProg();

    // 1D: first element set, rest zero
    // CHECK: a1=11,0,0,0
    printf('a1=%d,%d,%d,%d$N', mainProg.a1[0], mainProg.a1[1], mainProg.a1[2], mainProg.a1[3]);

    // 2D flat: first element set, rest zero
    // CHECK: a2=42,0,0,0
    printf('a2=%d,%d,%d,%d$N', mainProg.a2[0,0], mainProg.a2[0,1], mainProg.a2[1,0], mainProg.a2[1,1]);

    // Nested: first inner element set, rest zero
    // CHECK: a3=99,0,0,0
    printf('a3=%d,%d,%d,%d$N', mainProg.a3[0][0], mainProg.a3[0][1], mainProg.a3[1][0], mainProg.a3[1][1]);

    main := 0;
END_FUNCTION
