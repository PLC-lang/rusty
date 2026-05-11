// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression: when both Base and Child declare `FB_INIT`, each must run
// exactly once during construction. Pre-fix the doubled base-ctor (issue 1)
// and the inherited FB_INIT re-emit (issue 2) inflated these counts.
FUNCTION_BLOCK Base
VAR
    base_calls : DINT := 0;
END_VAR
    METHOD FB_INIT
        base_calls := base_calls + 1;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Child EXTENDS Base
VAR
    child_calls : DINT := 0;
END_VAR
    METHOD FB_INIT
        child_calls := child_calls + 1;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    c : Child;
END_VAR
    printf('%d$N', c.base_calls);  // CHECK: 1
    printf('%d$N', c.child_calls); // CHECK: 1
END_FUNCTION
