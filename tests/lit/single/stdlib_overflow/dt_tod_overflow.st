// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that DT and TOD arithmetic wraps mod 2^64 on overflow and yields plain
// signed values below zero

FUNCTION main : DINT
VAR
    a : DT;
    b : DT;
    c : TIME;
END_VAR
    a := ADD_DT_TIME(DT#2262-04-11-23:47:16, TIME#1d);
    // CHECK: -9223285637709551616
    printf('%lld$N', a);

    b := SUB_DT_TIME(DT#1970-01-01-01:00:00, TIME#2h);
    // CHECK: -3600000000000
    printf('%lld$N', b);

    c := SUB_TOD_TOD(TOD#01:00:00, TOD#02:00:00);
    // CHECK: -3600000000000
    printf('%lld$N', c);

    main := 0;
END_FUNCTION
