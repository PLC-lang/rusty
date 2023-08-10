    CLASS MyClass
    VAR
        x : DINT;
        y : DINT;
    END_VAR
    END_CLASS

    PROGRAM MyProg
    VAR_IN_OUT
        cls : MyClass;
    END_VAR
    VAR_OUTPUT
        x : DINT;
        y : DINT;
    END_VAR
    x := cls.x;
    cls.y := y;
    END_PROGRAM

    FUNCTION main : DINT
    VAR_TEMP
        cls : MyClass;
    END_VAR
    VAR
        x : DINT;
        y : DINT;
    END_VAR
    cls.x := 2;
    MyProg.y := 3;
    MyProg(cls);
    x := MyProg.x;
    y := cls.y;
    main := x + y;
    END_FUNCTION