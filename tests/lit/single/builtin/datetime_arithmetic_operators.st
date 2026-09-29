// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// The operators on date and time operands: a time of day wraps at midnight, a real factor is
// applied as such, and every other combination is the plain nanosecond arithmetic

FUNCTION main
    VAR
        t1, t2, t : TIME;
        tod : TOD;
        d1, d2 : DATE;
        dt1, dt2 : DT;
        ltod : LTOD;
        lt : LTIME;
    END_VAR
    t1 := T#1s; t2 := T#500ms; t := t1 + t2;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#1s500ms
    tod := TOD#23:59:50 + T#20s;
    printf('%s %lld$N', REF(TOD_TO_STRING(tod)), tod);                              // CHECK: TOD#00:00:10 10000000000
    tod := TOD#00:00:10 - T#20s;
    printf('%s %lld$N', REF(TOD_TO_STRING(tod)), tod);                              // CHECK: TOD#23:59:50 86390000000000
    dt1 := DT#2024-01-15-12:30:00 + T#5s;
    printf('%s$N', REF(DT_TO_STRING(dt1)));                                         // CHECK: DT#2024-01-15-12:30:05
    t1 := T#10s; t2 := T#3s; t := t1 - t2;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#7s
    d1 := D#2024-01-15; d2 := D#2024-01-10; t := d1 - d2;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#5d
    t := TOD#10:30:00 - TOD#10:20:00;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#10m
    dt1 := DT#2024-01-15-12:30:10; dt2 := DT#2024-01-15-12:30:00; t := dt1 - dt2;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#10s
    t := t1 * 3;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#30s
    t := 2 * t1;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#20s
    t := t1 / 4;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#2s500ms
    t := t1 * 1.5;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#15s
    t := 1.5 * t1;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#15s
    t := t1 / 2.5;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#4s
    ltod := LTOD#23:59:50 + LTIME#20s;
    printf('%s %lld$N', REF(LTOD_TO_STRING(ltod)), ltod);                           // CHECK: LTOD#00:00:10 10000000000
    lt := LTIME#10s * 1.5;
    printf('%s$N', REF(LTIME_TO_STRING(lt)));                                       // CHECK: LTIME#15s
    t := t1 + 5;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#10s5ns
    dt1 := DT#2027-05-05-22:30:15 + T#20s + T#20s + T#2h555s;
    printf('%s$N', REF(DT_TO_STRING(dt1)));                                         // CHECK: DT#2027-05-06-00:40:10
END_FUNCTION
