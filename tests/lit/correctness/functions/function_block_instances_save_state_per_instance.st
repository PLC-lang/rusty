// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK foo
    VAR_INPUT
        i: INT;
    END_VAR

    i := i + 1;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        f: foo;
        j: foo;
    END_VAR

    f();
    f();
    j(4);
    j();
    j();

    printf('f.i = %d$N', f.i); // CHECK: f.i = 2
    printf('j.i = %d$N', j.i); // CHECK: j.i = 7
END_FUNCTION