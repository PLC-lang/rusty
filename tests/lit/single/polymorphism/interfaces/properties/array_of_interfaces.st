// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An array of interface references, each holding a different concrete instance.
// Iterates over the array and reads/writes properties through the interface.

INTERFACE IA
    PROPERTY id : DINT
        GET END_GET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK Fb1 IMPLEMENTS IA
    PROPERTY id : DINT
        GET
            id := 1;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK Fb2 IMPLEMENTS IA
    PROPERTY id : DINT
        GET
            id := 2;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK Fb3 IMPLEMENTS IA
    PROPERTY id : DINT
        GET
            id := 3;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        inst1 : Fb1;
        inst2 : Fb2;
        inst3 : Fb3;
        refs : ARRAY[1..3] OF IA;
        i : DINT;
    END_VAR

    refs[1] := inst1;
    refs[2] := inst2;
    refs[3] := inst3;

    // CHECK: 1
    // CHECK: 2
    // CHECK: 3
    FOR i := 1 TO 3 DO
        printf('%d$N', refs[i].id);
    END_FOR;
END_FUNCTION
