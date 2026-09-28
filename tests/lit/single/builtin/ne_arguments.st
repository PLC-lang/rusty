// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// NE gives the same result for the implicit (positional) and the explicit (named) call form, in
// declaration order and reordered. A call may also mix both forms: a named argument claims the slot
// its name declares, and the positional arguments fill the remaining slots in the order they are
// written.

FUNCTION main
VAR
    lr : LREAL := 1.5;
    x  : DINT  := 9;
    y  : DINT  := 7331;
    p  : BOOL;
    q  : BOOL;
    r  : BOOL;
END_VAR

p := NE(lr, 0);
q := NE(IN1 := lr, IN2 := 0);
r := NE(IN2 := 0, IN1 := lr);
// CHECK: NE=1 1 1
printf('NE=%d %d %d$N', p, q, r);

// a mix of implicit and explicit arguments
p := NE(lr, IN2 := 0);
q := NE(IN2 := 0, lr);
// CHECK: NE-mixed=1 1
printf('NE-mixed=%d %d$N', p, q);

// parenthesized arguments
p := NE((lr), 0);
q := NE(IN1 := (lr), IN2 := 0);
// CHECK: NE-paren=1 1
printf('NE-paren=%d %d$N', p, q);

// arguments of the same type
p := NE(x, y);
q := NE(x, x);
// CHECK: NE-same-type=1 0
printf('NE-same-type=%d %d$N', p, q);
END_FUNCTION
