// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION foo
VAR_IN_OUT
    str80_io : STRING;
    str10_io : STRING[10];
END_VAR
    str80_io := 'hello';
    str10_io := 'ralph';
END_FUNCTION

FUNCTION main
VAR
    str80 : STRING;
    str10 : STRING[10];
END_VAR
    foo(str80, str10);
    // CHECK: ralph hello
    printf('%s %s$N', REF(str10), REF(str80));
END_FUNCTION
