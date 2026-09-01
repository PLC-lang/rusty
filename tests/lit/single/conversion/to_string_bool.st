// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    printf('%s$N', REF(BOOL_TO_STRING(TRUE)));  // CHECK: TRUE
    printf('%s$N', REF(BOOL_TO_STRING(FALSE))); // CHECK-NEXT: FALSE
    printf('%s$N', REF(BOOL_TO_STRING(0)));     // CHECK-NEXT: FALSE
    printf('%s$N', REF(BOOL_TO_STRING(1)));     // CHECK-NEXT: TRUE
    printf('%s$N', REF(BOOL_TO_STRING(2)));     // CHECK-NEXT: FALSE
END_FUNCTION
