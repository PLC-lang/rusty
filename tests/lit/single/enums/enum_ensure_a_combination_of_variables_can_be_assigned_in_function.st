// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE EnumType : (
        red,
        green,
        blue
    );
END_TYPE

FUNCTION fn : INT
VAR
    myVar	: EnumType;
END_VAR
VAR_OUTPUT
    outVar	: EnumType;
END_VAR
VAR_IN_OUT
    inOutVar	: EnumType;
END_VAR
myVar := red;
outVar := green;
inOutVar := blue;
END_FUNCTION

PROGRAM prog
VAR
    inOutVar : EnumType;
    outVar: EnumType;
END_VAR
    fn(outVar, inOutVar);

    // CHECK: 1
    printf('%d$N', outVar);
    // CHECK: 2
    printf('%d$N', inOutVar);
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
