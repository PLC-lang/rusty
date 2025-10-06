// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD compute : INT
        VAR_INPUT
            value: INT;
        END_VAR
        
        compute := value * 2;
        printf('A::compute(%d) = %d$N', value, compute);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD compute : INT
        VAR_INPUT
            value: INT;
        END_VAR
        
        compute := value * 3;
        printf('B::compute(%d) = %d$N', value, compute);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION processValue : INT
    VAR_INPUT
        obj: POINTER TO A;
        input: INT;
    END_VAR
    
    processValue := obj^.compute(input);
END_FUNCTION

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        objRef: POINTER TO A;
        result: INT;
    END_VAR

    // ~~ Direct polymorphic calls ~~
    // CHECK: A::compute(5) = 10
    // CHECK: B::compute(5) = 15
    
    objRef := ADR(instanceA);
    result := objRef^.compute(5);
    
    objRef := ADR(instanceB);
    result := objRef^.compute(5);

    // ~~ Polymorphic calls through function ~~
    // CHECK: A::compute(7) = 14
    // CHECK: B::compute(7) = 21
    
    objRef := ADR(instanceA);
    result := processValue(objRef, 7);
    
    objRef := ADR(instanceB);
    result := processValue(objRef, 7);
END_FUNCTION