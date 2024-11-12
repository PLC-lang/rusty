// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    VAR
        myChar : CHAR;
        myString : STRING[1];
    END_VAR

    // TODO: There's a bug, printf('%s$N', TO_CHAR(STRING#'foo')); returns a false positive

    // TO_CHAR
    myChar := TO_CHAR(WCHAR#"a");
    printf('%c$N', myChar);    // CHECK: a

    myChar := TO_CHAR(STRING#'foo');
    printf('%c$N', myChar);             // CHECK: f

    // TO_WCHAR
    printf('%c$N', TO_WCHAR(CHAR#'a'));    // CHECK: a
    printf('%c$N', TO_WCHAR(WSTRING#"foo"));    // CHECK: f

    // TO_STRING
    myString := TO_STRING(CHAR#'a');
    printf('%s$N', REF(myString));    // CHECK: a

    printf('%s$N', REF(TO_STRING(WSTRING#"foo")));    // CHECK: foo

    // TO_WSTRING; yes, ~~fuck me~~ really we have to call TO_STRING on the result for printf to work...
    printf('%s$N', REF(TO_STRING(TO_WSTRING(WCHAR#"a"))));      // CHECK: a
    printf('%s$N', REF(TO_STRING(TO_WSTRING(STRING#'foo'))));   // CHECK: foo
END_FUNCTION