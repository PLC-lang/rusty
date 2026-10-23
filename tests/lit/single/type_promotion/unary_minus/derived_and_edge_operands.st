// RUN: %COMPILE %s && %RUN | %CHECK %s
// A negation of an expression, of a subrange or an enum, and of the smallest LINT, and a negation cut down to a narrow target.
TYPE
    SRU64 : ULINT(0..10);
    SRD : DINT(-10..10);
    EU64 : ULINT (EA := 10, EB := 11);
END_TYPE

VAR_GLOBAL CONSTANT
    csu : SRU64 := 10;
END_VAR

FUNCTION main : DINT
    VAR
        a : USINT := 200;
        s : SINT := -128;
        li : LINT := -9223372036854775808;
        su : SRU64 := 10;
        sd : SRD := -10;
        eu : EU64 := EB;
        d : DINT;
        l : LINT;
        s2 : SINT;
        lr : LREAL;
    END_VAR

    // A negated expression
    d := -(a + 1);
    printf('-(USINT 200 + 1) = %d$N', d);               // CHECK: -(USINT 200 + 1) = -201

    // The smallest LINT has no positive counterpart and wraps to itself
    l := -li;
    printf('-LINT min = %lld$N', l);                    // CHECK-NEXT: -LINT min = -9223372036854775808

    // The DINT result is cut down to a narrow target afterwards
    s2 := -s;
    printf('SINT := -SINT -128 = %d$N', s2);            // CHECK-NEXT: SINT := -SINT -128 = -128

    // A subrange or an enum is negated like its base type, for a variable and for a constant
    lr := -su;
    printf('-SRU64 10 = %.1f$N', lr);                   // CHECK-NEXT: -SRU64 10 = -10.0
    lr := -csu;
    printf('-constant SRU64 10 = %.1f$N', lr);          // CHECK-NEXT: -constant SRU64 10 = -10.0
    d := -sd;
    printf('-SRD -10 = %d$N', d);                       // CHECK-NEXT: -SRD -10 = 10
    l := -eu;
    printf('-EU64 EB = %lld$N', l);                     // CHECK-NEXT: -EU64 EB = -11
END_FUNCTION
