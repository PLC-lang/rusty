// Downstream application: extends FB_B and passes a derived instance
// as the VAR_IN_OUT argument to FB_A. The lib's FB_A.body, compiled in
// isolation, must dispatch `target()` through the vtable so the
// derived body runs.

{external}
FUNCTION_BLOCK FB_B
END_FUNCTION_BLOCK

{external}
FUNCTION_BLOCK FB_A
VAR_IN_OUT
    target : FB_B;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_B_DERIVED EXTENDS FB_B
    printf('FB_B_DERIVED body$N');
END_FUNCTION_BLOCK

FUNCTION main
VAR
    a       : FB_A;
    base    : FB_B;
    derived : FB_B_DERIVED;
END_VAR

a(target := base);
a(target := derived);
END_FUNCTION
