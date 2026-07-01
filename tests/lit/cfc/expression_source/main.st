// Entry point: set localA, run the program, and observe `result := localA + 5`.
// If the data source "localA + 5" were NOT parsed as an expression (treated as a bare
// one-token name instead), compilation would fail on the unresolvable variable and this
// test would produce no output -- so a passing `result = 42` proves it was parsed.
FUNCTION main : DINT
    Expr.localA := 37;
    Expr();
    printf('result = %d$N', Expr.result);
END_FUNCTION
