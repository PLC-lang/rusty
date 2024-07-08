// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// CHECK: 2
FUNCTION main
    VAR
        res : LREAL;
    END_VAR
    printf('%d$N', REAL_TO_DINT(SQRT(4.0)));
END_FUNCTION
