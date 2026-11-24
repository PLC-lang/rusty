// RUN: %COMPILE %s && %RUN | %CHECK %s
// The relational operators compare BOOL values with FALSE < TRUE.
FUNCTION main : DINT
    VAR
        f : BOOL := FALSE;
        t : BOOL := TRUE;
    END_VAR

    printf('FALSE < TRUE = %d, TRUE < FALSE = %d$N', f < t, t < f);          // CHECK: FALSE < TRUE = 1, TRUE < FALSE = 0
    printf('TRUE > FALSE = %d, FALSE > TRUE = %d$N', t > f, f > t);          // CHECK-NEXT: TRUE > FALSE = 1, FALSE > TRUE = 0
    printf('TRUE <= TRUE = %d, TRUE <= FALSE = %d$N', t <= t, t <= f);       // CHECK-NEXT: TRUE <= TRUE = 1, TRUE <= FALSE = 0
    printf('FALSE >= FALSE = %d, FALSE >= TRUE = %d$N', f >= f, f >= TRUE);  // CHECK-NEXT: FALSE >= FALSE = 1, FALSE >= TRUE = 0
END_FUNCTION
