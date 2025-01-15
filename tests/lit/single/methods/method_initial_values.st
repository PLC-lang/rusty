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
    printf('%d$N', x); // CHECK: 3
END_FUNCTION_BLOCK

PROGRAM prg
VAR
cl : MyClass;
END_VAR
    // cl.testMethod(); // FIXME: validation checks for input params which need not be there
END_PROGRAM