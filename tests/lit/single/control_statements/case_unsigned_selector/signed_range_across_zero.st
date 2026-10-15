// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A range on a signed SINT selector compares the selector as a signed value, so ranges with
// negative bounds and ranges across zero match as written.

FUNCTION classify : DINT
    VAR_INPUT
        x : SINT;
    END_VAR
    CASE x OF
        -128..-101: classify := 1;
        -100..100:  classify := 2;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : SINT;
    END_VAR
    printf('%d: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(-128);
    show(-101);
    show(-100);
    show(0);
    show(100);
    show(101);
    show(127);
    // CHECK:      -128: 1
    // CHECK-NEXT: -101: 1
    // CHECK-NEXT: -100: 2
    // CHECK-NEXT: 0: 2
    // CHECK-NEXT: 100: 2
    // CHECK-NEXT: 101: 0
    // CHECK-NEXT: 127: 0
END_FUNCTION
