// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// GT declares a single variadic parameter. A variadic parameter cannot be named, thus GT has an
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

p := GT(a, b);
q := GT(a, b, c);
r := GT(a, c, b);
// CHECK: GT=1 1 0
printf('GT=%d %d %d$N', p, q, r);
END_FUNCTION
