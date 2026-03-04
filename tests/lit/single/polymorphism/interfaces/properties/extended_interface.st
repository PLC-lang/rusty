// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: IB extends IA. IA defines a property, IB adds a method. A function block implements
// IB and can be assigned to an IA typed variable. Verifies property dispatch works through the
// parent interface when the implementing POU declared IMPLEMENTS on the child interface.

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
    END_VAR

    // Access through parent interface IA
    refA := instance;
    printf('%d$N', refA.value);         // CHECK: 50

    refA.value := 99;
    printf('%d$N', refA.value);         // CHECK: 99
END_FUNCTION
