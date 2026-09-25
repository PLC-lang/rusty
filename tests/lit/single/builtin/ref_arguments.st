// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// REF returns the same pointer for the implicit (positional) and the explicit (named) call form.

FUNCTION main
VAR
    i     : INT := 5;
    ptr_a : REF_TO INT;
    ptr_b : REF_TO INT;
    p     : BOOL;
END_VAR

ptr_a := REF(i);
ptr_b := REF(in := i);
p := ptr_a = ptr_b;
// CHECK: REF=1
printf('REF=%d$N', p);
END_FUNCTION
