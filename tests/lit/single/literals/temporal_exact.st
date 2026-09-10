// Temporal literals are evaluated with exact integer arithmetic: no 32-bit truncation of a
// nanosecond segment, no rounding of a fraction into the next second, and the same value whether
// the literal is written in code or converted from a string at runtime.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
VAR
    big_ns       : LTIME := LTIME#5000000000ns;
    fraction     : LTIME := LTIME#1.5h30m;
    plus         : TIME  := T#+1s;
    digit_groups : TIME  := T#1_000ms;
    exact_max    : LTIME := LTIME#106751d23h47m16s854ms775us807ns;
    tod_fraction : LTOD  := LTOD#23:59:59.9999999999;
    dt_fraction  : DT    := DT#2024-01-01-23:59:59.9999999999;
END_VAR
    printf('big_ns=%s$N', REF(LTIME_TO_STRING(big_ns)));             // CHECK: big_ns=LTIME#5s
    printf('fraction=%s$N', REF(LTIME_TO_STRING(fraction)));         // CHECK-NEXT: fraction=LTIME#2h
    printf('plus=%s$N', REF(TIME_TO_STRING(plus)));                  // CHECK-NEXT: plus=T#1s
    printf('digit_groups=%s$N', REF(TIME_TO_STRING(digit_groups)));  // CHECK-NEXT: digit_groups=T#1s
    printf('exact_max=%s$N', REF(LTIME_TO_STRING(exact_max)));       // CHECK-NEXT: exact_max=LTIME#106751d23h47m16s854ms775us807ns
    printf('tod_fraction=%s$N', REF(LTOD_TO_STRING(tod_fraction)));  // CHECK-NEXT: tod_fraction=LTOD#23:59:59.999999999
    printf('dt_fraction=%s$N', REF(DT_TO_STRING(dt_fraction)));      // CHECK-NEXT: dt_fraction=DT#2024-01-01-23:59:59.999999999

    printf('same_as_runtime=%d$N', big_ns = STRING_TO_LTIME('LTIME#5000000000ns') AND exact_max = STRING_TO_LTIME('LTIME#106751d23h47m16s854ms775us807ns')); // CHECK-NEXT: same_as_runtime=1
END_FUNCTION
