// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// EQ declares a single variadic parameter. A variadic parameter cannot be named, thus EQ has an
// implicit (positional) call form only. The form is extensible: every neighbouring pair is compared
// and the results are combined with AND.

FUNCTION main
VAR
    a : DINT := 30;
    b : DINT := 20;
    c : DINT := 10;
    p : BOOL;
    q : BOOL;
    r : BOOL;
END_VAR

p := EQ(b, b);
q := EQ(b, b, b);
r := EQ(a, b, b);
// CHECK: EQ=1 1 0
printf('EQ=%d %d %d$N', p, q, r);
END_FUNCTION
