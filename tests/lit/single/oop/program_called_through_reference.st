// RUN: %COMPILE %s && %RUN | %CHECK %s
PROGRAM mainProg
VAR_INPUT
    in : DINT;
END_VAR
VAR
    x : DINT;
END_VAR
    METHOD setX
    VAR_INPUT
        v : DINT;
    END_VAR
        x := v;
    END_METHOD

    METHOD getX : DINT
        getX := x;
    END_METHOD

    x := x + in;
END_PROGRAM

FUNCTION main
VAR
    refProg : REFERENCE TO mainProg;
END_VAR
    refProg REF= mainProg;

    refProg.setX(5);
    printf('%d$N', refProg.getX()); // CHECK: 5

    refProg(in := 2);
    printf('%d$N', refProg.getX()); // CHECK: 7
END_FUNCTION
