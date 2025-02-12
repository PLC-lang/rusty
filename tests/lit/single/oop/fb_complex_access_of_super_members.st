// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK grandparent
VAR
    y : ARRAY[0..5] OF INT;
    a : INT;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK parent extends grandparent
    VAR
        x : ARRAY[0..10] OF INT;
        b : INT;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
    VAR
        z : ARRAY[0..10] OF INT;
    END_VAR
    y[b + z[b*2] - a] := 20;
    b := 1; 
    z[b] := 3;
    z[b*2] := 4;
    a := 2;
    y[b + z[b*2] - a] := 60;
    printf('%d$N', y[0]); //CHECK: 20
    printf('%d$N', y[3]); //CHECK: 60
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        fb : child;
    END_VAR
    fb();
END_FUNCTION