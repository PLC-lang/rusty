FUNCTION main : DINT
    // 2^24 + 1 is not representable in REAL; an exact sum proves the
    // accumulator stayed DINT and was never promoted to floating point.
    add_feedback.seed := 16_777_217;
    add_feedback();
    add_feedback();
    add_feedback();
    // CHECK: acc = 50331651
    printf('acc = %d$N', add_feedback.acc);
END_FUNCTION
