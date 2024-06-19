// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    VAR
        foo : REFERENCE TO DINT;
        bar : REFERENCE TO DINT;
        qux : DINT;
    END_VAR

    foo REF= bar;

    // CHECK-NOT: ^5$
    printf('%d$N', foo); // We expect some random value here because foo isn't pointing to any variable yet

    // CHECK-NOT: ^5$
    printf('%d$N', bar); // Same as above

    bar REF= qux;
    qux := 5;

    // CHECK: 5
    printf('%d$N', bar); // We expect 5 here because bar references qux, and qux has a value of 5
END_FUNCTION