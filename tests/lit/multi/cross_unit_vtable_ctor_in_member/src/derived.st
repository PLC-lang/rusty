// Derived FB overrides `kind`. Lives in a separate compilation unit so its
// vtable is emitted in this unit while Wrapper's vtable lives elsewhere.
// Dispatch through a `POINTER TO Base` aimed at `wrapper.member` must
// resolve through the right vtable for the actual `Derived` instance — if
// `Derived`'s `__vtable__ctor` was dropped from `Wrapper`'s constructor
// chain, the dispatch would land on uninitialized memory.

FUNCTION_BLOCK Derived EXTENDS Base
METHOD PUBLIC kind : DINT
    kind := 2;
END_METHOD
END_FUNCTION_BLOCK
