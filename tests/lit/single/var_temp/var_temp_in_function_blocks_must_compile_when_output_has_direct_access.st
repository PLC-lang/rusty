// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK Foo
    VAR_TEMP
        tmp : INT;
    END_VAR

    VAR_OUTPUT
        q : BOOL;
    END_VAR

    q := TRUE;
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        fb : Foo;
        result : BYTE;
    END_VAR

    fb(q => result.0);

    // CHECK: 1
    printf('%d$N', result.0);
END_FUNCTION
