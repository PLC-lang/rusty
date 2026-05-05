// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Regression test for https://github.com/PLC-lang/rusty/issues/1275
// Initializing an ARRAY of TON with a repeat-initializer used to overflow the stack.
TYPE TONs: ARRAY[1..50] OF TON := [50(PT:=T#100ms)]; END_TYPE

FUNCTION main : DINT
    VAR
        timers : TONs;
        outVal : BOOL;
        outTime : TIME;
    END_VAR

    // Drive one of the array elements to confirm the initializer was applied.
    timers[1](IN := FALSE, Q => outVal, ET => outTime);
    PRINTF('PT_first=%lld$N', timers[1].PT);
    // CHECK: PT_first=100000000

    // Anchor that the repeat-initializer reached the *last* element too — a
    // regression that initialized only the first slot would still pass the
    // first PRINTF alone.
    PRINTF('PT_last=%lld$N', timers[50].PT);
    // CHECK: PT_last=100000000
END_FUNCTION
