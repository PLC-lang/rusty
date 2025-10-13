// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE EnumType : (
        red,
        green,
        blue
    );
END_TYPE

FUNCTION fn : INT
VAR_IN_OUT
    inOutVar	: EnumType;
END_VAR
inOutVar := blue;
END_FUNCTION

PROGRAM prog
VAR
    inOutVar : EnumType;
END_VAR
    fn(inOutVar);

    // CHECK: 2
    printf('%d$N', inOutVar);
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
