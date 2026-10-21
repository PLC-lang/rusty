// RUN: %COMPILE %s && %RUN | %CHECK %s
// The comparison functions compare two UDINT values unsigned, as the operators do.
FUNCTION main : DINT
    VAR
        hi : UDINT := 4000000000;
        mid : UDINT := 3000000000;
        lo : UDINT := 5;
    END_VAR

    printf('GT(hi, lo) = %d$N', GT(hi, lo));                // CHECK: GT(hi, lo) = 1
    printf('GE(hi, lo) = %d$N', GE(hi, lo));                // CHECK-NEXT: GE(hi, lo) = 1
    printf('LT(lo, hi) = %d$N', LT(lo, hi));                // CHECK-NEXT: LT(lo, hi) = 1
    printf('LE(lo, hi) = %d$N', LE(lo, hi));                // CHECK-NEXT: LE(lo, hi) = 1
    printf('GT(hi, mid, lo) = %d$N', GT(hi, mid, lo));      // CHECK-NEXT: GT(hi, mid, lo) = 1
    printf('LT(lo, mid, hi) = %d$N', LT(lo, mid, hi));      // CHECK-NEXT: LT(lo, mid, hi) = 1
    printf('MAX(hi, lo) = %u$N', MAX(hi, lo));              // CHECK-NEXT: MAX(hi, lo) = 4000000000
    printf('MIN(hi, lo) = %u$N', MIN(hi, lo));              // CHECK-NEXT: MIN(hi, lo) = 5
    printf('LIMIT(lo, hi, mid) = %u$N', LIMIT(lo, hi, mid));    // CHECK-NEXT: LIMIT(lo, hi, mid) = 3000000000
END_FUNCTION
