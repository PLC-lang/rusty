FUNCTION main : DINT
VAR
    w   : Wrapper;
    ptr : POINTER TO Base;
END_VAR

// Dispatch through a base pointer aimed at the embedded Derived. This
// only works if Wrapper's constructor chain set up `w.member.__vtable`,
// which in turn requires `Derived__ctor(self.member)` to be present in
// Wrapper's synthesised constructor.
ptr := ADR(w.member);

// CHECK: kind via base ptr -> wrapper.member = 2
printf('kind via base ptr -> wrapper.member = %d$N', ptr^.kind());
END_FUNCTION
