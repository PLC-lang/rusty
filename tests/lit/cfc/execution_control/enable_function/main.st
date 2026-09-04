FUNCTION main : DINT
    // EN was never TRUE: the call and its capture were skipped, sum stays 0.
    enable_function();
    // CHECK: idle: sum = 0, done = FALSE
    printf('idle: sum = %d, done = %s$N', enable_function.sum, SEL(enable_function.done, 'FALSE', 'TRUE'));

    // EN is TRUE: the function runs, the return value reaches sum.
    enable_function.trigger := TRUE;
    enable_function();
    // CHECK: run: sum = 7, done = TRUE
    printf('run: sum = %d, done = %s$N', enable_function.sum, SEL(enable_function.done, 'FALSE', 'TRUE'));

    // EN back to FALSE: the capture is skipped, sum yields the last recorded
    // value even though its inputs changed; ENO follows EN.
    enable_function.trigger := FALSE;
    enable_function.a := 100;
    enable_function();
    // CHECK: hold: sum = 7, done = FALSE
    printf('hold: sum = %d, done = %s$N', enable_function.sum, SEL(enable_function.done, 'FALSE', 'TRUE'));
END_FUNCTION
