// Downstream application linking against libnopoly.so. Re-declares the
// shape of FB_B and FB_A as `{external}` so codegen emits a reference
// rather than a definition. Adds a derived FB_B_DERIVED EXTENDS FB_B.
//
// The mere presence of `EXTENDS` here causes the polymorphism dispatch
// pass to fire while compiling *this* unit. The library has already been
// compiled in isolation — FB_A's `helper()` call inside it is already
// emitted as a direct call to FB_B__body. Running main() must print
// FB_B's body output, NOT FB_B_DERIVED's, even though both types exist
// in the final binary.

{external}
FUNCTION_BLOCK FB_B
END_FUNCTION_BLOCK

{external}
FUNCTION_BLOCK FB_A
VAR
    helper : FB_B;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_B_DERIVED EXTENDS FB_B
    printf('FB_B_DERIVED body$N');
END_FUNCTION_BLOCK

FUNCTION main
VAR
    a       : FB_A;
    derived : FB_B_DERIVED;
END_VAR

a();
derived();
END_FUNCTION
