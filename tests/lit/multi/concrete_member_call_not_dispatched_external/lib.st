// Old-style library compiled in isolation. Two FBs, no methods, no
// inheritance, no interfaces. FB_A holds a concrete `FB_B` member and
// invokes it via a plain `helper()` call. The compiled .so must call
// FB_B's body directly — at lib-compile time there is no derivation in
// scope, and the call is statically resolvable to FB_B__body.

FUNCTION_BLOCK FB_B
    printf('FB_B body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_A
VAR
    helper : FB_B;
END_VAR
    helper();
END_FUNCTION_BLOCK
