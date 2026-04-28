// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK Foo
    VAR_TEMP
        tmp : INT;
    END_VAR

    VAR_INPUT
        x : INT;
    END_VAR

    x := x + 1;
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        fb : Foo;
    END_VAR

    fb(5);

    // CHECK: 6
    printf('%d$N', fb.x);
END_FUNCTION
