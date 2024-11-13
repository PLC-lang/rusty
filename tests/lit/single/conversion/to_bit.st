// Note: Tests are kept simple because there are already tests covering the underliny X_TO_Y functions, which
//       the TO_Y(X) functions are based on.

// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    printf('%d$N', TO_BOOL(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_BOOL(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_BOOL(BYTE#0));            // CHECK: 0
    printf('%d$N', TO_BOOL(BYTE#5));            // CHECK: 1
    printf('%d$N', TO_BOOL(WORD#0));            // CHECK: 0
    printf('%d$N', TO_BOOL(WORD#5));            // CHECK: 1
    printf('%d$N', TO_BOOL(DWORD#0));           // CHECK: 0
    printf('%d$N', TO_BOOL(DWORD#5));           // CHECK: 1
    printf('%d$N', TO_BOOL(LWORD#0));           // CHECK: 0
    printf('%d$N', TO_BOOL(LWORD#5));           // CHECK: 1

    printf('%d$N', TO_BYTE(BYTE#5));            // CHECK: 5
    printf('%d$N', TO_BYTE(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_BYTE(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_BYTE(CHAR#'a'));          // CHECK: 97
    printf('%d$N', TO_BYTE(WORD#5));            // CHECK: 5
    printf('%d$N', TO_BYTE(DWORD#5));           // CHECK: 5
    printf('%d$N', TO_BYTE(LWORD#5));           // CHECK: 5

    printf('%d$N', TO_WORD(WORD#5));            // CHECK: 5
    printf('%d$N', TO_WORD(BOOL#FALSE));        // CHECK: 0
    printf('%d$N', TO_WORD(BOOL#TRUE));         // CHECK: 1
    printf('%d$N', TO_WORD(CHAR#'a'));          // CHECK: 97
    printf('%d$N', TO_WORD(WCHAR#"a"));         // CHECK: 97
    printf('%d$N', TO_WORD(BYTE#5));            // CHECK: 5
    printf('%d$N', TO_WORD(DWORD#5));           // CHECK: 5
    printf('%d$N', TO_WORD(LWORD#5));           // CHECK: 5

    printf('%d$N', TO_DWORD(DWORD#5));          // CHECK: 5
    printf('%d$N', TO_DWORD(BOOL#FALSE));       // CHECK: 0
    printf('%d$N', TO_DWORD(BOOL#TRUE));        // CHECK: 1
    printf('%d$N', TO_DWORD(CHAR#'a'));         // CHECK: 97
    printf('%d$N', TO_DWORD(WCHAR#'a'));        // CHECK: 97
    printf('%d$N', TO_DWORD(BYTE#5));           // CHECK: 5
    printf('%d$N', TO_DWORD(WORD#5));           // CHECK: 5
    printf('%d$N', TO_DWORD(LWORD#5));          // CHECK: 5

    printf('%d$N', TO_LWORD(LWORD#5));          // CHECK: 5
    printf('%d$N', TO_LWORD(BOOL#FALSE));       // CHECK: 0
    printf('%d$N', TO_LWORD(BOOL#TRUE));        // CHECK: 1
    printf('%d$N', TO_LWORD(CHAR#'a'));         // CHECK: 97
    printf('%d$N', TO_LWORD(WCHAR#'a'));        // CHECK: 97
    printf('%d$N', TO_LWORD(BYTE#5));           // CHECK: 5
    printf('%d$N', TO_LWORD(WORD#5));           // CHECK: 5
    printf('%d$N', TO_LWORD(DWORD#5));          // CHECK: 5
END_FUNCTION