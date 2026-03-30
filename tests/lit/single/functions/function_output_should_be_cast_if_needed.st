// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION outputCastIntToReal : INT
    VAR_OUTPUT
        result : REAL;
    END_VAR

    result := 42.0;
END_FUNCTION

FUNCTION outputNoCastInt : INT
    VAR_OUTPUT
        result : INT;
    END_VAR

    result := 52;
END_FUNCTION

FUNCTION outputCastRealToInt : INT
    VAR_OUTPUT
        result : INT;
    END_VAR

    result := 62;
END_FUNCTION

FUNCTION outputNoCastReal : INT
    VAR_OUTPUT
        result : REAL;
    END_VAR

    result := 72.0;
END_FUNCTION

FUNCTION main
    VAR
        i1 : INT;
        i2 : INT;
        r1 : REAL;
        r2 : REAL;
    END_VAR

    i1 := 31;
    i2 := 32;
    r1 := 33;
    r2 := 34;

    // ----------------- //
    // -- INT to REAL -- //

    outputCastIntToReal(
        result => i1
    );

    // CHECK: 42
    PRINTF('%d$N', i1);

    // ---------------- //
    // -- INT to INT -- //

    outputNoCastInt(
        result => i2
    );

    // CHECK: 52
    PRINTF('%d$N', i2);

    // ----------------- //
    // -- REAL to INT -- //

    outputCastRealToInt(
        result => r1
    );

    // CHECK: 62.0
    PRINTF('%.1f$N', r1);

    // ------------------ //
    // -- REAL to REAL -- //

    outputNoCastReal(
        result => r2
    );

    // CHECK: 72.0
    PRINTF('%.1f$N', r2);
END_FUNCTION
