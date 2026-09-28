// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// SUB gives the same result for the implicit (positional) and the explicit (named) call form, in
// declaration order and reordered. A call may also mix both forms: a named argument claims the slot
// its name declares, and the positional arguments fill the remaining slots in the order they are
// written.

FUNCTION main
VAR
    x   : DINT  := 9;
    y   : DINT  := 7331;
    lr  : LREAL := 1.5;
    lrr : LREAL;
    a   : DINT;
    b   : DINT;
    c   : DINT;
END_VAR

a := SUB(y, x);
b := SUB(IN1 := y, IN2 := x);
c := SUB(IN2 := x, IN1 := y);
// CHECK: SUB=7322 7322 7322
printf('SUB=%d %d %d$N', a, b, c);

// named argument before the positional one, IN1 is filled by y
a := SUB(IN2 := x, y);
// CHECK: SUB-named-first=7322
printf('SUB-named-first=%d$N', a);

// arguments of a different type
lrr := SUB(lr, 1);
// CHECK: SUB-mixed-types=0.5
printf('SUB-mixed-types=%.1f$N', lrr);
lrr := SUB(IN1 := lr, IN2 := 1);
// CHECK: SUB-mixed-types-named=0.5
printf('SUB-mixed-types-named=%.1f$N', lrr);

// parenthesized arguments
lrr := SUB((lr), 1);
// CHECK: SUB-paren=0.5
printf('SUB-paren=%.1f$N', lrr);
lrr := SUB(IN1 := (lr), IN2 := 1);
// CHECK: SUB-paren-named=0.5
printf('SUB-paren-named=%.1f$N', lrr);
END_FUNCTION
