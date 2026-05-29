// Old-style library: two FBs, no methods, no inheritance. FB_A holds a
// concrete `FB_B` member and invokes it via a plain `helper()` call (no
// pointer, no interface, no SUPER). This is the call we want to keep
// non-polymorphic — the static type of `helper` is `FB_B` and the body
// that runs must be FB_B__body, even if a third unit in the same
// compilation extends FB_B.

VAR_GLOBAL
    counter : DINT := 0;
END_VAR

FUNCTION_BLOCK FB_B
    counter := counter + 100;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_A
VAR
    helper : FB_B;
END_VAR
    helper();
END_FUNCTION_BLOCK
