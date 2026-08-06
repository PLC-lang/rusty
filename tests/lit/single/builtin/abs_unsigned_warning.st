// RUN: %COMPILE %s 2>&1 | %CHECK %s
//
// ABS of an unsigned value has no effect and reports a warning for each call.

FUNCTION main
VAR
    u8  : USINT;
    u16 : UINT;
    u32 : UDINT;
    u64 : ULINT;
    res : ULINT;
END_VAR

// CHECK: {{.*}}warning[E150]{{.*}}ABS on a value of unsigned type 'USINT' has no effect{{.*}}
res := ABS(u8);

// CHECK: {{.*}}warning[E150]{{.*}}ABS on a value of unsigned type 'UINT' has no effect{{.*}}
res := ABS(u16);

// CHECK: {{.*}}warning[E150]{{.*}}ABS on a value of unsigned type 'UDINT' has no effect{{.*}}
res := ABS(u32);

// CHECK: {{.*}}warning[E150]{{.*}}ABS on a value of unsigned type 'ULINT' has no effect{{.*}}
res := ABS(u64);
END_FUNCTION
