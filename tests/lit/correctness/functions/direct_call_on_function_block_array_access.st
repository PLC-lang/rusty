// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK foo
    VAR_INPUT
        i: INT;
    END_VAR
    VAR
        x: INT;
    END_VAR

    x := i;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        f: ARRAY[1..2] OF foo;
        x: INT;
        y: INT;
    END_VAR

    f[1](i := 10);
    x := f[1].x;

    f[2](i := 20);
    y := f[2].x;

    printf('x = %d$N', x); // CHECK: x = 10
    printf('y = %d$N', y); // CHECK: y = 20
END_FUNCTION