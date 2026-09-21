// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// NE accepts its arguments by name and gives the same result as the positional form.

FUNCTION main
VAR
    lr : LREAL := 1.5;
    bl : BOOL;
END_VAR

bl := NE(lr, 0);
// CHECK: positional=1
printf('positional=%d$N', bl);

bl := NE(IN1 := lr, IN2 := 0);
// CHECK: named=1
printf('named=%d$N', bl);

bl := NE(IN2 := 0, IN1 := lr);
// CHECK: named-reordered=1
printf('named-reordered=%d$N', bl);

bl := NE(lr, IN2 := 0);
// CHECK: mixed=1
printf('mixed=%d$N', bl);
END_FUNCTION
