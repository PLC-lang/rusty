CLASS MyClass
    VAR
        x, y : INT;
    END_VAR

    METHOD testMethod
        VAR_INPUT myMethodArg : INT; END_VAR
        VAR myMethodLocalVar : INT; END_VAR

        x := myMethodArg;
    END_METHOD
END_CLASS
