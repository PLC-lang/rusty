// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION main
VAR
    b  : BOOL  := TRUE;
    dw : DWORD := 4294967295;
    i  : INT   := -1;
    di : DINT  := -2147483648;
    ud : UDINT := 1000;
    r  : REAL  := 2.5;
    lr : LREAL := -1.0;
END_VAR
    // TO_TIME
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(LTIME#10s))));                        // CHECK: {{^T#10s *$}}
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(b))));                                // CHECK: T#1ms
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(dw))));                               // CHECK: T#49d17h2m47s295ms
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(i))));                                // CHECK: T#49d17h2m47s295ms
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(di))));                               // CHECK: T#24d20h31m23s648ms
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(ud))));                               // CHECK: T#1s
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(r))));                                // CHECK: T#3ms
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(lr))));                               // CHECK: T#49d17h2m47s295ms

    // TO_LTIME
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_LTIME(TIME#10s))));                        // CHECK: T#16d7h41m5s408ms
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(LTIME#10s500ns))));                 // CHECK: LTIME#10s500ns
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(b))));                              // CHECK: LTIME#1ns
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(dw))));                             // CHECK: LTIME#4s294ms967us295ns
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(i))));                              // CHECK: LTIME#213503d23h34m33s709ms551us615ns
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(di))));                             // CHECK: LTIME#213503d23h34m31s562ms67us968ns
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(ud))));                             // CHECK: LTIME#1us
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(r))));                              // CHECK: LTIME#3ns
    printf('%s$N', REF(LTIME_TO_STRING(TO_LTIME(lr))));                             // CHECK: LTIME#213503d23h34m33s709ms551us615ns

    // TO_DT
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_DT(LDT#2024-11-12-16:36:15.100))));        // CHECK: T#20d57m9s375ms

    // TO_LDT
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_LDT(DT#2024-11-12-16:36:15.100))));        // CHECK: T#40d21h14m29s760ms
    printf('%s$N', REF(LDT_TO_STRING(TO_LDT(LDT#2024-11-12-16:36:15.100))));        // CHECK: LDT#2024-11-12-16:36:15.100

    // TO_DATE
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_DATE(DT#2024-11-12-16:36:15.100))));       // CHECK: T#20d56m9s600ms
    printf('%s$N', REF(TIME_TO_STRING(TO_DATE(LDT#2024-11-12-16:36:15.100))));      // CHECK: T#20d56m9s600ms

    // TO_LDATE
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_LDATE(DT#2024-11-12-16:36:15.100))));       // CHECK: T#18d10h18m48s192ms
    printf('%s$N', REF(TIME_TO_STRING(TO_LDATE(LDT#2024-11-12-16:36:15.100))));      // CHECK: T#18d10h18m48s192ms

    // TO_TOD
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TOD_TO_STRING(TO_TOD(LTOD#16:36:15))));                      // CHECK: TOD#16:36:15
    printf('%s$N', REF(TOD_TO_STRING(TO_TOD(DT#2024-11-12-16:36:15.100))));         // CHECK: TOD#16:36:15
    printf('%s$N', REF(TOD_TO_STRING(TO_TOD(LDT#2024-11-12-16:36:15.100))));        // CHECK: TOD#16:36:15.100

    // TO_LTOD
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(TOD#16:36:15))));                     // CHECK: LTOD#16:36:15
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(LTOD#16:36:15.100))));                // CHECK: LTOD#16:36:15.100
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(DT#2024-11-12-16:36:15.100))));       // CHECK: LTOD#16:36:15
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(LDT#2024-11-12-16:36:15.100))));      // CHECK: LTOD#16:36:15.100
END_FUNCTION
