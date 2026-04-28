// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    VAR
        var_sint    : SINT;
        var_int     : INT;
        var_dint    : DINT;
        var_lint    : LINT;

        var_usint   : USINT;
        var_uint    : UINT;
        var_udint   : UDINT;
        var_ulint   : ULINT;
    END_VAR

    // Set max values
    // 8-bit [(2^7)-1]/[(2^8)-1]
    var_sint := 127;                    // Min: -128
    var_usint := 255;                   // Min: 0

    // 16-bit [(2^15)-1]/[(2^16)-1]
    var_int := 32767;                   // Min: -32768
    var_uint := 65535;                  // Min: 0

    // 32-bit [(2^31)-1]/[(2^31)-1]
    var_dint := 2147483647;             // Min: -2147483648
    var_udint := 4294967295;            // Min: 0

    // 64-bit [(2^63)-1]/[(2^64)-1]
    var_lint := 9223372036854775807;    // Min: -9223372036854775808
    var_ulint := 18446744073709551615;  // Min: 0

    // Print and check values
    printf('%d$N', var_sint);       // CHECK: 127
    printf('%d$N', var_int);        // CHECK-NEXT: 32767
    printf('%d$N', var_dint);       // CHECK-NEXT: 2147483647
    printf('%lld$N', var_lint);     // CHECK-NEXT: 9223372036854775807

    printf('%u$N', var_usint);      // CHECK-NEXT: 255
    printf('%u$N', var_uint);       // CHECK-NEXT: 65535
    printf('%u$N', var_udint);      // CHECK-NEXT: 4294967295
    printf('%llu$N', var_ulint);    // CHECK-NEXT: 18446744073709551615

    // Print overflow values
    var_sint := var_sint + 1;
    var_int := var_int + 1;
    var_dint := var_dint + 1;
    var_lint := var_lint + 1;

    var_usint := var_usint + 1;
    var_uint := var_uint + 1;
    var_udint := var_udint + 1;
    var_ulint := var_ulint + 1;

    printf('%d$N', var_sint);       // CHECK: -128
    printf('%d$N', var_int);        // CHECK-NEXT: -32768
    printf('%d$N', var_dint);       // CHECK-NEXT: -2147483648
    printf('%lld$N', var_lint);     // CHECK-NEXT: -9223372036854775808

    printf('%u$N', var_usint);      // CHECK-NEXT: 0
    printf('%u$N', var_uint);       // CHECK-NEXT: 0
    printf('%u$N', var_udint);      // CHECK-NEXT: 0
    printf('%llu$N', var_ulint);    // CHECK-NEXT: 0

END_FUNCTION
