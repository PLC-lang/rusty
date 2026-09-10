// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Motor : STRUCT
    name : STRING[20];
    id   : DINT;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        motor      : Motor;
        long_name  : STRING := 'Motor3';
        short_name : STRING[5] := 'M4';
    END_VAR

    motor := (name := long_name, id := 3);
    // CHECK: Motor3 3
    printf('%s %d$N', motor.name, motor.id);

    motor := (name := short_name, id := 4);
    // CHECK: M4 4
    printf('%s %d$N', motor.name, motor.id);
END_FUNCTION
