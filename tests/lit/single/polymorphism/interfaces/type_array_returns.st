// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Ensure ARRAY[1..5] OF DINT return values are passed correctly through interface dispatch.

TYPE DintArray5: ARRAY[1..5] OF DINT; END_TYPE

INTERFACE A
    METHOD return_array: DintArray5
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    VAR
        callCounter: DINT;
    END_VAR

    METHOD return_array: DintArray5
        callCounter := callCounter + 1;

        return_array[1] := callCounter * 10 + 1;
        return_array[2] := callCounter * 10 + 2;
        return_array[3] := callCounter * 10 + 3;
        return_array[4] := callCounter * 10 + 4;
        return_array[5] := callCounter * 10 + 5;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: FbA;
        reference: A;
        result: DintArray5;
    END_VAR

    reference := instance;

    // CHECK: main::result_first = [11, 12, 13, 14, 15]
    result := reference.return_array();
    printf('main::result_first = [%d, %d, %d, %d, %d]$N',
        result[1],
        result[2],
        result[3],
        result[4],
        result[5]
    );

    // CHECK-NEXT: main::result_second = [21, 22, 23, 24, 25]
    result := reference.return_array();
    printf('main::result_second = [%d, %d, %d, %d, %d]$N',
        result[1],
        result[2],
        result[3],
        result[4],
        result[5]
    );
END_FUNCTION
