// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// A callee that never assigns its STRING return must yield an empty string, even
// after a previous call left non-empty content in another result temp.

FUNCTION make : STRING
    make := 'leftover content';
END_FUNCTION

FUNCTION partial : STRING
VAR_INPUT
    cond : BOOL;
END_VAR
    IF cond THEN
        partial := 'assigned';
    END_IF;
END_FUNCTION

FUNCTION main : DINT
VAR
    s : STRING;
END_VAR
    s := make();
    // CHECK: len=16
    printf('len=%d$N', LEN(s));

    // the unassigned return must read as empty
    s := partial(FALSE);
    // CHECK: len=0
    printf('len=%d$N', LEN(s));

    s := partial(TRUE);
    // CHECK: assigned
    printf('%s$N', REF(s));
END_FUNCTION
