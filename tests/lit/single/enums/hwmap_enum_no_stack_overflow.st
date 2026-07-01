// --hwmap-file must not stack-overflow on programs that
// contain enum typed variables. The instance iterator used to descend into
// enum variants, re-entering the same enum type forever until the stack
// was exhausted.

// First we make sure that the hwmap file does not stack overflow
// RUN: timeout 10s %PLC -i %UTILS --check --hwmap-file=%T/%basename_t.hwmap.json %s

// Then we do a sanity run to make sure the program executes to completion and produces the expected output
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE enumType : (
    element0,
    element1
);
END_TYPE

VAR_GLOBAL
    myGlobalEnum : enumType;
END_VAR

PROGRAM mainProg
VAR
    myLocalEnum : enumType;
END_VAR
END_PROGRAM

FUNCTION main
    mainProg();

    // CHECK: Complete
    printf('Complete$N');
END_FUNCTION
