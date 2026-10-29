// RUN: %COMPILE %s && %RUN | %CHECK %s
// The operand of a negation is widened to DINT first, so a result outside the operand's width does not wrap.
FUNCTION main : DINT
    VAR
        s : SINT := -128;
        i : INT := -32768;
        us : USINT := 200;
        usmax : USINT := 255;
        ui : UINT := 40000;
        uimax : UINT := 65535;
        rd : DINT;
        rl : LINT;
        ri : INT;
    END_VAR

    // Signed minimum values
    rd := -s;
    printf('-SINT -128 = %d$N', rd);        // CHECK: -SINT -128 = 128
    rd := -i;
    printf('-INT -32768 = %d$N', rd);       // CHECK-NEXT: -INT -32768 = 32768

    // Unsigned values above the signed range of the same width
    rd := -us;
    printf('-USINT 200 = %d$N', rd);        // CHECK-NEXT: -USINT 200 = -200
    rd := -usmax;
    printf('-USINT 255 = %d$N', rd);        // CHECK-NEXT: -USINT 255 = -255
    rd := -ui;
    printf('-UINT 40000 = %d$N', rd);       // CHECK-NEXT: -UINT 40000 = -40000
    rd := -uimax;
    printf('-UINT 65535 = %d$N', rd);       // CHECK-NEXT: -UINT 65535 = -65535

    // Typed literal operand
    rd := -USINT#200;
    printf('-USINT#200 = %d$N', rd);        // CHECK-NEXT: -USINT#200 = -200

    // Negation inside a larger expression
    rd := -us + 1;
    printf('-USINT 200 + 1 = %d$N', rd);    // CHECK-NEXT: -USINT 200 + 1 = -199
    printf('-USINT 200 < 0 = %d$N', -us < 0);   // CHECK-NEXT: -USINT 200 < 0 = 1
    printf('-UINT 40000 < 0 = %d$N', -ui < 0);  // CHECK-NEXT: -UINT 40000 < 0 = 1

    // Other targets
    rl := -i;
    printf('LINT := -INT -32768 = %lld$N', rl); // CHECK-NEXT: LINT := -INT -32768 = 32768
    ri := -s;
    printf('INT := -SINT -128 = %d$N', ri);     // CHECK-NEXT: INT := -SINT -128 = 128
END_FUNCTION
