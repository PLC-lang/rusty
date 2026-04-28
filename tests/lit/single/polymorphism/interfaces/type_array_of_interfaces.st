// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An interface hierarchy (IA <- IB <- IC) with function blocks implementing each level.
// An array of the root interface IA is populated with all concrete types — direct and indirect
// implementors — and dispatched through, testing that fat pointers in array slots handle inheritance
// correctly.

INTERFACE IA
    METHOD id : DINT
    END_METHOD
END_INTERFACE

INTERFACE IB EXTENDS IA
    METHOD label : STRING
    END_METHOD
END_INTERFACE

INTERFACE IC EXTENDS IB
    METHOD depth : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK Fb1 IMPLEMENTS IA
    METHOD id : DINT
        id := 1;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Fb2 IMPLEMENTS IB
    METHOD id : DINT
        id := 2;
    END_METHOD

    METHOD label : STRING
        label := 'Fb2';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Fb3 IMPLEMENTS IC
    METHOD id : DINT
        id := 3;
    END_METHOD

    METHOD label : STRING
        label := 'Fb3';
    END_METHOD

    METHOD depth : DINT
        depth := 3;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        inst1: Fb1;
        inst2: Fb2;
        inst3: Fb3;

        arrA: ARRAY[1..5] OF IA;
        arrB: ARRAY[1..2] OF IB;
        i: DINT;
    END_VAR

    // All three FBs are assignable to IA (direct or indirect implementation)
    arrA[1] := inst1;
    arrA[2] := inst2;
    arrA[3] := inst3;
    arrA[4] := inst2;
    arrA[5] := inst1;

    // Iterate root interface array — each element dispatches id() correctly
    // CHECK: id=1
    // CHECK-NEXT: id=2
    // CHECK-NEXT: id=3
    // CHECK-NEXT: id=2
    // CHECK-NEXT: id=1
    FOR i := 1 TO 5 DO
        printf('id=%d$N', arrA[i].id());
    END_FOR;

    // Fb2 and Fb3 are assignable to IB (direct or indirect)
    arrB[1] := inst2;
    arrB[2] := inst3;

    // Dispatch through IB array — both id() and label() work
    // CHECK-NEXT: id=2, label=Fb2
    // CHECK-NEXT: id=3, label=Fb3
    FOR i := 1 TO 2 DO
        printf('id=%d, label=%s$N', arrB[i].id(), arrB[i].label());
    END_FOR;

    // Overwrite root array element and verify dispatch changes
    // CHECK-NEXT: id=3
    arrA[1] := inst3;
    printf('id=%d$N', arrA[1].id());
END_FUNCTION
