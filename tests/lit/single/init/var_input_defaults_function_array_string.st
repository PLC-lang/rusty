// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test VAR_INPUT defaults with STRING/ARRAY and mixed overrides

TYPE ArrType : ARRAY[1..3] OF DINT;
END_TYPE

FUNCTION test : DINT
VAR_INPUT
    s : STRING := 'abc';
    a : ArrType := [1, 2, 3];
END_VAR
test := a[1] + a[2] + a[3];
printf('s=%s$N', REF(s));
printf('sum=%d$N', test);
END_FUNCTION

FUNCTION main
VAR
    tmp : DINT;
END_VAR
// Defaults (no overrides)
tmp := test();
// CHECK: s=abc
// CHECK: sum=6
printf('case=default$N'); // CHECK: case=default

// Override string only
tmp := test(s := 'xyz');
// CHECK: s=xyz
// CHECK: sum=6
printf('case=string$N'); // CHECK: case=string

// Override array only
tmp := test(a := [4, 5, 6]);
// CHECK: s=abc
// CHECK: sum=15
printf('case=array$N'); // CHECK: case=array

// Override both
tmp := test(s := 'pqr', a := [7, 8, 9]);
// CHECK: s=pqr
// CHECK: sum=24
printf('case=both$N'); // CHECK: case=both
END_FUNCTION
