// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Regression test for https://github.com/PLC-lang/rusty/issues/1275
// Initializing an ARRAY of TON with a repeat-initializer used to overflow the stack.
TYPE TONs: ARRAY[1..50] OF TON := [50(PT:=T#100ms)]; END_TYPE
TYPE LTONs: ARRAY[1..50] OF TON_LTIME := [50(PT:=LT#100ms)]; END_TYPE

FUNCTION main : DINT
    VAR
        timers : TONs;
        ltimers : LTONs;
        outVal : BOOL;
        outTime : TIME;
        outLTime : LTIME;
    END_VAR

    // Drive one of the array elements to confirm the initializer was applied.
    timers[1](IN := FALSE, Q => outVal, ET => outTime);
    PRINTF('PT_first=%lld$N', timers[1].PT);
    // Updated for short TIME millisecond semantics (was nanoseconds before the migration).
    // CHECK: PT_first=100

    // Anchor that the repeat-initializer reached the *last* element too — a
    // regression that initialized only the first slot would still pass the
    // first PRINTF alone.
    PRINTF('PT_last=%lld$N', timers[50].PT);
    // Updated for short TIME millisecond semantics (was nanoseconds before the migration).
    // CHECK: PT_last=100

    // Added LTIME coverage: verify repeat-initializer applies to TON_LTIME arrays too.
    ltimers[1](IN := FALSE, Q => outVal, ET => outLTime);
    PRINTF('LPT_first=%lld$N', ltimers[1].PT);
    // CHECK: LPT_first=100000000

    PRINTF('LPT_last=%lld$N', ltimers[50].PT);
    // CHECK: LPT_last=100000000
END_FUNCTION
