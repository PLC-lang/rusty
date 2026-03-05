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

    // ✅ Works: property accessed through declaring interface
    refA := instance;
    printf('refA.value = %d$N', refA.value);

    refA.value := 99;
    printf('refA.value after set = %d$N', refA.value);

    // ✅ Works: method on IB
    refB := instance;
    refB.describe();

    // ❌ Fails: property accessed through child interface
    printf('refB.value = %d$N', refB.value);

    refB.value := 42;
    printf('refB.value after set = %d$N', refB.value);
END_FUNCTION
