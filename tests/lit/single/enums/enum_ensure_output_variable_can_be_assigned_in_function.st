// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE EnumType : (
        red,
        green,
        blue
    );
END_TYPE

FUNCTION fn : INT
VAR_OUTPUT
    outVar	: EnumType;
END_VAR
outVar := green;
END_FUNCTION

PROGRAM prog
VAR
    outVar: EnumType;
END_VAR
    fn(outVar);

    // CHECK: 1
    printf('%d$N', outVar);
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
