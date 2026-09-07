// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that date/time constructors yield 0 for unrepresentable inputs
// and clamp out-of-range dates to the DATE range

FUNCTION main : DINT
VAR
    a : DATE;
    b : TOD;
    c : DATE;
    d : DATE;
    e : DATE;
END_VAR
    a := CONCAT_DATE(INT#2024, INT#13, INT#45);
    // CHECK: 0
    printf('%lld$N', a);

    b := CONCAT_TOD(INT#25, INT#0, INT#0, INT#0);
    // CHECK: 0
    printf('%lld$N', b);

    // pre-1970 dates stay representable in the signed range
    c := CONCAT_DATE(INT#1969, INT#6, INT#1);
    // CHECK: -18489600000000000
    printf('%lld$N', c);

    d := CONCAT_DATE(INT#2024, INT#2, INT#29);
    // CHECK: 1709164800000000000
    printf('%lld$N', d);

    // beyond the representable range clamps to the range maximum
    e := CONCAT_DATE(INT#2500, INT#1, INT#1);
    // CHECK: 9223372036854775807
    printf('%lld$N', e);

    main := 0;
END_FUNCTION
