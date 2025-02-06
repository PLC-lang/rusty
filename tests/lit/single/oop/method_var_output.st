// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
METHOD baz
VAR_OUTPUT 
    out : STRING;
END_VAR
    out := 'hello';
END_METHOD
END_FUNCTION_BLOCK


FUNCTION main
VAR 
    s: STRING;
    fb: foo; 
END_VAR
    fb.baz(out => s);
    printf('%s$n', REF(s)); // CHECK: hello
    s := 'reset';
    printf('%s$n', REF(s)); // CHECK: reset
    fb.baz(s);
    printf('%s$n', REF(s)); // CHECK: hello
END_FUNCTION