// STRING_TO_BOOL accepts true values and rejects malformed values.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    printf('garbage=%d$N', STRING_TO_BOOL('garbage'));       // CHECK: garbage=0
    printf('numeric_one=%d$N', STRING_TO_BOOL('1'));         // CHECK-NEXT: numeric_one=1
    printf('upper_true=%d$N', STRING_TO_BOOL('TRUE'));       // CHECK-NEXT: upper_true=1
    printf('lower_true=%d$N', STRING_TO_BOOL('true'));       // CHECK-NEXT: lower_true=1
    printf('trimmed_true=%d$N', STRING_TO_BOOL('  true  ')); // CHECK-NEXT: trimmed_true=1
    printf('numeric_zero=%d$N', STRING_TO_BOOL('0'));        // CHECK-NEXT: numeric_zero=0
    printf('numeric_two=%d$N', STRING_TO_BOOL('2'));         // CHECK-NEXT: numeric_two=0
    printf('false=%d$N', STRING_TO_BOOL('false'));           // CHECK-NEXT: false=0
    printf('suffix=%d$N', STRING_TO_BOOL('TRUEX'));          // CHECK-NEXT: suffix=0
END_FUNCTION
