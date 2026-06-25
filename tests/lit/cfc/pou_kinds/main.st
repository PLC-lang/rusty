// Entry point: exercise all three CFC POU kinds — a FUNCTION (called with args),
// a FUNCTION_BLOCK (instantiated), and a PROGRAM (singleton) — all defined in CFC
// and called from ST.
FUNCTION main : DINT
VAR
    adder : CfcAdder;
END_VAR
    // CFC FUNCTION
    printf('func = %d$N', CfcSum(2, 3));

    // CFC FUNCTION_BLOCK
    adder(a := 4, b := 5);
    printf('fb = %d$N', adder.s);

    // CFC PROGRAM (singleton)
    CfcCounter.a := 6;
    CfcCounter.b := 7;
    CfcCounter();
    printf('prog = %d$N', CfcCounter.total);
END_FUNCTION
