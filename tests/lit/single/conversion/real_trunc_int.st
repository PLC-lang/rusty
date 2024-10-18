// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    printf('%d$N', REAL_TRUNC_SINT(5.6));       // CHECK: 5
    printf('%d$N', REAL_TRUNC_USINT(5.6));      // CHECK: 5
    printf('%d$N', REAL_TRUNC_INT(5.6));        // CHECK: 5
    printf('%d$N', REAL_TRUNC_UINT(5.6));       // CHECK: 5
    printf('%d$N', REAL_TRUNC_INT(5.6));        // CHECK: 5
    printf('%d$N', REAL_TRUNC_UINT(5.6));       // CHECK: 5
    printf('%d$N', REAL_TRUNC_DINT(5.6));       // CHECK: 5
    printf('%d$N', REAL_TRUNC_UDINT(5.6));      // CHECK: 5
    printf('%d$N', REAL_TRUNC_LINT(5.6));       // CHECK: 5
    printf('%d$N', REAL_TRUNC_ULINT(5.6));      // CHECK: 5

    printf('%d$N', REAL_TRUNC_SINT(-5.6));      // CHECK: -5
    printf('%d$N', REAL_TRUNC_INT(-5.6));       // CHECK: -5
    printf('%d$N', REAL_TRUNC_DINT(-5.6));      // CHECK: -5
    printf('%d$N', REAL_TRUNC_LINT(-5.6));      // CHECK: -5
END_FUNCTION