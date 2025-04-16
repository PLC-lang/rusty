// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE
    bar : STRUCT
        f: foo; 
    END_STRUCT;
END_TYPE

FUNCTION_BLOCK foo
VAR
    x : INT := 0;
    y : INT := 0;
END_VAR
    METHOD FB_INIT
        x := 1;
        y := 2;
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM prog 
VAR 
    str: bar;
END_VAR
    printf('%d$N', str.f.x); // CHECK: 1
    printf('%d$N', str.f.y); // CHECK: 2
END_PROGRAM

FUNCTION main: DINT
    prog();
END_FUNCTION