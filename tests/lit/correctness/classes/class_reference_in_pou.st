// RUN: (%COMPILE %s && %RUN) | %CHECK %s

CLASS MyClass
    VAR
        x, y: INT;
    END_VAR

    METHOD testMethod: INT
        VAR_INPUT myMethodArg: INT; END_VAR
        VAR myMethodLocalVar: INT; END_VAR

        x := myMethodArg;
        y := x + 1;
        myMethodLocalVar := y + 1;
        testMethod := myMethodLocalVar + 1;
    END_METHOD
END_CLASS

FUNCTION main
    VAR
        cl: MyClass;
        x: INT := 0;
    END_VAR

    x := 1;
    cl.x := 1;
    x := x + cl.x;
    x := x + cl.testMethod(x);
    x := cl.testMethod(myMethodArg := x);

    printf('%d$N', x); // CHECK: 10
END_FUNCTION