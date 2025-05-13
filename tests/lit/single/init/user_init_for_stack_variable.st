// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
    METHOD FB_INIT
        printf('FB_INIT called$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
VAR
    fb: foo;
END_VAR
    // CHECK: FB_INIT called
END_FUNCTION