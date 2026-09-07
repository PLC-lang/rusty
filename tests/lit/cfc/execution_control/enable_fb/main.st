FUNCTION main : DINT
    // EN was never TRUE: the block did not run, out and done keep defaults.
    enable_fb();
    // CHECK: idle: out = 0, done = FALSE
    printf('idle: out = %d, done = %s$N', enable_fb.localOut, SEL(enable_fb.done, 'FALSE', 'TRUE'));

    // EN is TRUE: the block runs once, ENO mirrors EN.
    enable_fb.trigger := TRUE;
    enable_fb();
    // CHECK: run: out = 5, done = TRUE
    printf('run: out = %d, done = %s$N', enable_fb.localOut, SEL(enable_fb.done, 'FALSE', 'TRUE'));

    // EN back to FALSE: out yields the last recorded value, ENO follows EN.
    enable_fb.trigger := FALSE;
    enable_fb();
    // CHECK: hold: out = 5, done = FALSE
    printf('hold: out = %d, done = %s$N', enable_fb.localOut, SEL(enable_fb.done, 'FALSE', 'TRUE'));
END_FUNCTION
