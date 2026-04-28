// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// This test verifies that array of struct initialization in the body works correctly
// when the array is declared as VAR_TEMP and only partial elements are provided.
// Case: Missing initialization - only first element is assigned, second should be zero

TYPE myStruct : STRUCT
    a, b : DINT;
    c : ARRAY[0..1] OF DINT;
END_STRUCT
END_TYPE

PROGRAM mainProg
VAR_TEMP
    arr : ARRAY[0..1] OF myStruct;
END_VAR
VAR
    a, b, c, d : DINT;
    e, f, g, h : DINT;
END_VAR
    arr := [(a := 10, b := 20, c := [30, 40])];

    a := arr[0].a;
    b := arr[0].b;
    c := arr[0].c[0];
    d := arr[0].c[1];

    e := arr[1].a;
    f := arr[1].b;
    g := arr[1].c[0];
    h := arr[1].c[1];
END_PROGRAM

FUNCTION main
    mainProg();

    printf('a=%d$N', mainProg.a); // CHECK: a=10
    printf('b=%d$N', mainProg.b); // CHECK: b=20
    printf('c=%d$N', mainProg.c); // CHECK: c=30
    printf('d=%d$N', mainProg.d); // CHECK: d=40
    printf('e=%d$N', mainProg.e); // CHECK: e=0
    printf('f=%d$N', mainProg.f); // CHECK: f=0
    printf('g=%d$N', mainProg.g); // CHECK: g=0
    printf('h=%d$N', mainProg.h); // CHECK: h=0
END_FUNCTION
