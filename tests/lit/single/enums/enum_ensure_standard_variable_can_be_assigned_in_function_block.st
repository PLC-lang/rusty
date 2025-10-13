// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE EnumType : (
        red,
        green,
        blue
    );
END_TYPE

FUNCTION_BLOCK fb
VAR
    myVar	: EnumType;
END_VAR
myVar := red;
END_FUNCTION_BLOCK

PROGRAM prog
VAR
    fb : fb;
END_VAR
    fb();

    // CHECK: no output
    printf('no output$N');
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
