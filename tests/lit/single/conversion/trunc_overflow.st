// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// REAL and LREAL to the integer types truncate toward zero. For SINT, USINT, INT, UINT and DINT, the
// truncated value then becomes a signed 32-bit integer and keeps the low bits of the target width;
// NaN, the infinities and anything outside the 32-bit range become -2^31, so DINT sees -2147483648
// and the narrower targets 0. UDINT, LINT and ULINT go through a signed 64-bit integer the same way
// (-2^63 when out of range), so UDINT is exact up to 2^32, and ULINT keeps a value in [2^63, 2^64)
// exact. The INT and DINT cells of the rows that the CODESYS sheet covers were measured on CODESYS
// 3.5 SP22 on x86-64 (TRUNC_INT and TRUNC, the only two truncations it provides); the other rows and
// columns extend the same rule. Each line prints SINT USINT INT UINT DINT UDINT LINT ULINT.
FUNCTION typed_real
VAR_INPUT
    x : REAL;
END_VAR
    printf('%d %d %d %d %d %u %lld %llu$N',
        REAL_TRUNC_SINT(x), REAL_TRUNC_USINT(x), REAL_TRUNC_INT(x), REAL_TRUNC_UINT(x),
        REAL_TRUNC_DINT(x), REAL_TRUNC_UDINT(x), REAL_TRUNC_LINT(x), REAL_TRUNC_ULINT(x));
END_FUNCTION

FUNCTION typed_lreal
VAR_INPUT
    x : LREAL;
END_VAR
    printf('%d %d %d %d %d %u %lld %llu$N',
        LREAL_TRUNC_SINT(x), LREAL_TRUNC_USINT(x), LREAL_TRUNC_INT(x), LREAL_TRUNC_UINT(x),
        LREAL_TRUNC_DINT(x), LREAL_TRUNC_UDINT(x), LREAL_TRUNC_LINT(x), LREAL_TRUNC_ULINT(x));
END_FUNCTION

FUNCTION generic_real
VAR_INPUT
    x : REAL;
END_VAR
    printf('%d %d %d %d %d %u %lld %llu$N',
        TRUNC_SINT(x), TRUNC_USINT(x), TRUNC_INT(x), TRUNC_UINT(x),
        TRUNC_DINT(x), TRUNC_UDINT(x), TRUNC_LINT(x), TRUNC_ULINT(x));
END_FUNCTION

FUNCTION generic_lreal
VAR_INPUT
    x : LREAL;
END_VAR
    printf('%d %d %d %d %d %u %lld %llu$N',
        TRUNC_SINT(x), TRUNC_USINT(x), TRUNC_INT(x), TRUNC_UINT(x),
        TRUNC_DINT(x), TRUNC_UDINT(x), TRUNC_LINT(x), TRUNC_ULINT(x));
END_FUNCTION

FUNCTION main
VAR
    ninf  : REAL;
    lninf : LREAL;
END_VAR
ninf  := -INF_REAL;
lninf := -INF_LREAL;

// ---- REAL source, typed form
// truncation toward zero
typed_real(REAL#5.9);       // CHECK: 5 5 5 5 5 5 5 5
typed_real(REAL#-5.9);      // CHECK: -5 251 -5 65531 -5 4294967291 -5 18446744073709551611
typed_real(REAL#-0.9);      // CHECK: 0 0 0 0 0 0 0 0
typed_real(REAL#-1.9);      // CHECK: -1 255 -1 65535 -1 4294967295 -1 18446744073709551615
// inside the 32-bit range, wraps modulo the target width
typed_real(REAL#300.7);     // CHECK: 44 44 300 300 300 300 300 300
typed_real(REAL#40000.9);   // CHECK: 64 64 -25536 40000 40000 40000 40000 40000
// outside the 32-bit range: -2^31 for SINT to DINT, while UDINT stays exact up to 2^32
typed_real(REAL#3.0E9);     // CHECK: 0 0 0 0 -2147483648 3000000000 3000000000 3000000000
typed_real(REAL#1.0E10);    // CHECK: 0 0 0 0 -2147483648 1410065408 10000000000 10000000000
typed_real(REAL#1.0E19);    // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9999999980506447872
typed_real(REAL#1.0E20);    // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808
// not a number
typed_real(INF_REAL);       // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808
typed_real(ninf);           // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808
typed_real(NAN_REAL);       // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808

// ---- LREAL source, typed form
typed_lreal(LREAL#-5.9);           // CHECK: -5 251 -5 65531 -5 4294967291 -5 18446744073709551611
typed_lreal(LREAL#300.7);          // CHECK: 44 44 300 300 300 300 300 300
typed_lreal(LREAL#40000.9);        // CHECK: 64 64 -25536 40000 40000 40000 40000 40000
// the edges of the 32-bit range
typed_lreal(LREAL#2147483647.9);   // CHECK: -1 255 -1 65535 2147483647 2147483647 2147483647 2147483647
typed_lreal(LREAL#2147483648.0);   // CHECK: 0 0 0 0 -2147483648 2147483648 2147483648 2147483648
typed_lreal(LREAL#-2147483648.9);  // CHECK: 0 0 0 0 -2147483648 2147483648 -2147483648 18446744071562067968
typed_lreal(LREAL#4294967295.9);   // CHECK: 0 0 0 0 -2147483648 4294967295 4294967295 4294967295
typed_lreal(LREAL#4294967296.0);   // CHECK: 0 0 0 0 -2147483648 0 4294967296 4294967296
typed_lreal(LREAL#1.0E10);         // CHECK: 0 0 0 0 -2147483648 1410065408 10000000000 10000000000
// the 64-bit band: ULINT keeps [2^63, 2^64) exact, LINT goes through the signed path
typed_lreal(LREAL#1.0E19);         // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 10000000000000000000
typed_lreal(LREAL#-1.0E19);        // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808
typed_lreal(LREAL#1.0E20);         // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808
typed_lreal(lninf);                // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808
typed_lreal(NAN_LREAL);            // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808

// ---- the generic form calls the typed one
generic_real(REAL#-5.9);       // CHECK: -5 251 -5 65531 -5 4294967291 -5 18446744073709551611
generic_real(REAL#1.0E10);     // CHECK: 0 0 0 0 -2147483648 1410065408 10000000000 10000000000
generic_real(NAN_REAL);        // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 9223372036854775808
generic_lreal(LREAL#40000.9);  // CHECK: 64 64 -25536 40000 40000 40000 40000 40000
generic_lreal(LREAL#1.0E19);   // CHECK: 0 0 0 0 -2147483648 0 -9223372036854775808 10000000000000000000

// ---- the deprecated TRUNC gives DINT, as TRUNC_DINT does
printf('%d %d %d %d$N', TRUNC(REAL#-5.9), TRUNC(LREAL#2147483647.9), TRUNC(LREAL#1.0E19), TRUNC(NAN_LREAL));  // CHECK: -5 2147483647 -2147483648 -2147483648
END_FUNCTION
