// RUN: %COMPILE %s && %RUN | %CHECK %s
// Nested instances are exited depth first: a member is exited together with the members below
// it, before the next member, and a whole instance before the next independent instance.
FUNCTION_BLOCK Leaf
    METHOD FB_EXIT
        printf('leaf$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Branch
    VAR
        id: DINT;
        leaf: Leaf;
    END_VAR

    METHOD FB_EXIT
        printf('branch %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Tree
    VAR
        id: DINT;
        left: Branch := (id := 1);
        right: Branch := (id := 2);
    END_VAR

    METHOD FB_EXIT
        printf('tree %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    oak: Tree := (id := 1);
    pine: Tree := (id := 2);
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: tree 2
    // CHECK-NEXT: branch 2
    // CHECK-NEXT: leaf
    // CHECK-NEXT: branch 1
    // CHECK-NEXT: leaf
    // CHECK-NEXT: tree 1
    // CHECK-NEXT: branch 2
    // CHECK-NEXT: leaf
    // CHECK-NEXT: branch 1
    // CHECK-NEXT: leaf
    // CHECK-NOT: {{tree|branch|leaf}}
END_FUNCTION
