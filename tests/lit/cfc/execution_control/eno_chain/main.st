FUNCTION main : DINT
    // One signal switches the whole cascade off: nothing ran, all defaults.
    eno_chain();
    // CHECK: idle: result = 0, done = FALSE
    printf('idle: result = %d, done = %s$N', eno_chain.result, SEL(eno_chain.done, 'FALSE', 'TRUE'));

    // The signal switches the whole cascade on: both blocks run in order.
    eno_chain.trigger := TRUE;
    eno_chain();
    // CHECK: run: result = 5, done = TRUE
    printf('run: result = %d, done = %s$N', eno_chain.result, SEL(eno_chain.done, 'FALSE', 'TRUE'));

    // Off again: both blocks hold, the tail ENO follows the head EN.
    eno_chain.trigger := FALSE;
    eno_chain();
    // CHECK: hold: result = 5, done = FALSE
    printf('hold: result = %d, done = %s$N', eno_chain.result, SEL(eno_chain.done, 'FALSE', 'TRUE'));
END_FUNCTION
