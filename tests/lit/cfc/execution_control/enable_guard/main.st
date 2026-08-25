FUNCTION main : DINT
    // Divisor 0: the guard computes FALSE, the division never runs.
    enable_guard();
    // CHECK: guarded: result = 0
    printf('guarded: result = %d$N', enable_guard.result);

    // Divisor 4: the guard computes TRUE, the division runs.
    enable_guard.b := 4;
    enable_guard();
    // CHECK: run: result = 5
    printf('run: result = %d$N', enable_guard.result);

    // Divisor back to 0: the call is skipped, result holds the last value.
    enable_guard.b := 0;
    enable_guard();
    // CHECK: hold: result = 5
    printf('hold: result = %d$N', enable_guard.result);
END_FUNCTION
