// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// Regression test for PRG-4521: constant expressions (typed literals like
// `UDINT#0 + UDINT#1`, plain arithmetic, and const references) inside array
// initializers must be const-folded. Nested `ARRAY OF ARRAY` initializers
// previously failed codegen with "Cannot generate Literal for BinaryExpression".

TYPE nestedArr : ARRAY[0..1] OF ARRAY[0..1] OF UDINT := [[UDINT#0 + UDINT#1, UDINT#0], [UDINT#0, UDINT#2 - UDINT#1]]; END_TYPE

TYPE withArr : STRUCT
    m : ARRAY[0..1] OF ARRAY[0..1] OF UDINT := [[UDINT#1 + UDINT#1, UDINT#0], [UDINT#0, UDINT#3 - UDINT#1]];
END_STRUCT END_TYPE

VAR_GLOBAL CONSTANT
    two : UDINT := 2;
END_VAR

VAR_GLOBAL
    gNested : ARRAY[0..1] OF ARRAY[0..1] OF UDINT := [[two * UDINT#3, 1 + 2], [5 - 1, two]];
END_VAR

PROGRAM mainProg
VAR
    arr   : ARRAY[0..1] OF UDINT := [UDINT#0 + UDINT#1, UDINT#2 - UDINT#1];
    mArr  : ARRAY[0..1] OF ARRAY[0..1] OF UDINT := [[UDINT#0 + UDINT#1, UDINT#0], [UDINT#0, UDINT#2 - UDINT#1]];
    mArr2 : ARRAY[0..1, 0..1] OF UDINT := [UDINT#0, UDINT#0 + UDINT#1, UDINT#2 - UDINT#1, UDINT#0];
    plain : ARRAY[0..1] OF ARRAY[0..1] OF DINT := [[0 + 1, 0], [0, 2 - 1]];
    tArr  : nestedArr;
    s     : withArr;
END_VAR
    ;
END_PROGRAM

FUNCTION main : DINT
    mainProg();

    // CHECK: arr=1,1
    printf('arr=%d,%d$N', mainProg.arr[0], mainProg.arr[1]);

    // CHECK: mArr=1,0,0,1
    printf('mArr=%d,%d,%d,%d$N', mainProg.mArr[0][0], mainProg.mArr[0][1], mainProg.mArr[1][0], mainProg.mArr[1][1]);

    // CHECK: mArr2=0,1,1,0
    printf('mArr2=%d,%d,%d,%d$N', mainProg.mArr2[0,0], mainProg.mArr2[0,1], mainProg.mArr2[1,0], mainProg.mArr2[1,1]);

    // CHECK: plain=1,0,0,1
    printf('plain=%d,%d,%d,%d$N', mainProg.plain[0][0], mainProg.plain[0][1], mainProg.plain[1][0], mainProg.plain[1][1]);

    // CHECK: tArr=1,0,0,1
    printf('tArr=%d,%d,%d,%d$N', mainProg.tArr[0][0], mainProg.tArr[0][1], mainProg.tArr[1][0], mainProg.tArr[1][1]);

    // CHECK: s.m=2,0,0,2
    printf('s.m=%d,%d,%d,%d$N', mainProg.s.m[0][0], mainProg.s.m[0][1], mainProg.s.m[1][0], mainProg.s.m[1][1]);

    // CHECK: gNested=6,3,4,2
    printf('gNested=%d,%d,%d,%d$N', gNested[0][0], gNested[0][1], gNested[1][0], gNested[1][1]);

    main := 0;
END_FUNCTION
