// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK MyClass
    VAR
        x, y : INT;
    END_VAR

    METHOD testMethod
    VAR_INPUT
        myMethodArg : INT := 3;
    END_VAR
        x := myMethodArg;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
VAR
cl : MyClass;
END_VAR
    cl.testMethod();
    printf('%d$N', cl.x); // CHECK: 3
END_FUNCTION
