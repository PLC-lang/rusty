// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    VAR
        cnt : CTU;
    END_VAR

    cnt(CU := TRUE, R := FALSE, PV := 100);

    PRINTF('CU=%d, R=%d, PV=%d, Q=%d, CV=%d$N', cnt.CU, cnt.R, cnt.PV, cnt.Q, cnt.CV);
    // CHECK: CU=1, R=0, PV=100, Q=0, CV=1

    cnt(CU := FALSE, R := FALSE);

    PRINTF('CU=%d, R=%d, PV=%d, Q=%d, CV=%d$N', cnt.CU, cnt.R, cnt.PV, cnt.Q, cnt.CV);
    // CHECK-NEXT: CU=0, R=0, PV=100, Q=0, CV=1

    cnt(CU := TRUE, R := TRUE);

    PRINTF('CU=%d, R=%d, PV=%d, Q=%d, CV=%d$N', cnt.CU, cnt.R, cnt.PV, cnt.Q, cnt.CV);
    // CHECK-NEXT: CU=1, R=1, PV=100, Q=0, CV=0

    cnt(CU := TRUE, R := FALSE);

    PRINTF('CU=%d, R=%d, PV=%d, Q=%d, CV=%d$N', cnt.CU, cnt.R, cnt.PV, cnt.Q, cnt.CV);
    // CHECK-NEXT: CU=1, R=0, PV=100, Q=0, CV=0

END_FUNCTION
