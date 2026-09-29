// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// AND declares a single variadic parameter. A variadic parameter cannot be named, thus AND has an
// implicit (positional) call form only. The form is extensible. AND gives the same result as the
// `AND` operator, for BOOL values as well as for bit and integer values.

FUNCTION main
VAR
    t : BOOL := TRUE;
    f : BOOL := FALSE;
    x : BYTE := 16#F0;
    y : BYTE := 16#3C;
    z : BYTE := 16#18;
    i : DINT := 12;
    j : DINT := 10;
    a : BOOL;
    b : BOOL;
    c : BYTE;
    d : BYTE;
    e : DINT;
END_VAR

a := AND(t, t);
b := AND(t, t, f);
// CHECK: AND=1 0 1 0
printf('AND=%u %u %u %u$N', a, b, t AND t, t AND t AND f);

c := AND(x, y);
d := AND(x, y, z);
e := AND(i, j);
// CHECK: AND-bits=48 16 8
printf('AND-bits=%u %u %d$N', c, d, e);
// CHECK: AND-bits-operator=48 16 8
printf('AND-bits-operator=%u %u %d$N', x AND y, x AND y AND z, i AND j);
END_FUNCTION
