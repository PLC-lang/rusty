// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that date/time constructors yield 0 for unrepresentable inputs
// and clamp out-of-range dates to the DATE range

FUNCTION main : DINT
VAR
    a : DATE;
    b : TOD;
    c : DATE;
    d : DATE;
END_VAR
    a := CONCAT_DATE(INT#2024, INT#13, INT#45);
    // CHECK: 0
    printf('%u$N', a);

    b := CONCAT_TOD(INT#25, INT#0, INT#0, INT#0);
    // CHECK: 0
    printf('%u$N', b);

    c := CONCAT_DATE(INT#1969, INT#6, INT#1);
    // CHECK: 0
    printf('%u$N', c);

    d := CONCAT_DATE(INT#2024, INT#2, INT#29);
    // CHECK: 1709164800
    printf('%u$N', d);

    main := 0;
END_FUNCTION
