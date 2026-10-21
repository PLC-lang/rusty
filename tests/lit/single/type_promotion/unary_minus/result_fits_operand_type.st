// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION main : DINT
    VAR
        s : SINT := 100;
        i : INT := 30000;
        us : USINT := 100;
        ui : UINT := 30000;
        d : DINT := -2147483647;
        ud : UDINT := 5;
        l : LINT := 9223372036854775807;
        ul : ULINT := 5;
        r : REAL := 1.5;
        rd : DINT;
        rl : LINT;
        rr : REAL;
    END_VAR

    // Narrow operands whose negation fits the operand's width
    rd := -s;
    printf('-SINT 100 = %d$N', rd);         // CHECK: -SINT 100 = -100
    rd := -i;
    printf('-INT 30000 = %d$N', rd);        // CHECK-NEXT: -INT 30000 = -30000
    rd := -us;
    printf('-USINT 100 = %d$N', rd);        // CHECK-NEXT: -USINT 100 = -100
    rd := -ui;
    printf('-UINT 30000 = %d$N', rd);       // CHECK-NEXT: -UINT 30000 = -30000

    // 32-bit and 64-bit operands
    rd := -d;
    printf('-DINT = %d$N', rd);             // CHECK-NEXT: -DINT = 2147483647
    rd := -ud;
    printf('-UDINT 5 = %d$N', rd);          // CHECK-NEXT: -UDINT 5 = -5
    rl := -l;
    printf('-LINT = %lld$N', rl);           // CHECK-NEXT: -LINT = -9223372036854775807
    rl := -ul;
    printf('-ULINT 5 = %lld$N', rl);        // CHECK-NEXT: -ULINT 5 = -5

    // Reals, double negation and the binary form
    rr := -r;
    printf('-REAL 1.5 = %f$N', rr);         // CHECK-NEXT: -REAL 1.5 = -1.500000
    rd := -(-s);
    printf('-(-SINT 100) = %d$N', rd);      // CHECK-NEXT: -(-SINT 100) = 100
    us := 200;
    rd := 0 - us;
    printf('0 - USINT 200 = %d$N', rd);     // CHECK-NEXT: 0 - USINT 200 = -200
END_FUNCTION
