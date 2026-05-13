FUNCTION main : DINT
VAR
    base    : Base;
    derived : Derived;
    ptr     : POINTER TO Base;
END_VAR

// Dispatch through a base pointer must call the actual instance's `kind`.
ptr := ADR(base);
// CHECK: kind via base ptr -> base = 1
printf('kind via base ptr -> base = %d$N', ptr^.kind());

ptr := ADR(derived);
// CHECK: kind via base ptr -> derived = 2
printf('kind via base ptr -> derived = %d$N', ptr^.kind());
END_FUNCTION
