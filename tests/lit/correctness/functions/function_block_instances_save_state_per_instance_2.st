// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK Baz
    VAR_INPUT
        i: INT;
    END_VAR

    i := i + 1;
END_FUNCTION_BLOCK

FUNCTION_BLOCK foo
    VAR_INPUT
        i: INT;
        baz: Baz;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        f: foo;
        j: foo;
    END_VAR

    f.baz.i := f.baz.i + 1;
    f.baz.i := f.baz.i + 1;


    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;

    printf('f.baz.i = %d$N', f.baz.i); // CHECK: f.baz.i = 2
    printf('j.baz.i = %d$N', j.baz.i); // CHECK: j.baz.i = 4
END_FUNCTION
