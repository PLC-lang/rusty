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
myVar := red;
END_FUNCTION

PROGRAM prog
    fn();

    // CHECK: no output
    printf('no output$N');
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
