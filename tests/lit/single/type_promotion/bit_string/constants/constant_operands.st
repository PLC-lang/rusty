// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// Compares and divisions on DWORD constants follow the same signed rules as on variables.
VAR_GLOBAL CONSTANT
    mask : DWORD := 16#FFFF_FFFF;
END_VAR

VAR_GLOBAL
    half : DWORD := mask / 2;
END_VAR

FUNCTION main : DINT
    printf('initializer mask / 2 = %u$N', half);            // CHECK: initializer mask / 2 = 2147483647
    printf('mask > 1 = %d$N', mask > 1);                    // CHECK-NEXT: mask > 1 = 1
END_FUNCTION
