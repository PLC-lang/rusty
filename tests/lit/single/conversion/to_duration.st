// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION main : DINT
    // BOOL and bit strings count milliseconds
    printf('%s$N', REF(TIME_TO_STRING(BOOL_TO_TIME(TRUE))));                // CHECK: T#1ms
    printf('%s$N', REF(TIME_TO_STRING(BYTE_TO_TIME(255))));                 // CHECK-NEXT: T#255ms
    printf('%s$N', REF(TIME_TO_STRING(WORD_TO_TIME(65535))));               // CHECK-NEXT: T#1m5s535ms
    printf('%s$N', REF(TIME_TO_STRING(DWORD_TO_TIME(4294967295))));         // CHECK-NEXT: T#49d17h2m47s295ms

    // unsigned integers zero-extend
    printf('%s$N', REF(TIME_TO_STRING(USINT_TO_TIME(255))));                // CHECK-NEXT: T#255ms
    printf('%s$N', REF(TIME_TO_STRING(UINT_TO_TIME(65535))));               // CHECK-NEXT: T#1m5s535ms
    printf('%s$N', REF(TIME_TO_STRING(UDINT_TO_TIME(2147483648))));         // CHECK-NEXT: T#24d20h31m23s648ms

    // signed integers wrap like a two's complement cast
    printf('%s$N', REF(TIME_TO_STRING(SINT_TO_TIME(127))));                 // CHECK-NEXT: T#127ms
    printf('%s$N', REF(TIME_TO_STRING(SINT_TO_TIME(-1))));                  // CHECK-NEXT: T#49d17h2m47s295ms
    printf('%s$N', REF(TIME_TO_STRING(SINT_TO_TIME(-128))));                // CHECK-NEXT: T#49d17h2m47s168ms
    printf('%s$N', REF(TIME_TO_STRING(INT_TO_TIME(-32768))));               // CHECK-NEXT: T#49d17h2m14s528ms
    printf('%s$N', REF(TIME_TO_STRING(DINT_TO_TIME(2147483647))));          // CHECK-NEXT: T#24d20h31m23s647ms
    printf('%s$N', REF(TIME_TO_STRING(DINT_TO_TIME(-2147483648))));         // CHECK-NEXT: T#24d20h31m23s648ms

    // reals round half away from zero, then wrap
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(0.5))));                 // CHECK-NEXT: T#1ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(2.5))));                 // CHECK-NEXT: T#3ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(-0.4))));                // CHECK-NEXT: T#0ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(-0.5))));                // CHECK-NEXT: T#49d17h2m47s295ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(-1000.0))));             // CHECK-NEXT: T#49d17h2m46s296ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(4294967040.0))));        // CHECK-NEXT: T#49d17h2m47s40ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(4294967296.0))));        // CHECK-NEXT: T#0ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(1.0E10))));              // CHECK-NEXT: T#16d7h41m5s408ms
    printf('%s$N', REF(TIME_TO_STRING(LREAL_TO_TIME(2.5))));                // CHECK-NEXT: T#3ms
    printf('%s$N', REF(TIME_TO_STRING(LREAL_TO_TIME(-1.0))));               // CHECK-NEXT: T#49d17h2m47s295ms
    printf('%s$N', REF(TIME_TO_STRING(LREAL_TO_TIME(4294967296.0))));       // CHECK-NEXT: T#0ms
    printf('%s$N', REF(TIME_TO_STRING(REAL_TO_TIME(3.4E38))));              // CHECK-NEXT: T#0ms
    printf('%s$N', REF(TIME_TO_STRING(LREAL_TO_TIME(-1.0E19))));            // CHECK-NEXT: T#22d22h21m24s352ms

    // BOOL and bit strings count nanoseconds
    printf('%s$N', REF(LTIME_TO_STRING(BOOL_TO_LTIME(TRUE))));              // CHECK-NEXT: LTIME#1ns
    printf('%s$N', REF(LTIME_TO_STRING(BYTE_TO_LTIME(255))));               // CHECK-NEXT: LTIME#255ns
    printf('%s$N', REF(LTIME_TO_STRING(WORD_TO_LTIME(65535))));             // CHECK-NEXT: LTIME#65us535ns
    printf('%s$N', REF(LTIME_TO_STRING(DWORD_TO_LTIME(4294967295))));       // CHECK-NEXT: LTIME#4s294ms967us295ns

    // unsigned integers zero-extend
    printf('%s$N', REF(LTIME_TO_STRING(USINT_TO_LTIME(255))));              // CHECK-NEXT: LTIME#255ns
    printf('%s$N', REF(LTIME_TO_STRING(UINT_TO_LTIME(65535))));             // CHECK-NEXT: LTIME#65us535ns
    printf('%s$N', REF(LTIME_TO_STRING(UDINT_TO_LTIME(2147483648))));       // CHECK-NEXT: LTIME#2s147ms483us648ns

    // signed integers sign-extend
    printf('%s$N', REF(LTIME_TO_STRING(SINT_TO_LTIME(127))));               // CHECK-NEXT: LTIME#127ns
    printf('%s$N', REF(LTIME_TO_STRING(SINT_TO_LTIME(-1))));                // CHECK-NEXT: LTIME#213503d23h34m33s709ms551us615ns
    printf('%s$N', REF(LTIME_TO_STRING(SINT_TO_LTIME(-128))));              // CHECK-NEXT: LTIME#213503d23h34m33s709ms551us488ns
    printf('%s$N', REF(LTIME_TO_STRING(INT_TO_LTIME(-32768))));             // CHECK-NEXT: LTIME#213503d23h34m33s709ms518us848ns
    printf('%s$N', REF(LTIME_TO_STRING(DINT_TO_LTIME(2147483647))));        // CHECK-NEXT: LTIME#2s147ms483us647ns
    printf('%s$N', REF(LTIME_TO_STRING(DINT_TO_LTIME(-2147483648))));       // CHECK-NEXT: LTIME#213503d23h34m31s562ms67us968ns

    // reals round half away from zero, then wrap, keeping the range above LINT max
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(0.5))));               // CHECK-NEXT: LTIME#1ns
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(2.5))));               // CHECK-NEXT: LTIME#3ns
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(-0.4))));              // CHECK-NEXT: LTIME#0ns
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(-0.5))));              // CHECK-NEXT: LTIME#213503d23h34m33s709ms551us615ns
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(-1000.0))));           // CHECK-NEXT: LTIME#213503d23h34m33s709ms550us616ns
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(1.0E10))));            // CHECK-NEXT: LTIME#10s
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(1.8446743E19))));      // CHECK-NEXT: LTIME#213503d23h16m14s197ms923us840ns
    printf('%s$N', REF(LTIME_TO_STRING(LREAL_TO_LTIME(2.5))));              // CHECK-NEXT: LTIME#3ns
    printf('%s$N', REF(LTIME_TO_STRING(LREAL_TO_LTIME(-1.0))));             // CHECK-NEXT: LTIME#213503d23h34m33s709ms551us615ns
    printf('%s$N', REF(LTIME_TO_STRING(LREAL_TO_LTIME(1.0E10))));           // CHECK-NEXT: LTIME#10s
    printf('%s$N', REF(LTIME_TO_STRING(REAL_TO_LTIME(1.8446744E19))));      // CHECK-NEXT: LTIME#0ns
    printf('%s$N', REF(LTIME_TO_STRING(LREAL_TO_LTIME(3.4E38))));           // CHECK-NEXT: LTIME#0ns
END_FUNCTION
