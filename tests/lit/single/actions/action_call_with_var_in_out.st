// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression coverage for #1576 (fixed in commit 2b3388f). Calling an action
// that inherits a `VAR_IN_OUT` parameter from its parent FB must store the
// caller's address into the FB's in-out slot before invoking the action.
// Before the fix the slot stayed NULL — the action then dereferenced a null
// pointer and segfaulted at runtime.

PROGRAM mainProg
VAR
    fb : fb_t;
    localBool : BOOL;
END_VAR
    localBool := TRUE;
    fb.foo(myInOut := localBool);

    localBool := FALSE;
    fb.foo(myInOut := localBool);
END_PROGRAM

FUNCTION_BLOCK fb_t
VAR_IN_OUT
    myInOut : BOOL;
END_VAR
END_FUNCTION_BLOCK

ACTIONS
    ACTION foo
        // CHECK: myInout = 1
        // CHECK: myInout = 0
        printf('myInout = %d$N', myInOut);
    END_ACTION
END_ACTIONS

FUNCTION main : DINT
    mainProg();
    main := 0;
END_FUNCTION
