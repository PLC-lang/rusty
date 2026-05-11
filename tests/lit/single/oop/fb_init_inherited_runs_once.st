// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression: an inherited `FB_INIT` must only run once per construction.
// Pre-fix the derived ctor invoked it both via the base ctor chain and via a
// spurious second emit (find_method walked up the super class).
FUNCTION_BLOCK Base
VAR
    init_calls : DINT := 0;
END_VAR
    METHOD FB_INIT
        init_calls := init_calls + 1;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Child EXTENDS Base
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    c : Child;
END_VAR
    printf('%d$N', c.init_calls); // CHECK: 1
END_FUNCTION
