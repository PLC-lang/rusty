// Derived FB in a separate unit; overrides `kind`. The vtable for
// Derived is emitted in this unit, the vtable for Base in base.st —
// dispatch through a `POINTER TO Base` has to resolve through the right
// vtable for the right object.

FUNCTION_BLOCK Derived EXTENDS Base
METHOD PUBLIC kind : DINT
    kind := 2;
END_METHOD
END_FUNCTION_BLOCK
