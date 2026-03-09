// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Ensure STRING return values are passed correctly through interface dispatch.

INTERFACE A
    METHOD return_string: STRING
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    VAR
        callCounter: DINT;
    END_VAR

    METHOD return_string: STRING
        callCounter := callCounter + 1;

        IF callCounter = 1 THEN
            return_string := 'FbA::return_string, first';
        ELSE
            return_string := 'FbA::return_string, second';
        END_IF
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: FbA;
        reference: A;
        result: STRING;
    END_VAR

    reference := instance;

    // CHECK: main::result_first = FbA::return_string, first
    result := reference.return_string();
    printf('main::result_first = %s$N', result);

    // CHECK-NEXT: main::result_second = FbA::return_string, second
    result := reference.return_string();
    printf('main::result_second = %s$N', result);
END_FUNCTION
