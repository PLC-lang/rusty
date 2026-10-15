// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK FB
    VAR
        value : DINT;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        fb  : FB;
        rfb : REFERENCE TO FB;
    END_VAR

    rfb REF= fb;
    rfb.value := 42;

    // CHECK: 42
    printf('%d$N', fb.value);
END_FUNCTION
