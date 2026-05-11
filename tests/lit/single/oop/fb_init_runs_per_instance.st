// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Sanity: the fix for double base-ctor / inherited FB_INIT must not
// over-correct. Each constructed instance must still run the inherited
// `FB_INIT` once. Two instances => count of 2.
VAR_GLOBAL
    base_init_count : DINT := 0;
END_VAR

FUNCTION_BLOCK Base
    METHOD FB_INIT
        base_init_count := base_init_count + 1;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Child EXTENDS Base
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    a : Child;
    b : Child;
END_VAR
    printf('%d$N', base_init_count); // CHECK: 2
END_FUNCTION
