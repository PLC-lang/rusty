// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb
    VAR
        returnValue: ARRAY[1..5] OF STRING := ['a', 'b', 'c', 'd', 'e'];
    END_VAR

    PROPERTY_GET foo: ARRAY[1..5] OF STRING
        foo := returnValue;
    END_GET

    PROPERTY_GET bar: DINT
        bar := 5;
    END_GET
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
