// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test self-referential struct broken by POINTER TO
TYPE LinkedList : STRUCT
    value : DINT;
    next : POINTER TO LinkedList;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        list1 : LinkedList;
        list2 : LinkedList;
    END_VAR

    list1.value := 10;
    list2.value := 20;

    // Link them together
    list1.next := ADR(list2);
    list2.next := ADR(list1);

    // CHECK: List1 value: 10
    printf('List1 value: %d$N', list1.value);

    // CHECK: List2 value: 20
    printf('List2 value: %d$N', list2.value);

    main := 0;
END_FUNCTION
