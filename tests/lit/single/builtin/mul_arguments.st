// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// MUL declares a single variadic parameter. A variadic parameter cannot be named, thus MUL has an
// implicit (positional) call form only. The form is extensible.

FUNCTION main
VAR
    x : DINT := 9;
    a : DINT;
    b : DINT;
END_VAR

a := MUL(x, 3);
b := MUL(x, 3, 2);
// CHECK: MUL=27 54
printf('MUL=%d %d$N', a, b);
END_FUNCTION
