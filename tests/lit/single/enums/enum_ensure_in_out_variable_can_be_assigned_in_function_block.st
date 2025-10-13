// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE EnumType : (
        red,
        green,
        blue
    );
END_TYPE

FUNCTION_BLOCK fb
VAR_IN_OUT
    inOutVar	: EnumType;
END_VAR
inOutVar := blue;
END_FUNCTION_BLOCK

PROGRAM prog
VAR
    inOutVar : EnumType;
    fb : fb;
END_VAR
    fb(inOutVar);

    // CHECK: 2
    printf('%d$N', inOutVar);
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
