// RUN: %COMPILE %s -lm && %RUN | %CHECK %s
//
// MOD gives the same result for the implicit (positional) and the explicit (named) call form, in
// declaration order and reordered. MOD gives the same result as the `MOD` operator, for integer as
// well as for REAL values, and for two TIME or two LTIME values. Without optimization a REAL remainder
// calls `fmodf`, thus the test links libm.

FUNCTION main
VAR
    x : DINT := 9;
    y : DINT := 7331;
    a : DINT;
    b : DINT;
    c : DINT;
    r : REAL := 7.5;
    s : REAL := 2.0;
    q : REAL;
    t1 : TIME := T#7s;
    t2 : TIME := T#3s;
    t : TIME;
    lt1 : LTIME := LTIME#7s500ms;
    lt2 : LTIME := LTIME#2s;
    lt : LTIME;
END_VAR

a := MOD(y, x);
b := MOD(IN1 := y, IN2 := x);
c := MOD(IN2 := x, IN1 := y);
// CHECK: MOD=5 5 5 5
printf('MOD=%d %d %d %d$N', a, b, c, y MOD x);

q := MOD(r, s);
// CHECK: MOD-real=1.5 1.5
printf('MOD-real=%.1f %.1f$N', q, r MOD s);

t := MOD(t1, t2);
// CHECK: MOD-time=T#1s
printf('MOD-time=%s$N', REF(TIME_TO_STRING(t)));

lt := MOD(IN2 := lt2, IN1 := lt1);
// CHECK: MOD-ltime=LTIME#1s500ms
printf('MOD-ltime=%s$N', REF(LTIME_TO_STRING(lt)));
END_FUNCTION
