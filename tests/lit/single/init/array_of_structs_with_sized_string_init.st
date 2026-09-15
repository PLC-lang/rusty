// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Motor : STRUCT
    name  : STRING[20];
    speed : REAL;
    id    : DINT;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        motors : ARRAY[0..1] OF Motor := [
            (name := 'Motor1', speed := 10.5, id := 1),
            (name := 'Motor2', speed := 20.5, id := 2)
        ];
    END_VAR

    // CHECK: Motor1 1
    printf('%s %d$N', motors[0].name, motors[0].id);

    // CHECK: Motor2 2
    printf('%s %d$N', motors[1].name, motors[1].id);
END_FUNCTION
