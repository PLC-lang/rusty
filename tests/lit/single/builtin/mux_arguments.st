// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// MUX names its first parameter K and takes the remaining ones through a variadic parameter. A
// variadic parameter cannot be named, thus only K has an explicit (named) call form.

FUNCTION main
VAR
    x : DINT := 9;
    y : DINT := 7331;
    z : DINT := 42;
    n : DINT := -1;
    a : DINT;
    b : DINT;
    c : DINT;
    lr : LREAL;
END_VAR

a := MUX(1, x, y);
b := MUX(K := 1, x, y);
// CHECK: MUX=7331 7331
printf('MUX=%d %d$N', a, b);

c := MUX(2, x, y, z);
// CHECK: MUX-extended=42
printf('MUX-extended=%d$N', c);

// the variadic arguments resolve the generic type of the mixed call as they do in the positional call,
// thus a negative value keeps its sign when it is widened
lr := MUX(0, n, x);
// CHECK: MUX-widened=-1.0
printf('MUX-widened=%.1f$N', lr);
lr := MUX(K := 0, n, x);
// CHECK: MUX-mixed-widened=-1.0
printf('MUX-mixed-widened=%.1f$N', lr);
END_FUNCTION
