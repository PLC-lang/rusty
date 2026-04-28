// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM mainProg
    foo();
END_PROGRAM

FUNCTION foo
    VAR
        vEnum : myEnum := e2;
    END_VAR

    printf('%d$N', vEnum);

END_FUNCTION

TYPE
	myEnum : (e1, e2:=BYTE#30) BYTE;
END_TYPE

FUNCTION main
    // CHECK: 30
    mainProg();
END_FUNCTION
