// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// NOT gives the same result for the implicit (positional) and the explicit (named) call form. NOT
// gives the same result as the `NOT` operator, for BOOL values as well as for bit and integer values.

FUNCTION main
VAR
    t : BOOL := TRUE;
    x : BYTE := 16#F0;
    i : DINT := 12;
    a : BOOL;
    b : BOOL;
    c : BYTE;
    d : BYTE;
    e : DINT;
END_VAR

a := NOT(t);
b := NOT(IN := t);
// CHECK: NOT=0 0 0
printf('NOT=%u %u %u$N', a, b, NOT t);

c := NOT(x);
d := NOT(IN := x);
e := NOT(IN := i);
// CHECK: NOT-bits=15 15 -13
printf('NOT-bits=%u %u %d$N', c, d, e);
// CHECK: NOT-bits-operator=15 -13
printf('NOT-bits-operator=%u %d$N', NOT x, NOT i);
END_FUNCTION
