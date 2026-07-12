// Entry point: call the function twice and observe that the feedback wire reads the
// type default on each invocation (per-call VAR storage) -- both calls return 1.
FUNCTION main : DINT
    // CHECK: first = 1
    printf('first = %d$N', myFunc());

    // CHECK: second = 1
    printf('second = %d$N', myFunc());
END_FUNCTION
