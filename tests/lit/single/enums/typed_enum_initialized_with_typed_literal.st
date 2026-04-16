// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM mainProg
    foo();
END_PROGRAM

FUNCTION foo
    VAR
        vEnum : myEnum := e2;
    END_VAR

    // CHECK: 30
    printf('%d$N', vEnum);

END_FUNCTION

TYPE
	myEnum : (e1, e2:=BYTE#30) BYTE;
END_TYPE

FUNCTION main
    mainProg();
END_FUNCTION
