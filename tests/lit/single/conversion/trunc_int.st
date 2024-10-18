// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    // REAL
    printf('%d$N', TRUNC_SINT(REAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_USINT(REAL#5.6));      // CHECK: 5
    printf('%d$N', TRUNC_INT(REAL#5.6));        // CHECK: 5
    printf('%d$N', TRUNC_UINT(REAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_INT(REAL#5.6));        // CHECK: 5
    printf('%d$N', TRUNC_UINT(REAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_DINT(REAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_UDINT(REAL#5.6));      // CHECK: 5
    printf('%d$N', TRUNC_LINT(REAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_ULINT(REAL#5.6));      // CHECK: 5

    printf('%d$N', TRUNC_SINT(REAL#-5.6));      // CHECK: -5
    printf('%d$N', TRUNC_INT(REAL#-5.6));       // CHECK: -5
    printf('%d$N', TRUNC_DINT(REAL#-5.6));      // CHECK: -5
    printf('%d$N', TRUNC_LINT(REAL#-5.6));      // CHECK: -5

    // LREAL
    printf('%d$N', TRUNC_SINT(LREAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_USINT(LREAL#5.6));      // CHECK: 5
    printf('%d$N', TRUNC_INT(LREAL#5.6));        // CHECK: 5
    printf('%d$N', TRUNC_UINT(LREAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_INT(LREAL#5.6));        // CHECK: 5
    printf('%d$N', TRUNC_UINT(LREAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_DINT(LREAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_UDINT(LREAL#5.6));      // CHECK: 5
    printf('%d$N', TRUNC_LINT(LREAL#5.6));       // CHECK: 5
    printf('%d$N', TRUNC_ULINT(LREAL#5.6));      // CHECK: 5

    printf('%d$N', TRUNC_SINT(LREAL#-5.6));      // CHECK: -5
    printf('%d$N', TRUNC_INT(LREAL#-5.6));       // CHECK: -5
    printf('%d$N', TRUNC_DINT(LREAL#-5.6));      // CHECK: -5
    printf('%d$N', TRUNC_LINT(LREAL#-5.6));      // CHECK: -5
END_FUNCTION