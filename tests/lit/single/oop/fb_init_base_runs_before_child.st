// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Sequencing: `Base.FB_INIT` must run before `Child.FB_INIT`, so that the
// child's body can rely on initialized base state. Pre-fix, the doubled base
// ctor would still produce the right value here, but the inherited-FB_INIT
// re-emit ran the base FB_INIT after derived members were touched, which
// could clobber values written by the child. We pin the post-fix order.
FUNCTION_BLOCK Base
VAR
    x : DINT := 0;
END_VAR
    METHOD FB_INIT
        x := 10;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Child EXTENDS Base
VAR
    y : DINT := 0;
END_VAR
    METHOD FB_INIT
        // Base.FB_INIT must have already run, so x == 10 here.
        y := x * 2;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    c : Child;
END_VAR
    printf('%d$N', c.x); // CHECK: 10
    printf('%d$N', c.y); // CHECK: 20
END_FUNCTION
