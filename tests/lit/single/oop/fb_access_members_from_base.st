// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
VAR
    x : INT;
    y : INT;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
END_FUNCTION_BLOCK

FUNCTION_BLOCK foo
VAR 
    myFb : fb2;
END_VAR
    myFb.x := 1;
    printf('%d$N', myFb.x); //CHECK: 1
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        fb :foo;
    END_VAR
    fb();
END_FUNCTION