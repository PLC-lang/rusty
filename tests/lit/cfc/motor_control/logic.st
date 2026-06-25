// Boolean operator wrappers — stand-ins for AND/OR operator blocks (see ../OPEN.md).
FUNCTION MyAnd : BOOL
VAR_INPUT
    a, b : BOOL;
END_VAR
    MyAnd := a AND b;
END_FUNCTION

FUNCTION MyOr : BOOL
VAR_INPUT
    a, b : BOOL;
END_VAR
    MyOr := a OR b;
END_FUNCTION

// Reset-dominant set/reset latch — the motor's seal-in memory. Retains `q` between
// scans; reset wins when both inputs are asserted (safety: stop/e-stop dominate).
FUNCTION_BLOCK SrLatch
VAR_INPUT
    set : BOOL;
    reset : BOOL;
END_VAR
VAR_OUTPUT
    q : BOOL;
END_VAR
    IF reset THEN
        q := FALSE;
    ELSIF set THEN
        q := TRUE;
    END_IF;
END_FUNCTION_BLOCK
