// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    // TO_TIME
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(LTIME#10s))));                        // CHECK: 10s

    // TO_LTIME
    printf('%s$N', REF(TIME_TO_STRING(TO_LTIME(TIME#10s))));                        // CHECK: 10s

    // TO_DT
    printf('%s$N', REF(TIME_TO_STRING(TO_DT(LDT#2024-11-12-16:36:15.100))));        // CHECK: 20039d16h36m15s100ms

    // TO_LDT
    printf('%s$N', REF(TIME_TO_STRING(TO_LDT(DT#2024-11-12-16:36:15.100))));        // CHECK: 20039d16h36m15s100ms

    // TO_DATE
    printf('%s$N', REF(TIME_TO_STRING(TO_DATE(DT#2024-11-12-16:36:15.100))));       // CHECK: 20039d
    printf('%s$N', REF(TIME_TO_STRING(TO_DATE(LDT#2024-11-12-16:36:15.100))));      // CHECK: 20039d

    // TO_TOD
    printf('%s$N', REF(LTOD_TO_STRING(TO_TOD(LTOD#16:36:15))));                     // CHECK: 16:36:15
    printf('%s$N', REF(TOD_TO_STRING(TO_TOD(DT#2024-11-12-16:36:15.100))));         // CHECK: 16:36:15.100
    printf('%s$N', REF(TOD_TO_STRING(TO_TOD(LDT#2024-11-12-16:36:15.100))));        // CHECK: 16:36:15.100

    // TO_LTOD
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(TOD#16:36:15))));                     // CHECK: 16:36:15
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(DT#2024-11-12-16:36:15.100))));       // CHECK: 16:36:15.100
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(LDT#2024-11-12-16:36:15.100))));      // CHECK: 16:36:15.100
END_FUNCTION