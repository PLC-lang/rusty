// RUN: %COMPILE %s && %RUN | %CHECK %s
// Compares whose result does not depend on the signedness of the predicate.
FUNCTION main : DINT
    VAR
        ud : UDINT := 4000000000;
        ud2 : UDINT := 4000000000;
        ud5 : UDINT := 5;
        ul : ULINT := 18446744073709551615;
        us : USINT := 200;
        us2 : USINT := 100;
        ui : UINT := 60000;
        l1 : LINT := 1;
        sm1 : SINT := -1;
        dm10 : DINT := -10;
    END_VAR

    // Equality on values from the signed midpoint on
    printf('UDINT = UDINT = %d$N', ud = ud2);               // CHECK: UDINT = UDINT = 1
    printf('UDINT <> 5 = %d$N', ud <> ud5);                 // CHECK-NEXT: UDINT <> 5 = 1
    printf('ULINT = max = %d$N', ul = 18446744073709551615);    // CHECK-NEXT: ULINT = max = 1

    // Narrow unsigned types are widened to DINT before the compare
    printf('USINT 200 > USINT 100 = %d$N', us > us2);       // CHECK-NEXT: USINT 200 > USINT 100 = 1
    printf('UINT 60000 > 2 = %d$N', ui > 2);                // CHECK-NEXT: UINT 60000 > 2 = 1
    printf('USINT 200 > SINT -1 = %d$N', us > sm1);         // CHECK-NEXT: USINT 200 > SINT -1 = 1

    // Small unsigned values
    printf('UDINT 5 < 10 = %d$N', ud5 < 10);                // CHECK-NEXT: UDINT 5 < 10 = 1
    printf('UDINT 5 < UDINT 4e9 = %d$N', ud5 < ud);         // CHECK-NEXT: UDINT 5 < UDINT 4e9 = 0

    // A wider signed operand holds both values
    printf('UDINT 4e9 > LINT 1 = %d$N', ud > l1);           // CHECK-NEXT: UDINT 4e9 > LINT 1 = 1
    printf('UDINT 4e9 > 16#8000_0000 = %d$N', ud > 16#8000_0000);  // CHECK-NEXT: UDINT 4e9 > 16#8000_0000 = 1

    // Negative signed operands next to unsigned values
    printf('UDINT 5 > -1 = %d$N', ud5 > -1);                // CHECK-NEXT: UDINT 5 > -1 = 1
    printf('UDINT 5 > SINT -1 = %d$N', ud5 > sm1);          // CHECK-NEXT: UDINT 5 > SINT -1 = 1
    printf('DINT -10 < UDINT 5 = %d$N', dm10 < ud5);        // CHECK-NEXT: DINT -10 < UDINT 5 = 1

    // TIME, DATE and TOD below 2^31
    printf('T#1h > T#1s = %d$N', T#1h > T#1s);              // CHECK-NEXT: T#1h > T#1s = 1
    printf('TOD#23:00 > TOD#01:00 = %d$N', TOD#23:00:00 > TOD#01:00:00);  // CHECK-NEXT: TOD#23:00 > TOD#01:00 = 1
    printf('D#2020 > D#2010 = %d$N', D#2020-01-01 > D#2010-01-01);        // CHECK-NEXT: D#2020 > D#2010 = 1
    printf('LTIME#25d > LTIME#1s = %d$N', LTIME#25d > LTIME#1s);          // CHECK-NEXT: LTIME#25d > LTIME#1s = 1
END_FUNCTION
