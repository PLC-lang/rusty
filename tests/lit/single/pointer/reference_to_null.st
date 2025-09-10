// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main: DINT
    VAR
        foo : REFERENCE TO DINT;
        foo2 : REFERENCE TO INT;
        bar,baz,qux : DINT;
    END_VAR

    // Set to null
    foo REF= 0;
    // Should not crash

    // Set to null
    foo2 REF= 0;
    // Should not crash

    foo REF= bar;
    bar := 1;
    baz := 2;
    qux := 2;

    // CHECK: 2
    bar := bar + foo; // bar + bar => 1 + 1
    printf('%d$N', foo);

    // CHECK: 4
    baz := baz + foo; // baz + foo => baz + bar => 2 + 2
    foo REF= baz;
    printf('%d$N', foo);

    // CHECK: 6
    qux := qux + foo; // qux + foo => qux + baz => 2 + 4
    foo REF= qux;
    printf('%d$N', foo);

END_FUNCTION
