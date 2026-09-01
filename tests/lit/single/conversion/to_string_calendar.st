// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    printf('%s$N', REF(DATE_TO_STRING(D#2024-3-4)));          // CHECK: D#2024-03-04
    printf('%s$N', REF(DATE_TO_STRING(DATE#2106-02-07)));     // CHECK-NEXT: D#2106-02-07
    printf('%s$N', REF(DT_TO_STRING(DT#2024-3-4-5:6:7)));     // CHECK-NEXT: DT#2024-03-04-05:06:07
    printf('%s$N', REF(DT_TO_STRING(DT#2106-02-07-06:28:15))); // CHECK-NEXT: DT#2106-02-07-06:28:15
    printf('%s$N', REF(TOD_TO_STRING(TOD#1:2:3)));            // CHECK-NEXT: TOD#01:02:03
    printf('%s$N', REF(TOD_TO_STRING(TOD#23:59:59.999)));     // CHECK-NEXT: TOD#23:59:59.999
END_FUNCTION
