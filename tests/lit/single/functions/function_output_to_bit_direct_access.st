// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Regression test for https://github.com/PLC-lang/rusty/issues/1239
// `fun(out => bar.4)` and `myFb(out => bar.4)` previously generated IR that
// computed the bit address but never stored the function's output back into
// the underlying integer.
FUNCTION fun
    VAR_OUTPUT
        foo : BOOL;
    END_VAR
    foo := TRUE;
END_FUNCTION

FUNCTION_BLOCK fb
    VAR_OUTPUT
        foo : BOOL;
    END_VAR
    foo := TRUE;
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        bar  : DINT;
        myFb : fb;
    END_VAR

    bar := 0;
    fun(foo => bar.4);
    PRINTF('after_fun=%d$N', bar);
    // CHECK: after_fun=16

    bar := 0;
    myFb(foo => bar.4);
    PRINTF('after_fb=%d$N', bar);
    // CHECK: after_fb=16
END_FUNCTION
