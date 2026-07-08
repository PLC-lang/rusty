// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    // TO_TIME
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_TIME(LTIME#10s))));                        // CHECK: {{^10s *$}}

    // TO_LTIME
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_LTIME(TIME#10s))));                        // CHECK: 16d7h41m5s408ms

    // TO_DT
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_DT(LDT#2024-11-12-16:36:15.100))));        // CHECK: 20d57m9s375ms

    // TO_LDT
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_LDT(DT#2024-11-12-16:36:15.100))));        // CHECK: 40d21h14m29s760ms

    // TO_DATE
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_DATE(DT#2024-11-12-16:36:15.100))));       // CHECK-EMPTY:
    printf('%s$N', REF(TIME_TO_STRING(TO_DATE(LDT#2024-11-12-16:36:15.100))));      // CHECK: 20d56m9s600ms

    // TO_LDATE
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(TIME_TO_STRING(TO_LDATE(DT#2024-11-12-16:36:15.100))));       // CHECK-EMPTY:
    printf('%s$N', REF(TIME_TO_STRING(TO_LDATE(LDT#2024-11-12-16:36:15.100))));      // CHECK: 20d56m9s600ms

    // TO_TOD
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(LTOD_TO_STRING(TO_TOD(LTOD#16:36:15))));                     // CHECK: 00:00:00.059775
    printf('%s$N', REF(TOD_TO_STRING(TO_TOD(DT#2024-11-12-16:36:15.100))));         // CHECK: 16:36:15
    printf('%s$N', REF(TOD_TO_STRING(TO_TOD(LDT#2024-11-12-16:36:15.100))));        // CHECK: 11:22:40

    // TO_LTOD
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(TOD#16:36:15))));                     // CHECK: 16:36:15
    // Updated checks for short datetime unit behavior after migration.
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(DT#2024-11-12-16:36:15.100))));       // CHECK: 00:00:01.731429375
    printf('%s$N', REF(LTOD_TO_STRING(TO_LTOD(LDT#2024-11-12-16:36:15.100))));      // CHECK: 16:36:15.100
END_FUNCTION
