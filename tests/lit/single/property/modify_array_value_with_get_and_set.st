// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb
    VAR
        localArray: ARRAY[1..5] OF DINT := [1, 2, 3, 4, 5];
    END_VAR

    PROPERTY foo: ARRAY[1..5] OF DINT
        GET
            foo := localArray;
        END_GET

        SET
            localArray := foo;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        localArray: ARRAY[1..5] OF DINT;
        fbInstance: fb;
    END_VAR

    localArray := fbInstance.foo;
    localArray[1] := 5;
    localArray[5] := 1;

    fbInstance.foo := localArray;

    printf('%d$N', fbInstance.foo[1]); // CHECK: 5
    printf('%d$N', fbInstance.foo[5]); // CHECK: 1
END_FUNCTION
