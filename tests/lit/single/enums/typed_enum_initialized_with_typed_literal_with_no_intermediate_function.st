// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM mainProg
    VAR
        vEnum : myEnum := e2;
    END_VAR

    printf('%d$N', vEnum);
END_PROGRAM

TYPE
	myEnum : (e1, e2:=BYTE#30) BYTE;
END_TYPE

FUNCTION main
    // CHECK: 30
    mainProg();
END_FUNCTION
