// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK grandparent
VAR
    x : INT := 10;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK parent EXTENDS grandparent
VAR
    y : INT := 20;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
    // Access parent member directly with SUPER^
    SUPER^.y := 100;
    // Access grandparent member through SUPER^
    SUPER^.x := 200;
END_FUNCTION_BLOCK

FUNCTION main
VAR
    c : child;
END_VAR
    c();
    // Check if the values are set correctly
    printf('%d$N', c.x); // CHECK: 200
    printf('%d$N', c.y); // CHECK: 100
END_FUNCTION