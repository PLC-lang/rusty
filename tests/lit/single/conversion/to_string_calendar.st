// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    printf('%s$N', REF(DATE_TO_STRING(D#2024-3-4)));          // CHECK: D#2024-03-04
    printf('%s$N', REF(DATE_TO_STRING(DATE#2106-02-07)));     // CHECK-NEXT: D#2106-02-07
    printf('%s$N', REF(DT_TO_STRING(DT#2024-3-4-5:6:7)));     // CHECK-NEXT: DT#2024-03-04-05:06:07
    printf('%s$N', REF(DT_TO_STRING(DT#2106-02-07-06:28:15))); // CHECK-NEXT: DT#2106-02-07-06:28:15
    printf('%s$N', REF(TOD_TO_STRING(TOD#1:2:3)));            // CHECK-NEXT: TOD#01:02:03
    printf('%s$N', REF(TOD_TO_STRING(TOD#23:59:59.999)));     // CHECK-NEXT: TOD#23:59:59.999
    printf('%s$N', REF(DATE_TO_STRING(-1)));                  // CHECK-NEXT: D#2106-02-07
    printf('%s$N', REF(DT_TO_STRING(-1)));                    // CHECK-NEXT: DT#2106-02-07-06:28:15
    printf('%s$N', REF(TOD_TO_STRING(-1)));                   // CHECK-NEXT: TOD#17:02:47.295
    printf('%s$N', REF(DATE_TO_STRING(D#2106-02-08)));        // CHECK-NEXT: D#1970-01-01
    printf('%s$N', REF(DATE_TO_STRING(4294967296)));          // CHECK-NEXT: D#1970-01-01
    printf('%s$N', REF(DT_TO_STRING(DT#2106-02-08-00:00:00))); // CHECK-NEXT: DT#1970-01-01-17:31:44
    printf('%s$N', REF(DT_TO_STRING(4294967296)));            // CHECK-NEXT: DT#1970-01-01-00:00:00
    printf('%s$N', REF(LTOD_TO_STRING(86400000000000)));      // CHECK-NEXT: LTOD#00:00:00
END_FUNCTION
