
// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test VAR_INPUT defaults with WSTRING and overrides

TYPE ArrType : ARRAY[1..3] OF DINT;
END_TYPE

FUNCTION test : DINT
VAR_INPUT
    s : STRING := 'abc';
    ws : WSTRING := "def";
    a : ArrType := [1, 2, 3];
    i : DINT := 10;
END_VAR
test := a[1] + a[2] + a[3] + i;
printf('s=%s$N', REF(s));
printf('ws=%s$N', REF(TO_STRING(ws)));
printf('sum=%d$N', test);
END_FUNCTION

FUNCTION main : DINT
    // Case 1: all defaults
    test();
    // CHECK: s=abc
    // CHECK: ws=def
    // CHECK: sum=16

    // Case 2: override one input
    test(s := 'xyz');
    // CHECK: s=xyz
    // CHECK: ws=def
    // CHECK: sum=16

    // Case 3: override WSTRING and array
    test(ws := "uvw", a := [4, 5, 6]);
    // CHECK: s=abc
    // CHECK: ws=uvw
    // CHECK: sum=25

    // Case 4: override all inputs
    test(s := 'qqq', ws := "rst", a := [7, 8, 9], i := 1);
    // CHECK: s=qqq
    // CHECK: ws=rst
    // CHECK: sum=25
END_FUNCTION
