// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// GE declares a single variadic parameter. A variadic parameter cannot be named, thus GE has an
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

p := GE(a, b);
q := GE(a, b, c);
r := GE(a, c, b);
// CHECK: GE=1 1 0
printf('GE=%d %d %d$N', p, q, r);
END_FUNCTION
