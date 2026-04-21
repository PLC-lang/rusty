// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: IB extends IA, where IA declares a property and IB adds a method. FbImpl implements
// IB with both, FbChild extends FbImpl and overrides only the setter (adds 500). Dispatch through
// both IA and IB variables should resolve the property identically, while the method is only
// accessible through IB.

INTERFACE IA
    PROPERTY value : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

INTERFACE IB EXTENDS IA
    METHOD describe END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbImpl IMPLEMENTS IB
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY value : DINT
        GET
            value := _val;
        END_GET

        SET
            _val := value;
        END_SET
    END_PROPERTY

    METHOD describe
        printf('FbImpl(%d)$N', _val);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbChild EXTENDS FbImpl
    // Override only setter — adds 500
    PROPERTY value : DINT
        SET
            _val := value + 500;
        END_SET
    END_PROPERTY

    METHOD describe
        printf('FbChild(%d)$N', _val);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        implForA : FbImpl;
        implForB : FbImpl;
        childForA : FbChild;
        childForB : FbChild;
        refA : IA;
        refB : IB;
    END_VAR

    // Through IA — property only
    refA := implForA;
    printf('value=%d$N', refA.value);           // CHECK: value=10
    refA.value := 42;
    printf('value=%d$N', refA.value);           // CHECK-NEXT: value=42

    refA := childForA;
    printf('value=%d$N', refA.value);           // CHECK-NEXT: value=10
    refA.value := 42;
    printf('value=%d$N', refA.value);           // CHECK-NEXT: value=542

    // Through IB — property + method
    refB := implForB;
    refB.describe();                            // CHECK-NEXT: FbImpl(10)
    printf('value=%d$N', refB.value);           // CHECK-NEXT: value=10
    refB.value := 42;
    refB.describe();                            // CHECK-NEXT: FbImpl(42)

    refB := childForB;
    refB.describe();                            // CHECK-NEXT: FbChild(10)
    printf('value=%d$N', refB.value);           // CHECK-NEXT: value=10
    refB.value := 42;
    refB.describe();                            // CHECK-NEXT: FbChild(542)
    printf('value=%d$N', refB.value);           // CHECK-NEXT: value=542
END_FUNCTION
