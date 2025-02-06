// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
VAR
    s : STRING;
END_VAR
METHOD baz
    s := 'hello';
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK bar EXTENDS foo
    s := 'world';
END_FUNCTION_BLOCK

FUNCTION main: DINT
VAR
    fb: bar;
END_VAR
    fb.baz();
    printf('%s$N', REF(fb.s)); // CHECK: hello
    fb();
    printf('%s$N', REF(fb.s)); // CHECK: world
END_FUNCTION
