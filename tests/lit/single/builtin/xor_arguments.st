// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// XOR declares a single variadic parameter. A variadic parameter cannot be named, thus XOR has an
// implicit (positional) call form only. The form is extensible. XOR gives the same result as the
// `XOR` operator, for BOOL values as well as for bit and integer values.

FUNCTION main
VAR
    t : BOOL := TRUE;
    f : BOOL := FALSE;
    x : BYTE := 16#F0;
    y : BYTE := 16#3C;
    z : BYTE := 16#01;
    i : DINT := 12;
    j : DINT := 10;
    a : BOOL;
    b : BOOL;
    c : BYTE;
    d : BYTE;
    e : DINT;
END_VAR

a := XOR(t, f);
b := XOR(t, f, t);
// CHECK: XOR=1 0 1 0
printf('XOR=%u %u %u %u$N', a, b, t XOR f, t XOR f XOR t);

c := XOR(x, y);
d := XOR(x, y, z);
e := XOR(i, j);
// CHECK: XOR-bits=204 205 6
printf('XOR-bits=%u %u %d$N', c, d, e);
// CHECK: XOR-bits-operator=204 205 6
printf('XOR-bits-operator=%u %u %d$N', x XOR y, x XOR y XOR z, i XOR j);
END_FUNCTION
