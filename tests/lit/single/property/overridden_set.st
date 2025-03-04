// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
PROPERTY baz : INT
    GET END_GET
END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK bar EXTENDS foo
PROPERTY baz : INT
    SET END_SET
END_PROPERTY
    baz := baz + 42;
END_FUNCTION_BLOCK

FUNCTION main : DINT 
VAR 
    fb: bar;
END_VAR
    fb();
    printf('%d$N', fb.baz); // CHECK: 42
    fb();
    printf('%d$N', fb.baz); // CHECK: 84 
END_FUNCTION