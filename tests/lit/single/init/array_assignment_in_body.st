// Test to verify array assignment works in body (not initializer)
// This test confirms that array assignment works correctly when done in the body
// rather than as an initializer, which helps isolate initializer-specific issues.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    g1: INT := 10;
    g2: INT := 20;
END_VAR

TYPE Elem: STRUCT
    r: REF_TO INT := REF(g1);
END_STRUCT END_TYPE

PROGRAM mainProg
VAR
    arr: ARRAY[1..2] OF Elem;
END_VAR
    // Assign array in one go
    arr := [
        (r := REF(g1)),
        (r := REF(g2))
    ];
END_PROGRAM

FUNCTION main : DINT
VAR
    v1: INT;
    v2: INT;
END_VAR
    mainProg();
    v1 := mainProg.arr[1].r^;
    v2 := mainProg.arr[2].r^;
    printf('v1=%d$N', v1); // CHECK: v1=10
    printf('v2=%d$N', v2); // CHECK: v2=20
    mainProg.arr[1].r^ := 33;
    mainProg.arr[2].r^ := 44;
    printf('g1=%d$N', g1); // CHECK: g1=33
    printf('g2=%d$N', g2); // CHECK: g2=44
    main := 0;
END_FUNCTION
