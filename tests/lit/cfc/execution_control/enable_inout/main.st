FUNCTION main : DINT
    // EN is FALSE: the inout is neither evaluated nor transferred.
    enable_inout();
    // CHECK: idle: total = 10, result = 0
    printf('idle: total = %d, result = %d$N', enable_inout.total, enable_inout.result);

    // EN is TRUE: the call writes back through the inout.
    enable_inout.trigger := TRUE;
    enable_inout();
    // CHECK: run: total = 15, result = 15
    printf('run: total = %d, result = %d$N', enable_inout.total, enable_inout.result);

    // EN back to FALSE: no further transfer, both values hold.
    enable_inout.trigger := FALSE;
    enable_inout();
    // CHECK: hold: total = 15, result = 15
    printf('hold: total = %d, result = %d$N', enable_inout.total, enable_inout.result);
END_FUNCTION
