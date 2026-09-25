// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// LT declares a single variadic parameter. A variadic parameter cannot be named, thus LT has an
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

p := LT(c, b);
q := LT(c, b, a);
r := LT(c, a, b);
// CHECK: LT=1 1 0
printf('LT=%d %d %d$N', p, q, r);
END_FUNCTION
