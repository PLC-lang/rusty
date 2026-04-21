// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb
    VAR
        returnValue: ARRAY[1..5] OF STRING := ['a', 'b', 'c', 'd', 'e'];
    END_VAR

    PROPERTY foo: ARRAY[1..5] OF STRING
        GET
            foo := returnValue;
        END_GET
    END_PROPERTY

    PROPERTY bar: DINT
        GET
            bar := 5;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fbInstance: fb2;
    END_VAR

    printf('%s$N', REF(fbInstance.foo[1])); // CHECK: a
    printf('%d$N', fbInstance.bar); // CHECK: 5
END_FUNCTION