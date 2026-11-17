// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// REAL and LREAL to the integer types truncate toward zero, then keep the low bits of the target
// width, so an out-of-range value wraps and a negative value lands at the top of an unsigned range.
// The truncated value first becomes a signed 64-bit integer; NaN, the infinities and anything outside
// that range become -2^63 instead, so the targets narrower than 64 bits see 0 for them. ULINT is the
// exception: a value in [2^63, 2^64) converts exactly. This is the rule CODESYS 3.5 SP22 follows for
// its real conversions. Each line prints SINT USINT INT UINT DINT UDINT LINT ULINT.
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
// range, wraps modulo the target width
typed_real(REAL#300.7);     // CHECK: 44 44 300 300 300 300 300 300
typed_real(REAL#40000.9);   // CHECK: 64 64 -25536 40000 40000 40000 40000 40000
typed_real(REAL#1.0E10);    // CHECK: 0 0 -7168 58368 1410065408 1410065408 10000000000 10000000000
typed_real(REAL#1.0E19);    // CHECK: 0 0 0 0 0 0 -9223372036854775808 9999999980506447872
typed_real(REAL#1.0E20);    // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808
// not a number
typed_real(INF_REAL);       // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808
typed_real(ninf);           // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808
typed_real(NAN_REAL);       // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808

// ---- LREAL source, typed form
typed_lreal(LREAL#-5.9);    // CHECK: -5 251 -5 65531 -5 4294967291 -5 18446744073709551611
typed_lreal(LREAL#300.7);   // CHECK: 44 44 300 300 300 300 300 300
typed_lreal(LREAL#40000.9); // CHECK: 64 64 -25536 40000 40000 40000 40000 40000
typed_lreal(LREAL#1.0E10);  // CHECK: 0 0 -7168 58368 1410065408 1410065408 10000000000 10000000000
// the 64-bit band: ULINT keeps [2^63, 2^64) exact, the other targets go through the signed path
typed_lreal(LREAL#1.0E19);  // CHECK: 0 0 0 0 0 0 -9223372036854775808 10000000000000000000
typed_lreal(LREAL#-1.0E19); // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808
typed_lreal(LREAL#1.0E20);  // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808
typed_lreal(lninf);         // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808
typed_lreal(NAN_LREAL);     // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808

// ---- the generic form calls the typed one
generic_real(REAL#-5.9);       // CHECK: -5 251 -5 65531 -5 4294967291 -5 18446744073709551611
generic_real(REAL#1.0E10);     // CHECK: 0 0 -7168 58368 1410065408 1410065408 10000000000 10000000000
generic_real(NAN_REAL);        // CHECK: 0 0 0 0 0 0 -9223372036854775808 9223372036854775808
generic_lreal(LREAL#40000.9);  // CHECK: 64 64 -25536 40000 40000 40000 40000 40000
generic_lreal(LREAL#1.0E19);   // CHECK: 0 0 0 0 0 0 -9223372036854775808 10000000000000000000

// ---- the deprecated TRUNC gives LINT
printf('%lld %lld %lld$N', TRUNC(REAL#-5.9), TRUNC(LREAL#1.0E19), TRUNC(NAN_LREAL));  // CHECK: -5 -9223372036854775808 -9223372036854775808
END_FUNCTION
