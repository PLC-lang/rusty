// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Node : STRUCT
    data : DINT;
    next : REFERENCE TO Node;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        node1 : Node;
        node2 : Node;
    END_VAR

    node1.data := 42;
    node2.data := 84;

    // Create circular reference
    node1.next REF= node2;
    node2.next REF= node1;

    // CHECK: Node1 data: 42
    printf('Node1 data: %d$N', node1.data);

    // CHECK: Node2 data: 84
    printf('Node2 data: %d$N', node2.data);

    // CHECK: Node1->next->data: 84
    printf('Node1->next->data: %d$N', node1.next.data);

    // CHECK: Node2->next->data: 42
    printf('Node2->next->data: %d$N', node2.next.data);

    // CHECK: Circular reference works: 42
    printf('Circular reference works: %d$N', node1.next.next.data);

    main := 0;
END_FUNCTION
