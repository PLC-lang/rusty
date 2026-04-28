// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test VAR_INPUT {ref} defaults with STRING/ARRAY and mixed overrides

TYPE ArrType : ARRAY[1..3] OF DINT;
END_TYPE

FUNCTION test : DINT
VAR_INPUT {ref}
    s : STRING;
    a : ArrType;
END_VAR
test := a[1] + a[2] + a[3];
printf('s=%s$N', REF(s));
printf('sum=%d$N', test);
END_FUNCTION

FUNCTION main
VAR
    tmp : DINT;
    s_default : STRING := 'abc';
    s_override : STRING := 'xyz';
    s_both : STRING := 'pqr';
    a_default : ArrType := [1, 2, 3];
    a_override : ArrType := [4, 5, 6];
    a_both : ArrType := [7, 8, 9];
END_VAR
// Defaults (no overrides)
tmp := test(s := s_default, a := a_default);
// CHECK: s=abc
// CHECK: sum=6

// Override string only
tmp := test(s := s_override, a := a_default);
// CHECK: s=xyz
// CHECK: sum=6

// Override array only
tmp := test(s := s_default, a := a_override);
// CHECK: s=abc
// CHECK: sum=15

// Override both
tmp := test(s := s_both, a := a_both);
// CHECK: s=pqr
// CHECK: sum=24
END_FUNCTION
