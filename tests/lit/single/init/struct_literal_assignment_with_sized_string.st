// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Motor : STRUCT
    name  : STRING[20];
    speed : REAL;
    id    : DINT;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        motor  : Motor;
        motors : ARRAY[0..1] OF Motor;
    END_VAR

    motor := (name := 'Motor1', speed := 10.5, id := 1);
    motors[1] := (name := 'Motor2', speed := 20.5, id := 2);

    // CHECK: Motor1 1
    printf('%s %d$N', motor.name, motor.id);

    // CHECK: Motor2 2
    printf('%s %d$N', motors[1].name, motors[1].id);
END_FUNCTION
