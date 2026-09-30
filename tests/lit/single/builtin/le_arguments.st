// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// LE declares a single variadic parameter. A variadic parameter cannot be named, thus LE has an
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

p := LE(c, b);
q := LE(c, b, a);
r := LE(c, a, b);
// CHECK: LE=1 1 0
printf('LE=%d %d %d$N', p, q, r);
END_FUNCTION
