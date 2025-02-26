// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
    VAR
        x: DINT := 50;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK bar EXTENDS foo
    METHOD set0 // TODO(volsa): https://github.com/PLC-lang/rusty/issues/1408
        x := 25;
    END_METHOD

    METHOD shadowed
    VAR
        x: DINT := 100;
    END_VAR
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: bar;
    END_VAR

    instance.set0();
    printf('%d$N', instance.x); // CHECK: 25
    // assigning to a locally shadowed variable should not change the parent variable
    instance.shadowed();
    printf('%d$N', instance.x); // CHECK: 25
END_FUNCTION
