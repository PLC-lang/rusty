// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that DT and TOD arithmetic wraps mod 2^32 on overflow and underflow

FUNCTION main : DINT
VAR
    a : DT;
    b : DT;
    c : TIME;
END_VAR
    a := ADD_DT_TIME(DT#2106-02-07-06:28:00, TIME#1m);
    // CHECK: 44
    printf('%u$N', a);

    b := SUB_DT_TIME(DT#1970-01-01-01:00:00, TIME#2h);
    // CHECK: 4294963696
    printf('%u$N', b);

    c := SUB_TOD_TOD(TOD#01:00:00, TOD#02:00:00);
    // CHECK: 4291367296
    printf('%u$N', c);

    main := 0;
END_FUNCTION
