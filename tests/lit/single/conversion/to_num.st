// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// NOTE: These tests are kept simple on purpose, as the underlying
//       function (`A_TO_B`) has already been tested extensively; 
//       see `bit_num_conversion_tests.rs`
FUNCTION main 
    printf('%d$N', TO_SINT(SINT#10));       // CHECK: 10
    printf('%d$N', TO_SINT(USINT#10));      // CHECK: 10
    printf('%d$N', TO_SINT(INT#10));        // CHECK: 10
    printf('%d$N', TO_SINT(UINT#10));       // CHECK: 10
    printf('%d$N', TO_SINT(DINT#10));       // CHECK: 10
    printf('%d$N', TO_SINT(UDINT#10));      // CHECK: 10
    printf('%d$N', TO_SINT(LINT#10));       // CHECK: 10
    printf('%d$N', TO_SINT(ULINT#10));      // CHECK: 10
    printf('%d$N', TO_SINT(REAL#10));       // CHECK: 10
    printf('%d$N', TO_SINT(LREAL#10));      // CHECK: 10

    printf('%d$N', TO_USINT(SINT#10));      // CHECK: 10
    printf('%d$N', TO_USINT(USINT#10));     // CHECK: 10
    printf('%d$N', TO_USINT(INT#10));       // CHECK: 10
    printf('%d$N', TO_USINT(UINT#10));      // CHECK: 10
    printf('%d$N', TO_USINT(DINT#10));      // CHECK: 10
    printf('%d$N', TO_USINT(UDINT#10));     // CHECK: 10
    printf('%d$N', TO_USINT(LINT#10));      // CHECK: 10
    printf('%d$N', TO_USINT(ULINT#10));     // CHECK: 10
    printf('%d$N', TO_USINT(REAL#10));      // CHECK: 10
    printf('%d$N', TO_USINT(LREAL#10));     // CHECK: 10

    printf('%d$N', TO_INT(SINT#10));        // CHECK: 10
    printf('%d$N', TO_INT(USINT#10));       // CHECK: 10
    printf('%d$N', TO_INT(INT#10));         // CHECK: 10
    printf('%d$N', TO_INT(UINT#10));        // CHECK: 10
    printf('%d$N', TO_INT(DINT#10));        // CHECK: 10
    printf('%d$N', TO_INT(UDINT#10));       // CHECK: 10
    printf('%d$N', TO_INT(LINT#10));        // CHECK: 10
    printf('%d$N', TO_INT(ULINT#10));       // CHECK: 10
    printf('%d$N', TO_INT(REAL#10));        // CHECK: 10
    printf('%d$N', TO_INT(LREAL#10));       // CHECK: 10

    printf('%d$N', TO_UINT(SINT#10));       // CHECK: 10
    printf('%d$N', TO_UINT(USINT#10));      // CHECK: 10
    printf('%d$N', TO_UINT(INT#10));        // CHECK: 10
    printf('%d$N', TO_UINT(UINT#10));       // CHECK: 10
    printf('%d$N', TO_UINT(DINT#10));       // CHECK: 10
    printf('%d$N', TO_UINT(UDINT#10));      // CHECK: 10
    printf('%d$N', TO_UINT(LINT#10));       // CHECK: 10
    printf('%d$N', TO_UINT(ULINT#10));      // CHECK: 10
    printf('%d$N', TO_UINT(REAL#10));       // CHECK: 10
    printf('%d$N', TO_UINT(LREAL#10));      // CHECK: 10

    printf('%d$N', TO_DINT(SINT#10));       // CHECK: 10
    printf('%d$N', TO_DINT(USINT#10));      // CHECK: 10
    printf('%d$N', TO_DINT(INT#10));        // CHECK: 10
    printf('%d$N', TO_DINT(UINT#10));       // CHECK: 10
    printf('%d$N', TO_DINT(DINT#10));       // CHECK: 10
    printf('%d$N', TO_DINT(UDINT#10));      // CHECK: 10
    printf('%d$N', TO_DINT(LINT#10));       // CHECK: 10
    printf('%d$N', TO_DINT(ULINT#10));      // CHECK: 10
    printf('%d$N', TO_DINT(REAL#10));       // CHECK: 10
    printf('%d$N', TO_DINT(LREAL#10));      // CHECK: 10

    printf('%d$N', TO_UDINT(SINT#10));      // CHECK: 10
    printf('%d$N', TO_UDINT(USINT#10));     // CHECK: 10
    printf('%d$N', TO_UDINT(INT#10));       // CHECK: 10
    printf('%d$N', TO_UDINT(UINT#10));      // CHECK: 10
    printf('%d$N', TO_UDINT(DINT#10));      // CHECK: 10
    printf('%d$N', TO_UDINT(UDINT#10));     // CHECK: 10
    printf('%d$N', TO_UDINT(LINT#10));      // CHECK: 10
    printf('%d$N', TO_UDINT(ULINT#10));     // CHECK: 10
    printf('%d$N', TO_UDINT(REAL#10));      // CHECK: 10
    printf('%d$N', TO_UDINT(LREAL#10));     // CHECK: 10

    printf('%d$N', TO_LINT(SINT#10));       // CHECK: 10
    printf('%d$N', TO_LINT(USINT#10));      // CHECK: 10
    printf('%d$N', TO_LINT(INT#10));        // CHECK: 10
    printf('%d$N', TO_LINT(UINT#10));       // CHECK: 10
    printf('%d$N', TO_LINT(DINT#10));       // CHECK: 10
    printf('%d$N', TO_LINT(UDINT#10));      // CHECK: 10
    printf('%d$N', TO_LINT(LINT#10));       // CHECK: 10
    printf('%d$N', TO_LINT(ULINT#10));      // CHECK: 10
    printf('%d$N', TO_LINT(REAL#10));       // CHECK: 10
    printf('%d$N', TO_LINT(LREAL#10));      // CHECK: 10

    printf('%d$N', TO_ULINT(SINT#10));      // CHECK: 10
    printf('%d$N', TO_ULINT(USINT#10));     // CHECK: 10
    printf('%d$N', TO_ULINT(INT#10));       // CHECK: 10
    printf('%d$N', TO_ULINT(UINT#10));      // CHECK: 10
    printf('%d$N', TO_ULINT(DINT#10));      // CHECK: 10
    printf('%d$N', TO_ULINT(UDINT#10));     // CHECK: 10
    printf('%d$N', TO_ULINT(LINT#10));      // CHECK: 10
    printf('%d$N', TO_ULINT(ULINT#10));     // CHECK: 10
    printf('%d$N', TO_ULINT(REAL#10));      // CHECK: 10
    printf('%d$N', TO_ULINT(LREAL#10));     // CHECK: 10

    printf('%.1f$N', TO_REAL(SINT#10));     // CHECK: 10.0
    printf('%.1f$N', TO_REAL(USINT#10));    // CHECK: 10.0
    printf('%.1f$N', TO_REAL(INT#10));      // CHECK: 10.0
    printf('%.1f$N', TO_REAL(UINT#10));     // CHECK: 10.0
    printf('%.1f$N', TO_REAL(DINT#10));     // CHECK: 10.0
    printf('%.1f$N', TO_REAL(UDINT#10));    // CHECK: 10.0
    printf('%.1f$N', TO_REAL(LINT#10));     // CHECK: 10.0
    printf('%.1f$N', TO_REAL(ULINT#10));    // CHECK: 10.0
    printf('%.1f$N', TO_REAL(REAL#10));     // CHECK: 10.0
    printf('%.1f$N', TO_REAL(LREAL#10));    // CHECK: 10.0

    printf('%.1f$N', TO_LREAL(SINT#10));    // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(USINT#10));   // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(INT#10));     // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(UINT#10));    // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(DINT#10));    // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(UDINT#10));   // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(LINT#10));    // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(ULINT#10));   // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(REAL#10));    // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(LREAL#10));   // CHECK: 10.0
END_FUNCTION