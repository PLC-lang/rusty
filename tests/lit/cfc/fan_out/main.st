// Entry point: the Tick block output feeds two sinks (outA, outB). Both must read the
// same value, and Tick must run exactly once (calls == 1), not once per consumer.
FUNCTION main : DINT
    FanOut.seed := 7;
    FanOut();
    printf('outA = %d$N', FanOut.outA);
    printf('outB = %d$N', FanOut.outB);
    printf('calls = %d$N', calls);
END_FUNCTION
