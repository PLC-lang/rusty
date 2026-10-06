// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// OR declares a single variadic parameter. A variadic parameter cannot be named, thus OR has an
// implicit (positional) call form only. The form is extensible. OR gives the same result as the
// `OR` operator, for BOOL values as well as for bit and integer values.

FUNCTION main
VAR
    t : BOOL := TRUE;
    f : BOOL := FALSE;
    x : BYTE := 16#F0;
    y : BYTE := 16#0C;
    z : BYTE := 16#01;
    i : DINT := 12;
    j : DINT := 10;
    a : BOOL;
    b : BOOL;
    c : BYTE;
    d : BYTE;
    e : DINT;
END_VAR

a := OR(f, f);
b := OR(f, f, t);
// CHECK: OR=0 1 0 1
printf('OR=%u %u %u %u$N', a, b, f OR f, f OR f OR t);

c := OR(x, y);
d := OR(x, y, z);
e := OR(i, j);
// CHECK: OR-bits=252 253 14
printf('OR-bits=%u %u %d$N', c, d, e);
// CHECK: OR-bits-operator=252 253 14
printf('OR-bits-operator=%u %u %d$N', x OR y, x OR y OR z, i OR j);
END_FUNCTION
