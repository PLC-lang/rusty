// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// Every builtin function that declares named parameters must give the same result for the
// implicit (positional) and the explicit (named) call form, in declaration order and reordered.
//
// ADD, MUL, GT, GE, EQ, LE and LT declare a single variadic parameter. A variadic parameter
// cannot be named, thus these functions have an implicit call form only. MUX names its first
// parameter K and takes the remaining ones through a variadic parameter.

FUNCTION main
VAR
    i     : INT   := 5;
    x     : DINT  := 9;
    y     : DINT  := 7331;
    neg   : DINT  := -42;
    lr    : LREAL := 1.5;
    lrr   : LREAL;
    arr   : ARRAY[0..9] OF BYTE := [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ptr_a : REF_TO INT;
    ptr_b : REF_TO INT;
    a     : DINT;
    b     : DINT;
    c     : DINT;
    p     : BOOL;
    q     : BOOL;
    r     : BOOL;
END_VAR

// ADR
p := ADR(i) = ADR(in := i);
// CHECK: ADR=1
printf('ADR=%d$N', p);

// REF
ptr_a := REF(i);
ptr_b := REF(in := i);
p := ptr_a = ptr_b;
// CHECK: REF=1
printf('REF=%d$N', p);

// MOVE
a := MOVE(y);
b := MOVE(in := y);
// CHECK: MOVE=7331 7331
printf('MOVE=%d %d$N', a, b);

// SIZEOF
a := SIZEOF(arr);
b := SIZEOF(in := arr);
// CHECK: SIZEOF=10 10
printf('SIZEOF=%d %d$N', a, b);

// ABS
a := ABS(neg);
b := ABS(IN := neg);
// CHECK: ABS=42 42
printf('ABS=%d %d$N', a, b);

// SEL
a := SEL(TRUE, x, y);
b := SEL(G := TRUE, IN0 := x, IN1 := y);
c := SEL(IN1 := y, IN0 := x, G := TRUE);
// CHECK: SEL=7331 7331 7331
printf('SEL=%d %d %d$N', a, b, c);

// MUX, only K can be named
a := MUX(1, x, y);
b := MUX(K := 1, x, y);
// CHECK: MUX=7331 7331
printf('MUX=%d %d$N', a, b);

// SUB
a := SUB(y, x);
b := SUB(IN1 := y, IN2 := x);
c := SUB(IN2 := x, IN1 := y);
// CHECK: SUB=7322 7322 7322
printf('SUB=%d %d %d$N', a, b, c);

// SUB with arguments of a different type
lrr := SUB(lr, 1);
// CHECK: SUB-mixed=0.5
printf('SUB-mixed=%.1f$N', lrr);
lrr := SUB(IN1 := lr, IN2 := 1);
// CHECK: SUB-mixed-named=0.5
printf('SUB-mixed-named=%.1f$N', lrr);

// DIV
a := DIV(y, x);
b := DIV(IN1 := y, IN2 := x);
c := DIV(IN2 := x, IN1 := y);
// CHECK: DIV=814 814 814
printf('DIV=%d %d %d$N', a, b, c);

// NE
p := NE(lr, 0);
q := NE(IN1 := lr, IN2 := 0);
r := NE(IN2 := 0, IN1 := lr);
// CHECK: NE=1 1 1
printf('NE=%d %d %d$N', p, q, r);

// NE with a mix of implicit and explicit arguments
p := NE(lr, IN2 := 0);
// CHECK: NE-mixed=1
printf('NE-mixed=%d$N', p);

// SHL
a := SHL(x, 2);
b := SHL(IN := x, n := 2);
c := SHL(n := 2, IN := x);
// CHECK: SHL=36 36 36
printf('SHL=%d %d %d$N', a, b, c);

// SHR
a := SHR(x, 2);
b := SHR(IN := x, n := 2);
c := SHR(n := 2, IN := x);
// CHECK: SHR=2 2 2
printf('SHR=%d %d %d$N', a, b, c);

// Variadic builtins, implicit form only
a := ADD(x, y);
// CHECK: ADD=7340
printf('ADD=%d$N', a);

a := MUL(x, 3);
// CHECK: MUL=27
printf('MUL=%d$N', a);

p := GT(y, x);
q := GE(y, x);
r := EQ(x, x);
// CHECK: GT-GE-EQ=1 1 1
printf('GT-GE-EQ=%d %d %d$N', p, q, r);

p := LT(x, y);
q := LE(x, y);
r := NE(x, y);
// CHECK: LT-LE-NE=1 1 1
printf('LT-LE-NE=%d %d %d$N', p, q, r);

bounds(arr);
END_FUNCTION

FUNCTION bounds : DINT
VAR_IN_OUT
    vla : ARRAY[*] OF BYTE;
END_VAR
VAR
    dim : DINT := 1;
    a   : DINT;
    b   : DINT;
END_VAR

// LOWER_BOUND
a := LOWER_BOUND(vla, dim);
b := LOWER_BOUND(arr := vla, dim := dim);
// CHECK: LOWER_BOUND=0 0
printf('LOWER_BOUND=%d %d$N', a, b);

// UPPER_BOUND
a := UPPER_BOUND(vla, dim);
b := UPPER_BOUND(arr := vla, dim := dim);
// CHECK: UPPER_BOUND=9 9
printf('UPPER_BOUND=%d %d$N', a, b);
END_FUNCTION
