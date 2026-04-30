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
    PRINTF('PT=%lld$N', timers[1].PT);
    // CHECK: PT=100000000
END_FUNCTION
