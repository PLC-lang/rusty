// RUN: %COMPILE %s 2> %t.err && %RUN | %CHECK %s
// RUN: not grep -E "error|warning" %t.err
//
// ADD, SUB, MUL and DIV accept bit strings as their operators do, and compute the same values.

FUNCTION main : DINT
VAR
    b : BYTE := 16#FF;
    dw : DWORD := 16#100;
    di : DINT := 7;
END_VAR

// CHECK: ADD BYTE 256 256
printf('ADD BYTE %u %u$N', ADD(b, 1), b + 1);
// CHECK-NEXT: SUB DWORD 255 255
printf('SUB DWORD %u %u$N', SUB(dw, 1), dw - 1);
// CHECK-NEXT: SUB named 255 255
printf('SUB named %u %u$N', SUB(IN2 := 1, IN1 := dw), dw - 1);
// CHECK-NEXT: MUL DWORD 65536 65536
printf('MUL DWORD %u %u$N', MUL(dw, dw), dw * dw);
// CHECK-NEXT: DIV DWORD 128 128
printf('DIV DWORD %u %u$N', DIV(dw, 2), dw / 2);
// CHECK-NEXT: ADD DINT BYTE 262 262
printf('ADD DINT BYTE %d %d$N', ADD(di, b), di + b);
END_FUNCTION
