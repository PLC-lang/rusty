// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// CHECK: 5
FUNCTION main : DINT
    VAR
        foo : REFERENCE TO DINT;
        bar : REFERENCE TO DINT;
        qux : DINT;
    END_VAR

    bar REF= qux;
    foo REF= bar;
    qux := 5;

    printf('%d$N', foo); // foo -> bar -> qux
END_FUNCTION
