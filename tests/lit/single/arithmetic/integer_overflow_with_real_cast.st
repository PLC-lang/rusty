// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    VAR
        var_int     : INT;
    END_VAR

    printf('%d$N', var_int); // CHECK: 0

    // Cast the REAL value to INT
    var_int := REAL_TO_INT(70_000.4);

    printf('%d$N', var_int); // CHECK: 4464

END_FUNCTION
