// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// A RETURN in a getter returns the value of the property variable at the RETURN
FUNCTION_BLOCK fb
    VAR_INPUT
        mode : DINT;
    END_VAR

    PROPERTY_GET p : DINT
        IF mode = 1 THEN
            p := 5;
            RETURN;
        END_IF

        CASE mode OF
            2:
                p := 7;
                RETURN;
        END_CASE

        WHILE TRUE DO
            p := 8;
            IF mode = 3 THEN
                RETURN;
            END_IF
            EXIT;
        END_WHILE

        p := 6;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        f : fb;
    END_VAR

    // CHECK: 6
    printf('%d$N', f.p);

    f.mode := 1;
    // CHECK: 5
    printf('%d$N', f.p);

    f.mode := 2;
    // CHECK: 7
    printf('%d$N', f.p);

    f.mode := 3;
    // CHECK: 8
    printf('%d$N', f.p);
END_FUNCTION
