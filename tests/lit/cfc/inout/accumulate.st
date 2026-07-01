// A function with a VAR_IN_OUT (by-reference) parameter: it both reads and writes `sum`
// in place, and returns the new total.
FUNCTION accumulate : DINT
VAR_INPUT
    value : DINT;
END_VAR
VAR_IN_OUT
    sum : DINT;
END_VAR
    sum := sum + value;
    accumulate := sum;
END_FUNCTION
