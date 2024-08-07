// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    VAR
        foo : REFERENCE TO DINT;
        bar : REFERENCE TO DINT;
        qux : DINT;
    END_VAR

    foo REF= bar;
    bar REF= qux;

    bar REF= bar;
    qux := 5;

    // CHECK: 5
    printf('%d$N', bar); // bar (-> bar) -> qux
END_FUNCTION