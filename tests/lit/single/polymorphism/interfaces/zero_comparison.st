// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: `= 0` and `<> 0` check whether an interface is assigned, with the interface on either side.
// An unassigned interface guarded with AND_THEN is never called.

INTERFACE IA
    METHOD foo : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo : DINT
        foo := 42;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbHolder
    VAR_INPUT
        reference : IA;
    END_VAR

    METHOD isAssigned : BOOL
        isAssigned := reference <> 0;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance : FbA;
        reference : IA;
        holder : FbHolder;
    END_VAR

    // CHECK: unassigned: 1 0 1 0
    printf('unassigned: %d %d %d %d$N', reference = 0, reference <> 0, 0 = reference, holder.isAssigned());

    // CHECK-NEXT: not called
    IF reference <> 0 AND_THEN reference.foo() = 42 THEN
        printf('called$N');
    ELSE
        printf('not called$N');
    END_IF

    reference := instance;
    holder.reference := instance;

    // CHECK-NEXT: assigned: 0 1 1 1
    printf('assigned: %d %d %d %d$N', reference = 0, reference <> 0, 0 <> reference, holder.isAssigned());

    // CHECK-NEXT: called
    IF reference <> 0 AND_THEN reference.foo() = 42 THEN
        printf('called$N');
    END_IF
END_FUNCTION
