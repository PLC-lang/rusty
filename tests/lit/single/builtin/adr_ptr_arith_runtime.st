// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// `ADR(x) + N` is byte-stride pointer arithmetic — `ADR` is declared to
// return LWORD, so the `+` flows through the integer-arithmetic path; the
// receiver pointer ends up `N` bytes past the operand. Regression for #1749
// (previously panicked in `create_llvm_int_binary_expression`).

FUNCTION main
VAR
    arr : ARRAY[0..7] OF BYTE := [0, 0, 0, 0, 0, 0, 0, 0];
    pt  : REF_TO BYTE;
END_VAR

pt := ADR(arr) + 4;
pt^ := BYTE#16#AB;

// CHECK: arr[4]=171
printf('arr[4]=%d$N', arr[4]);

pt := ADR(arr) + 7;
pt^ := BYTE#16#5C;

// CHECK: arr[7]=92
printf('arr[7]=%d$N', arr[7]);

// Subtraction flows through the same path: take a pointer past the end
// and walk back. Writing to arr[2] via `ADR(arr[6]) - 4`.
pt := ADR(arr[6]) - 4;
pt^ := BYTE#16#11;

// CHECK: arr[2]=17
printf('arr[2]=%d$N', arr[2]);
END_FUNCTION
