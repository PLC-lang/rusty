// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    VAR
        cnt : CTD;
    END_VAR

    // Load counter with PV set to 100
    cnt(LD := TRUE, PV := 100, CD := TRUE);

    PRINTF('CD=%d, LD=%d, PV=%d, Q=%d, CV=%d$N', cnt.CD, cnt.LD, cnt.PV, cnt.Q, cnt.CV);
    // CHECK: CD=1, LD=1, PV=100, Q=0, CV=100

    cnt(LD := FALSE);

    PRINTF('CD=%d, LD=%d, PV=%d, Q=%d, CV=%d$N', cnt.CD, cnt.LD, cnt.PV, cnt.Q, cnt.CV);
    // CHECK-NEXT: CD=1, LD=0, PV=100, Q=0, CV=100

    cnt(CD := FALSE, LD := FALSE);

    PRINTF('CD=%d, LD=%d, PV=%d, Q=%d, CV=%d$N', cnt.CD, cnt.LD, cnt.PV, cnt.Q, cnt.CV);
    // CHECK-NEXT: CD=0, LD=0, PV=100, Q=0, CV=100

    cnt(CD := TRUE);

    PRINTF('CD=%d, LD=%d, PV=%d, Q=%d, CV=%d$N', cnt.CD, cnt.LD, cnt.PV, cnt.Q, cnt.CV);
    // CHECK: CD=1, LD=0, PV=100, Q=0, CV=99

END_FUNCTION
