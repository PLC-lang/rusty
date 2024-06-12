// TODO: The printf should be a standalone ST file which we link with each other ST file
{external}
FUNCTION printf : DINT
    VAR_INPUT {ref}
    format : STRING;
    END_VAR
    VAR_INPUT
    args : ...;
    END_VAR
END_FUNCTION

// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    VAR
        x : DINT;
        y : DINT;
    END_VAR

    x := 60;
    y := 9;
    printf('%d$N', x + y);
    // CHECK: 68
END_FUNCTION
