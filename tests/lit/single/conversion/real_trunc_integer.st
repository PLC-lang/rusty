// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    printf('%d$N', REAL_TRUNC_SINT(4.9));  // CHECK: 4
    printf('%d$N', REAL_TRUNC_USINT(4.9));  // CHECK: 4
    printf('%d$N', REAL_TRUNC_INT(4.9));  // CHECK: 4
    printf('%d$N', REAL_TRUNC_UINT(4.9));  // CHECK: 4
    printf('%d$N', REAL_TRUNC_DINT(4.9));  // CHECK: 4
    printf('%d$N', REAL_TRUNC_UDINT(4.9));  // CHECK: 4
    printf('%d$N', REAL_TRUNC_LINT(4.9));  // CHECK: 4
    printf('%d$N', REAL_TRUNC_ULINT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_SINT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_USINT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_INT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_UINT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_DINT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_UDINT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_LINT(4.9));  // CHECK: 4
    printf('%d$N', LREAL_TRUNC_ULINT(4.9));  // CHECK: 4
END_FUNCTION