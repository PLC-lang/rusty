// Old-style library: FB_A takes a `VAR_IN_OUT FB_B` parameter (an
// implicit pointer in IEC semantics) and invokes the parameter via
// `target()`. No methods, no inheritance, no interfaces. Like the
// POINTER TO case, the lib has no way to know whether a downstream
// consumer will pass a derived instance — the call must be
// vtable-indirected.

FUNCTION_BLOCK FB_B
    printf('FB_B body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_A
VAR_IN_OUT
    target : FB_B;
END_VAR
    target();
END_FUNCTION_BLOCK
