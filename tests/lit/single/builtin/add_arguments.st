// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// ADD declares a single variadic parameter. A variadic parameter cannot be named, thus ADD has an
// implicit (positional) call form only. The form is extensible.

FUNCTION main
VAR
    x : DINT := 9;
    y : DINT := 7331;
    z : DINT := 60;
    a : DINT;
    b : DINT;
END_VAR

a := ADD(x, y);
b := ADD(x, y, z);
// CHECK: ADD=7340 7400
printf('ADD=%d %d$N', a, b);
END_FUNCTION
