// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK foo
    VAR
        placeholder_one: DINT;
    END_VAR

    VAR_INPUT
        a, b: DINT;
    END_VAR

    VAR
        placeholder_two: DINT;
    END_VAR

    VAR_OUTPUT
        c: DINT;
    END_VAR

    c := a + b;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        foo: foo;
        local_a, local_b, local_c: DINT := 2;
    END_VAR

    foo(local_a, local_b, local_c);
    printf('%d$N', local_c); // CHECK: 4
END_FUNCTION