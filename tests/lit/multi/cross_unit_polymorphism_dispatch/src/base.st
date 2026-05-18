// Base function block in its own compilation unit. The polymorphism
// participant rewrites this unit (adds a __vtable_Base type and a global
// instance) before per-unit re-indexing makes those new symbols
// queryable.

FUNCTION_BLOCK Base
METHOD PUBLIC kind : DINT
    kind := 1;
END_METHOD
END_FUNCTION_BLOCK
