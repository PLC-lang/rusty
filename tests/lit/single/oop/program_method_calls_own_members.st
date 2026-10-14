// RUN: %COMPILE %s && %RUN | %CHECK %s
PROGRAM mainProg
VAR
    _value : DINT;
END_VAR
    PROPERTY_GET value : DINT
        value := _value;
    END_PROPERTY

    PROPERTY_SET value : DINT
        _value := value;
    END_PROPERTY

    METHOD sum : DINT
    VAR_INPUT
        in1, in2 : DINT;
    END_VAR
        sum := in1 + in2;
    END_METHOD

    METHOD run
        printf('%d$N', sum(1, 2));               // CHECK: 3
        printf('%d$N', sum(in1 := 10, in2 := 20)); // CHECK: 30

        value := 7;
        printf('%d$N', value);                   // CHECK: 7
    END_METHOD
END_PROGRAM

FUNCTION main
    mainProg.run();
END_FUNCTION
