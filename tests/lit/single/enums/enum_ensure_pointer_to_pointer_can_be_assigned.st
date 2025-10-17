// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    someEnum : EnumType;
END_VAR

TYPE EnumType : (
        red,
        green,
        blue
    );
END_TYPE

FUNCTION fn : INT
VAR_IN_OUT
    myVar	: EnumType;
END_VAR
myVar := green;
END_FUNCTION

PROGRAM prog
VAR
    enumRef : reference to EnumType ref= someEnum;
END_VAR
    fn(enumRef);

    // CHECK: 1
    printf('%d$N', enumRef);
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
