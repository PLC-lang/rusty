// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        val : LREAL := 3.14;
    END_VAR
    // CHECK: Value: 3.140000
    printf('Value: %f$N', val);
END_FUNCTION
