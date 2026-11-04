// RUN: %COMPILE %s && %RUN | %CHECK %s

// A reference to a variant of `Error` resolves to that variant, also when `Status`, declared
// first, has a variant with the same name as the type and the variant (`Error`).
TYPE Status : (Ok := 0, Error := 1);
END_TYPE

TYPE Error : (Timeout := 5, Error := 7);
END_TYPE

VAR_GLOBAL CONSTANT
    gCast : Error := Error#Error;
    gDot : Error := Error.Error;
END_VAR

// No local of type `Error`, so the variants of `Error` are not in the scope of the POU.
FUNCTION castWithoutLocal : DINT
    castWithoutLocal := Error#Error;
END_FUNCTION

FUNCTION dotWithoutLocal : DINT
    dotWithoutLocal := Error.Error;
END_FUNCTION

FUNCTION main : DINT
VAR
    initCast : Error := Error#Error;
    initDot : Error := Error.Error;
    bodyCast : Error;
    bodyDot : Error;
END_VAR
    bodyCast := Error#Error;
    bodyDot := Error.Error;

    // CHECK: init 7 7
    printf('init %d %d$N', initCast, initDot);
    // CHECK: body 7 7
    printf('body %d %d$N', bodyCast, bodyDot);
    // CHECK: global 7 7
    printf('global %d %d$N', gCast, gDot);
    // CHECK: without local 7 7
    printf('without local %d %d$N', castWithoutLocal(), dotWithoutLocal());
    // CHECK: status 1
    printf('status %d$N', Status#Error);
END_FUNCTION
