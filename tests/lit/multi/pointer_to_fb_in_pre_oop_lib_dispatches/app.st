// Downstream application: extends FB_B and assigns a derived instance
// to FB_A's pointer member, then calls FB_A. The body of FB_A is
// already in the linked `libdispatchptr.so` — if the lib's compilation
// resolved `inst_fb_b^()` statically as a direct call to FB_B's body,
// the derived instance's body never runs.

{external}
FUNCTION_BLOCK FB_B
END_FUNCTION_BLOCK

{external}
FUNCTION_BLOCK FB_A
VAR
    inst_fb_b : POINTER TO FB_B;
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

// First call: pointer targets a concrete FB_B. The lib's FB_A.body
// invokes `inst_fb_b^()` — vtable lookup yields FB_B's body.
a.inst_fb_b := ADR(base);
a();

// Second call: pointer retargeted to a FB_B_DERIVED instance. The
// lib's FB_A.body, compiled in isolation, must dispatch through the
// vtable so that the derived body runs.
a.inst_fb_b := ADR(derived);
a();
END_FUNCTION
