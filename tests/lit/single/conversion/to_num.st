// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    printf('%d$N', TO_SINT(SINT#10));           // CHECK: 10
    printf('%d$N', TO_SINT(USINT#10));          // CHECK: 10
    printf('%d$N', TO_SINT(INT#10));            // CHECK: 10
    printf('%d$N', TO_SINT(UINT#10));           // CHECK: 10
    printf('%d$N', TO_SINT(DINT#10));           // CHECK: 10
    printf('%d$N', TO_SINT(UDINT#10));          // CHECK: 10
    printf('%d$N', TO_SINT(LINT#10));           // CHECK: 10
    printf('%d$N', TO_SINT(ULINT#10));          // CHECK: 10
    printf('%d$N', TO_SINT(REAL#10.0));         // CHECK: 10
    printf('%d$N', TO_SINT(LREAL#10.0));        // CHECK: 10
    printf('%d$N', TO_SINT(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_SINT(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_SINT(BYTE#10));           // CHECK: 10
    printf('%d$N', TO_SINT(WORD#10));           // CHECK: 10
    printf('%d$N', TO_SINT(DWORD#10));          // CHECK: 10
    printf('%d$N', TO_SINT(LWORD#10));          // CHECK: 10

    printf('%d$N', TO_USINT(SINT#10));          // CHECK: 10
    printf('%d$N', TO_USINT(USINT#10));         // CHECK: 10
    printf('%d$N', TO_USINT(INT#10));           // CHECK: 10
    printf('%d$N', TO_USINT(UINT#10));          // CHECK: 10
    printf('%d$N', TO_USINT(DINT#10));          // CHECK: 10
    printf('%d$N', TO_USINT(UDINT#10));         // CHECK: 10
    printf('%d$N', TO_USINT(LINT#10));          // CHECK: 10
    printf('%d$N', TO_USINT(ULINT#10));         // CHECK: 10
    printf('%d$N', TO_USINT(REAL#10.0));        // CHECK: 10
    printf('%d$N', TO_USINT(LREAL#10.0));       // CHECK: 10
    printf('%d$N', TO_USINT(BOOL#FALSE));       // CHECK: 0
    printf('%d$N', TO_USINT(BOOL#TRUE));        // CHECK: 1
    printf('%d$N', TO_USINT(BYTE#10));          // CHECK: 10
    printf('%d$N', TO_USINT(WORD#10));          // CHECK: 10
    printf('%d$N', TO_USINT(DWORD#10));         // CHECK: 10
    printf('%d$N', TO_USINT(LWORD#10));         // CHECK: 10

    printf('%d$N', TO_INT(SINT#10));            // CHECK: 10
    printf('%d$N', TO_INT(USINT#10));           // CHECK: 10
    printf('%d$N', TO_INT(INT#10));             // CHECK: 10
    printf('%d$N', TO_INT(UINT#10));            // CHECK: 10
    printf('%d$N', TO_INT(DINT#10));            // CHECK: 10
    printf('%d$N', TO_INT(UDINT#10));           // CHECK: 10
    printf('%d$N', TO_INT(LINT#10));            // CHECK: 10
    printf('%d$N', TO_INT(ULINT#10));           // CHECK: 10
    printf('%d$N', TO_INT(REAL#10.0));          // CHECK: 10
    printf('%d$N', TO_INT(LREAL#10.0));         // CHECK: 10
    printf('%d$N', TO_INT(BOOL#FALSE));         // CHECK: 0
    printf('%d$N', TO_INT(BOOL#TRUE));          // CHECK: 1
    printf('%d$N', TO_INT(BYTE#10));            // CHECK: 10
    printf('%d$N', TO_INT(WORD#10));            // CHECK: 10
    printf('%d$N', TO_INT(DWORD#10));           // CHECK: 10
    printf('%d$N', TO_INT(LWORD#10));           // CHECK: 10

    printf('%d$N', TO_UINT(SINT#10));           // CHECK: 10
    printf('%d$N', TO_UINT(USINT#10));          // CHECK: 10
    printf('%d$N', TO_UINT(INT#10));            // CHECK: 10
    printf('%d$N', TO_UINT(UINT#10));           // CHECK: 10
    printf('%d$N', TO_UINT(DINT#10));           // CHECK: 10
    printf('%d$N', TO_UINT(UDINT#10));          // CHECK: 10
    printf('%d$N', TO_UINT(LINT#10));           // CHECK: 10
    printf('%d$N', TO_UINT(ULINT#10));          // CHECK: 10
    printf('%d$N', TO_UINT(REAL#10.0));         // CHECK: 10
    printf('%d$N', TO_UINT(LREAL#10.0));        // CHECK: 10
    printf('%d$N', TO_UINT(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_UINT(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_UINT(BYTE#10));           // CHECK: 10
    printf('%d$N', TO_UINT(WORD#10));           // CHECK: 10
    printf('%d$N', TO_UINT(DWORD#10));          // CHECK: 10
    printf('%d$N', TO_UINT(LWORD#10));          // CHECK: 10

    printf('%d$N', TO_DINT(SINT#10));           // CHECK: 10
    printf('%d$N', TO_DINT(USINT#10));          // CHECK: 10
    printf('%d$N', TO_DINT(INT#10));            // CHECK: 10
    printf('%d$N', TO_DINT(UINT#10));           // CHECK: 10
    printf('%d$N', TO_DINT(DINT#10));           // CHECK: 10
    printf('%d$N', TO_DINT(UDINT#10));          // CHECK: 10
    printf('%d$N', TO_DINT(LINT#10));           // CHECK: 10
    printf('%d$N', TO_DINT(ULINT#10));          // CHECK: 10
    printf('%d$N', TO_DINT(REAL#10.0));         // CHECK: 10
    printf('%d$N', TO_DINT(LREAL#10.0));        // CHECK: 10
    printf('%d$N', TO_DINT(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_DINT(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_DINT(BYTE#10));           // CHECK: 10
    printf('%d$N', TO_DINT(WORD#10));           // CHECK: 10
    printf('%d$N', TO_DINT(DWORD#10));          // CHECK: 10
    printf('%d$N', TO_DINT(LWORD#10));          // CHECK: 10

    printf('%d$N', TO_LINT(SINT#10));           // CHECK: 10
    printf('%d$N', TO_LINT(USINT#10));          // CHECK: 10
    printf('%d$N', TO_LINT(INT#10));            // CHECK: 10
    printf('%d$N', TO_LINT(UINT#10));           // CHECK: 10
    printf('%d$N', TO_LINT(DINT#10));           // CHECK: 10
    printf('%d$N', TO_LINT(UDINT#10));          // CHECK: 10
    printf('%d$N', TO_LINT(LINT#10));           // CHECK: 10
    printf('%d$N', TO_LINT(ULINT#10));          // CHECK: 10
    printf('%d$N', TO_LINT(REAL#10.0));         // CHECK: 10
    printf('%d$N', TO_LINT(LREAL#10.0));        // CHECK: 10
    printf('%d$N', TO_LINT(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_LINT(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_LINT(BYTE#10));           // CHECK: 10
    printf('%d$N', TO_LINT(WORD#10));           // CHECK: 10
    printf('%d$N', TO_LINT(DWORD#10));          // CHECK: 10
    printf('%d$N', TO_LINT(LWORD#10));          // CHECK: 10

    printf('%d$N', TO_ULINT(SINT#10));          // CHECK: 10
    printf('%d$N', TO_ULINT(USINT#10));         // CHECK: 10
    printf('%d$N', TO_ULINT(INT#10));           // CHECK: 10
    printf('%d$N', TO_ULINT(UINT#10));          // CHECK: 10
    printf('%d$N', TO_ULINT(DINT#10));          // CHECK: 10
    printf('%d$N', TO_ULINT(UDINT#10));         // CHECK: 10
    printf('%d$N', TO_ULINT(LINT#10));          // CHECK: 10
    printf('%d$N', TO_ULINT(ULINT#10));         // CHECK: 10
    printf('%d$N', TO_ULINT(REAL#10.0));        // CHECK: 10
    printf('%d$N', TO_ULINT(LREAL#10.0));       // CHECK: 10
    printf('%d$N', TO_ULINT(BOOL#FALSE));       // CHECK: 0
    printf('%d$N', TO_ULINT(BOOL#TRUE));        // CHECK: 1
    printf('%d$N', TO_ULINT(BYTE#10));          // CHECK: 10
    printf('%d$N', TO_ULINT(WORD#10));          // CHECK: 10
    printf('%d$N', TO_ULINT(DWORD#10));         // CHECK: 10
    printf('%d$N', TO_ULINT(LWORD#10));         // CHECK: 10

    printf('%.1f$N', TO_REAL(SINT#10));         // CHECK: 10.0
    printf('%.1f$N', TO_REAL(USINT#10));        // CHECK: 10.0
    printf('%.1f$N', TO_REAL(INT#10));          // CHECK: 10.0
    printf('%.1f$N', TO_REAL(UINT#10));         // CHECK: 10.0
    printf('%.1f$N', TO_REAL(DINT#10));         // CHECK: 10.0
    printf('%.1f$N', TO_REAL(UDINT#10));        // CHECK: 10.0
    printf('%.1f$N', TO_REAL(LINT#10));         // CHECK: 10.0
    printf('%.1f$N', TO_REAL(ULINT#10));        // CHECK: 10.0
    printf('%.1f$N', TO_REAL(REAL#10.0));       // CHECK: 10.0
    printf('%.1f$N', TO_REAL(LREAL#10.0));      // CHECK: 10.0
    printf('%.1f$N', TO_REAL(BOOL#FALSE));      // CHECK: 0.0
    printf('%.1f$N', TO_REAL(BOOL#TRUE));       // CHECK: 1.0
    printf('%.1f$N', TO_REAL(BYTE#10));         // CHECK: 10.0
    printf('%.1f$N', TO_REAL(WORD#10));         // CHECK: 10.0
    printf('%.1f$N', TO_REAL(DWORD#10));        // CHECK: 10.0
    printf('%.1f$N', TO_REAL(LWORD#10));        // CHECK: 10.0

    printf('%.1f$N', TO_LREAL(SINT#10));        // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(USINT#10));       // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(INT#10));         // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(UINT#10));        // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(DINT#10));        // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(UDINT#10));       // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(LINT#10));        // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(ULINT#10));       // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(REAL#10.0));      // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(LREAL#10.0));     // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(BOOL#FALSE));     // CHECK: 0.0
    printf('%.1f$N', TO_LREAL(BOOL#TRUE));      // CHECK: 1.0
    printf('%.1f$N', TO_LREAL(BYTE#10));        // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(WORD#10));        // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(DWORD#10));       // CHECK: 10.0
    printf('%.1f$N', TO_LREAL(LWORD#10));       // CHECK: 10.0

    printf('%d$N', TO_BOOL(SINT#0));            // CHECK: 0
    printf('%d$N', TO_BOOL(USINT#0));           // CHECK: 0
    printf('%d$n', TO_BOOL(INT#0));             // CHECK: 0
    printf('%d$n', TO_BOOL(UINT#0));            // CHECK: 0
    printf('%d$n', TO_BOOL(DINT#0));            // CHECK: 0
    printf('%d$n', TO_BOOL(UDINT#0));           // CHECK: 0
    printf('%d$n', TO_BOOL(LINT#0));            // CHECK: 0
    printf('%d$n', TO_BOOL(ULINT#0));           // CHECK: 0
    printf('%d$n', TO_BOOL(REAL#0.0));          // CHECK: 0
    printf('%d$n', TO_BOOL(LREAL#0.0));         // CHECK: 0
    printf('%d$n', TO_BOOL(BOOL#FALSE));        // CHECK: 0
    printf('%d$n', TO_BOOL(BYTE#0));            // CHECK: 0
    printf('%d$n', TO_BOOL(WORD#0));            // CHECK: 0
    printf('%d$n', TO_BOOL(DWORD#0));           // CHECK: 0
    printf('%d$n', TO_BOOL(LWORD#0));           // CHECK: 0

    printf('%d$N', TO_BOOL(SINT#1));            // CHECK: 1
    printf('%d$N', TO_BOOL(USINT#1));           // CHECK: 1
    printf('%d$n', TO_BOOL(INT#1));             // CHECK: 1
    printf('%d$n', TO_BOOL(UINT#1));            // CHECK: 1
    printf('%d$n', TO_BOOL(DINT#1));            // CHECK: 1
    printf('%d$n', TO_BOOL(UDINT#1));           // CHECK: 1
    printf('%d$n', TO_BOOL(LINT#1));            // CHECK: 1
    printf('%d$n', TO_BOOL(ULINT#1));           // CHECK: 1
    printf('%d$n', TO_BOOL(REAL#1.0));          // CHECK: 1
    printf('%d$n', TO_BOOL(LREAL#1.0));         // CHECK: 1
    printf('%d$n', TO_BOOL(BOOL#TRUE));         // CHECK: 1
    printf('%d$n', TO_BOOL(BYTE#1));            // CHECK: 1
    printf('%d$n', TO_BOOL(WORD#1));            // CHECK: 1
    printf('%d$n', TO_BOOL(DWORD#1));           // CHECK: 1
    printf('%d$n', TO_BOOL(LWORD#1));           // CHECK: 1

    printf('%d$N', TO_BOOL(SINT#5));            // CHECK: 1
    printf('%d$N', TO_BOOL(USINT#5));           // CHECK: 1
    printf('%d$n', TO_BOOL(INT#5));             // CHECK: 1
    printf('%d$n', TO_BOOL(UINT#5));            // CHECK: 1
    printf('%d$n', TO_BOOL(DINT#5));            // CHECK: 1
    printf('%d$n', TO_BOOL(UDINT#5));           // CHECK: 1
    printf('%d$n', TO_BOOL(LINT#5));            // CHECK: 1
    printf('%d$n', TO_BOOL(ULINT#5));           // CHECK: 1
    printf('%d$n', TO_BOOL(REAL#5.0));          // CHECK: 1
    printf('%d$n', TO_BOOL(LREAL#5.0));         // CHECK: 1
    printf('%d$n', TO_BOOL(BOOL#TRUE));         // CHECK: 1
    printf('%d$n', TO_BOOL(BYTE#5));            // CHECK: 1
    printf('%d$n', TO_BOOL(WORD#5));            // CHECK: 1
    printf('%d$n', TO_BOOL(DWORD#5));           // CHECK: 1
    printf('%d$n', TO_BOOL(LWORD#5));           // CHECK: 1

    printf('%d$N', TO_BYTE(SINT#10));           // CHECK: 10
    printf('%d$N', TO_BYTE(USINT#10));          // CHECK: 10
    printf('%d$N', TO_BYTE(INT#10));            // CHECK: 10
    printf('%d$N', TO_BYTE(UINT#10));           // CHECK: 10
    printf('%d$N', TO_BYTE(DINT#10));           // CHECK: 10
    printf('%d$N', TO_BYTE(UDINT#10));          // CHECK: 10
    printf('%d$N', TO_BYTE(LINT#10));           // CHECK: 10
    printf('%d$N', TO_BYTE(ULINT#10));          // CHECK: 10
    printf('%d$N', TO_BYTE(REAL#10.0));         // CHECK: 10
    printf('%d$N', TO_BYTE(LREAL#10.0));        // CHECK: 10
    printf('%d$N', TO_BYTE(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_BYTE(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_BYTE(BYTE#10));           // CHECK: 10
    printf('%d$N', TO_BYTE(WORD#10));           // CHECK: 10
    printf('%d$N', TO_BYTE(DWORD#10));          // CHECK: 10
    printf('%d$N', TO_BYTE(LWORD#10));          // CHECK: 10

    printf('%d$N', TO_WORD(SINT#10));           // CHECK: 10
    printf('%d$N', TO_WORD(USINT#10));          // CHECK: 10
    printf('%d$N', TO_WORD(INT#10));            // CHECK: 10
    printf('%d$N', TO_WORD(UINT#10));           // CHECK: 10
    printf('%d$N', TO_WORD(DINT#10));           // CHECK: 10
    printf('%d$N', TO_WORD(UDINT#10));          // CHECK: 10
    printf('%d$N', TO_WORD(LINT#10));           // CHECK: 10
    printf('%d$N', TO_WORD(ULINT#10));          // CHECK: 10
    printf('%d$N', TO_WORD(REAL#10.0));         // CHECK: 10
    printf('%d$N', TO_WORD(LREAL#10.0));        // CHECK: 10
    printf('%d$N', TO_WORD(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_WORD(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_WORD(BYTE#10));           // CHECK: 10
    printf('%d$N', TO_WORD(WORD#10));           // CHECK: 10
    printf('%d$N', TO_WORD(DWORD#10));          // CHECK: 10
    printf('%d$N', TO_WORD(LWORD#10));          // CHECK: 10

    printf('%d$N', TO_DWORD(SINT#10));           // CHECK: 10
    printf('%d$N', TO_DWORD(USINT#10));          // CHECK: 10
    printf('%d$N', TO_DWORD(INT#10));            // CHECK: 10
    printf('%d$N', TO_DWORD(UINT#10));           // CHECK: 10
    printf('%d$N', TO_DWORD(DINT#10));           // CHECK: 10
    printf('%d$N', TO_DWORD(UDINT#10));          // CHECK: 10
    printf('%d$N', TO_DWORD(LINT#10));           // CHECK: 10
    printf('%d$N', TO_DWORD(ULINT#10));          // CHECK: 10
    printf('%d$N', TO_DWORD(REAL#10.0));         // CHECK: 10
    printf('%d$N', TO_DWORD(LREAL#10.0));        // CHECK: 10
    printf('%d$N', TO_DWORD(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_DWORD(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_DWORD(BYTE#10));           // CHECK: 10
    printf('%d$N', TO_DWORD(WORD#10));           // CHECK: 10
    printf('%d$N', TO_DWORD(DWORD#10));          // CHECK: 10
    printf('%d$N', TO_DWORD(LWORD#10));          // CHECK: 10
END_FUNCTION

    // "SINT",
    // "USINT",
    // "INT",
    // "UINT",
    // "DINT",
    // "UDINT",
    // "LINT",
    // "ULINT",
    // "REAL",
    // "LREAL",

    // "BOOL",
    // "BYTE",
    // "WORD",
    // "DWORD",
    // "LWORD",