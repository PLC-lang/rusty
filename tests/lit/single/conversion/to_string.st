// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    // TO_CHAR
    printf('%c$N', TO_CHAR(WCHAR#"a"));             // CHECK: a
    printf('%c$N', TO_CHAR(STRING#'foo'));          // CHECK: f

    // TO_WCHAR
    printf('%c$N', TO_WCHAR(CHAR#'a'));             // CHECK: a
    printf('%c$N', TO_WCHAR(WSTRING#"foo"));        // CHECK: f

    // TO_STRING
    printf('%s$N', REF(TO_STRING(CHAR#'a')));       // CHECK: a
    printf('%s$N', REF(TO_STRING(WSTRING#"foo")));  // CHECK: foo

    // TO_WSTRING
    printf('%s$N', REF(TO_STRING(TO_WSTRING(WCHAR#"a"))));      // CHECK: a
    printf('%s$N', REF(TO_STRING(TO_WSTRING(STRING#'foo'))));   // CHECK: foo
END_FUNCTION
