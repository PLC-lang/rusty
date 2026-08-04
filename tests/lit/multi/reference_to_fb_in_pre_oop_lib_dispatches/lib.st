// Old-style library using REFERENCE TO (auto-dereferencing pointer)
// instead of POINTER TO. Same library-safety property: the call site
// must dispatch through the vtable so a derived target supplied
// downstream gets its own body invoked.

FUNCTION_BLOCK FB_B
    printf('FB_B body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_A
VAR
    target : REFERENCE TO FB_B;
END_VAR
    target();
END_FUNCTION_BLOCK
