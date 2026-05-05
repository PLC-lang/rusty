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

    // Non-zero seed: anchors that bits 0–3 are *preserved* when the FB output
    // writes bit 4. A regression that clobbered the integer (e.g. wrote a full
    // word instead of just the bit) would surface here as 16 instead of 17.
    bar := 1;
    myFb(foo => bar.4);
    PRINTF('after_fb_seeded=%d$N', bar);
    // CHECK: after_fb_seeded=17
END_FUNCTION
