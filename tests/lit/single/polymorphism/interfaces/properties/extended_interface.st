// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: IB extends IA. IA defines a property, IB adds a method. A function block implements
// IB. Verifies property dispatch works through both the parent interface (IA) and the child
// interface (IB) that inherits the property.

INTERFACE IA
    PROPERTY value : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

INTERFACE IB EXTENDS IA
    METHOD describe END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IB
    VAR
        _val : DINT := 50;
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
        printf('FbA val=%d$N', _val);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance : FbA;
        refA : IA;
        refB : IB;
    END_VAR

    // Access through parent interface IA
    refA := instance;
    printf('%d$N', refA.value);         // CHECK: 50

    refA.value := 99;
    printf('%d$N', refA.value);         // CHECK: 99

    // Access through child interface IB (inherited property)
    refB := instance;
    printf('%d$N', refB.value);         // CHECK: 99

    refB.value := 42;
    printf('%d$N', refB.value);         // CHECK: 42

    // Verify both views see the same state
    printf('%d$N', refA.value);         // CHECK: 42

    // Method on IB still works
    refB.describe();                    // CHECK: FbA val=42
END_FUNCTION
