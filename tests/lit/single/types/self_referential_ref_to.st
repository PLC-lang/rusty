// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test self-referential struct broken by REF_TO
TYPE TreeNode : STRUCT
    data : DINT;
    left : REF_TO TreeNode;
    right : REF_TO TreeNode;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        root : TreeNode;
        leftChild : TreeNode;
        rightChild : TreeNode;
    END_VAR

    root.data := 50;
    leftChild.data := 25;
    rightChild.data := 75;

    // Build tree structure
    root.left := REF(leftChild);
    root.right := REF(rightChild);

    // CHECK: Root: 50
    printf('Root: %d$N', root.data);

    // CHECK: Left: 25
    printf('Left: %d$N', root.left^.data);

    // CHECK: Right: 75
    printf('Right: %d$N', root.right^.data);

    main := 0;
END_FUNCTION
