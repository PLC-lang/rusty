// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    printf('%s$N', REF(TIME_TO_STRING(T#0ms)));              // CHECK: T#0ms
    printf('%s$N', REF(TIME_TO_STRING(T#1s500ms)));          // CHECK-NEXT: T#1s500ms
    printf('%s$N', REF(TIME_TO_STRING(T#1h1s)));              // CHECK-NEXT: T#1h1s
    printf('%s$N', REF(TIME_TO_STRING(T#1d5ms)));             // CHECK-NEXT: T#1d5ms
    printf('%s$N', REF(TIME_TO_STRING(TIME#49d17h2m47s295ms))); // CHECK-NEXT: T#49d17h2m47s295ms
    printf('%s$N', REF(LTIME_TO_STRING(LTIME#1ns)));          // CHECK-NEXT: LTIME#1ns
    printf('%s$N', REF(LTIME_TO_STRING(LTIME#1d7ns)));        // CHECK-NEXT: LTIME#1d7ns
    printf('%s$N', REF(LTIME_TO_STRING(LWORD_TO_LTIME(LWORD#18446744073709551615)))); // CHECK-NEXT: LTIME#213503d23h34m33s709ms551us615ns
    printf('%s$N', REF(TIME_TO_STRING(-1)));                  // CHECK-NEXT: T#49d17h2m47s295ms
    printf('%s$N', REF(LTIME_TO_STRING(-1)));                 // CHECK-NEXT: LTIME#213503d23h34m33s709ms551us615ns
    printf('%s$N', REF(TIME_TO_STRING(T#50d)));               // CHECK-NEXT: T#6h57m12s704ms
    printf('%s$N', REF(TIME_TO_STRING(4320000000)));          // CHECK-NEXT: T#6h57m12s704ms
    printf('%s$N', REF(LTIME_TO_STRING(LTIME#106752d)));      // CHECK-NEXT: LTIME#106751d23h47m16s854ms775us807ns
    printf('%s$N', REF(LTIME_TO_STRING(9223372800000000000)));// CHECK-NEXT: LTIME#106752d
END_FUNCTION
