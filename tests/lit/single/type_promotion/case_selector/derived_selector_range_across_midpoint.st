// RUN: %COMPILE %s && %RUN | %CHECK %s
// A range label that crosses the signed midpoint matches every value between its bounds, also through an alias or subrange type.
TYPE
    Level : USINT;
    Small : UINT(0..60000);
END_TYPE

FUNCTION branch_alias : DINT
    VAR_INPUT
        x : Level;
    END_VAR

    CASE x OF
        100..200: branch_alias := 1;
    ELSE
        branch_alias := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_subrange : DINT
    VAR_INPUT
        x : Small;
    END_VAR

    CASE x OF
        30000..50000: branch_subrange := 1;
    ELSE
        branch_subrange := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_list : DINT
    VAR_INPUT
        x : USINT;
    END_VAR

    CASE x OF
        1, 100..200, 250: branch_list := 1;
    ELSE
        branch_list := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    printf('alias 150 -> %d$N', branch_alias(150));             // CHECK: alias 150 -> 1
    printf('subrange 40000 -> %d$N', branch_subrange(40000));   // CHECK-NEXT: subrange 40000 -> 1
    printf('list 1 -> %d$N', branch_list(1));                   // CHECK-NEXT: list 1 -> 1
    printf('list 150 -> %d$N', branch_list(150));               // CHECK-NEXT: list 150 -> 1
    printf('list 250 -> %d$N', branch_list(250));               // CHECK-NEXT: list 250 -> 1
END_FUNCTION
