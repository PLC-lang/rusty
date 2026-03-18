// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main: DINT
    VAR
        x : UDINT := 21;
        y : LREAL := 42;
    END_VAR

    // CHECK: X(21): Y = 42.000000
    printf('X(%u): Y = %f', x, y);
END_FUNCTION
