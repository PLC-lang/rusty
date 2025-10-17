// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE EnumType : (
        red,
        green,
        blue
    );
END_TYPE

FUNCTION_BLOCK fb
VAR_OUTPUT
    outVar	: EnumType;
END_VAR
outVar := green;
END_FUNCTION_BLOCK

PROGRAM prog
VAR
    outVar: EnumType;
    fb : fb;
END_VAR
    fb(outVar);

    // CHECK: 1
    printf('%d$N', outVar);
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
