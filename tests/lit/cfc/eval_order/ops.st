// Two order-sensitive operations on a shared global, so the runtime result reveals the
// order the CFC blocks actually executed in.
VAR_GLOBAL
    acc : DINT;
END_VAR

FUNCTION DoubleIt : DINT
    acc := acc * 2;
    DoubleIt := acc;
END_FUNCTION

FUNCTION AddOne : DINT
    acc := acc + 1;
    AddOne := acc;
END_FUNCTION
