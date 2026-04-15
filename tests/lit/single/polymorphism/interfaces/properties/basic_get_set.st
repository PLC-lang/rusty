// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Single interface with a property declaring both PROPERTY_GET and PROPERTY_SET, implemented by a single
// function block. Assign the concrete instance to an interface variable and exercise both accessors
// through it. Baseline sanity test for property dispatch through interfaces.

INTERFACE IValue
    PROPERTY value : DINT
        PROPERTY_GET END_GET
        PROPERTY_SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbStore IMPLEMENTS IValue
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY value : DINT
        PROPERTY_GET
            value := _val;
        END_GET

        PROPERTY_SET
            _val := value;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance : FbStore;
        iface : IValue;
    END_VAR

    iface := instance;

    // CHECK: value=10
    printf('value=%d$N', iface.value);

    iface.value := 42;

    // CHECK-NEXT: value=42
    printf('value=%d$N', iface.value);

    iface.value := iface.value + 8;

    // CHECK-NEXT: value=50
    printf('value=%d$N', iface.value);
END_FUNCTION
