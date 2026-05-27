// New code that extends FB_B from the legacy lib. The mere presence of
// `EXTENDS` in the project causes the polymorphism dispatch pass to fire
// (vtables-may-be-needed). The pass must still leave FB_A's concrete
// `helper()` call alone — the call site uses a static-typed instance, not
// a `POINTER TO FB_B`, so it does not need vtable indirection.

FUNCTION_BLOCK FB_B_DERIVED EXTENDS FB_B
    counter := counter + 9000;
END_FUNCTION_BLOCK
