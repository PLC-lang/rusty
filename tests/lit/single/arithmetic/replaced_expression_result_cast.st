// RUN: %COMPILE %s && %RUN | %CHECK %s
// The result of a builtin or an operator the compiler carries out through a function call is
// converted to the type of the variable it is assigned to

FUNCTION main
    VAR
        t : TIME := T#10s;
        lt : LTIME := LTIME#10s;
        tod : TOD := TOD#23:59:50;
        n : DINT := 7;
        m : DINT := 3;
        i : INT;
        r : REAL := 2.5;
        x : LREAL;
    END_VAR

    x := t * 1.5;
    printf('%.1f$N', x);                                                            // CHECK: 15000.0
    r := t / 4.0;
    x := r;
    printf('%.1f$N', x);                                                            // CHECK: 2500.0
    x := lt + lt;
    printf('%.1f$N', x);                                                            // CHECK: 20000000000.0

    x := MUL(t, 1.5);
    printf('%.1f$N', x);                                                            // CHECK: 15000.0
    x := ADD(n, m);
    printf('%.1f$N', x);                                                            // CHECK: 10.0
    r := 2.5;
    n := ADD(n, r);
    printf('%d$N', n);                                                              // CHECK: 9
    i := SUB(t, T#4s);
    printf('%d$N', i);                                                              // CHECK: 6000

    tod := (tod + T#20s);
    printf('%s$N', REF(TOD_TO_STRING(tod)));                                        // CHECK: TOD#00:00:10

END_FUNCTION
