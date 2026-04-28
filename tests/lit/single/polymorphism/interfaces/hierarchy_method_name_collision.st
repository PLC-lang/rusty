// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Two unrelated interfaces define a method with the same name and signature. A single function
// block implements both. Dispatching through each interface variable should resolve to the same underlying
// method, but the vtable routing for each interface must be correct independently.

INTERFACE IA
    METHOD describe : STRING
    END_METHOD

    METHOD onlyA
    END_METHOD
END_INTERFACE

INTERFACE IB
    METHOD describe : STRING
    END_METHOD

    METHOD onlyB
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbBoth IMPLEMENTS IA, IB
    METHOD describe : STRING
        describe := 'FbBoth::describe';
    END_METHOD

    METHOD onlyA
        printf('FbBoth::onlyA$N');
    END_METHOD

    METHOD onlyB
        printf('FbBoth::onlyB$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbOnlyA IMPLEMENTS IA
    METHOD describe : STRING
        describe := 'FbOnlyA::describe';
    END_METHOD

    METHOD onlyA
        printf('FbOnlyA::onlyA$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbOnlyB IMPLEMENTS IB
    METHOD describe : STRING
        describe := 'FbOnlyB::describe';
    END_METHOD

    METHOD onlyB
        printf('FbOnlyB::onlyB$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        both: FbBoth;
        onlyA: FbOnlyA;
        onlyB: FbOnlyB;

        refA: IA;
        refB: IB;
    END_VAR

    // Dispatch describe() through IA — should hit FbBoth's single implementation
    // CHECK: FbBoth::describe
    refA := both;
    printf('%s$N', refA.describe());

    // Dispatch describe() through IB — same implementation, different vtable route
    // CHECK-NEXT: FbBoth::describe
    refB := both;
    printf('%s$N', refB.describe());

    // Interface-specific methods still work
    // CHECK-NEXT: FbBoth::onlyA
    refA.onlyA();

    // CHECK-NEXT: FbBoth::onlyB
    refB.onlyB();

    // Now assign different concrete types to each interface
    // CHECK-NEXT: FbOnlyA::describe
    refA := onlyA;
    printf('%s$N', refA.describe());

    // CHECK-NEXT: FbOnlyB::describe
    refB := onlyB;
    printf('%s$N', refB.describe());

    // CHECK-NEXT: FbOnlyA::onlyA
    refA.onlyA();

    // CHECK-NEXT: FbOnlyB::onlyB
    refB.onlyB();
END_FUNCTION
