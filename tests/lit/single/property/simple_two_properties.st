// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb
    VAR
        count : DINT;
    END_VAR

    PROPERTY getCount : DINT
        GET
            getCount := count;
        END_GET
    END_PROPERTY

    PROPERTY increaseCountBy : DINT
        SET
            count := count + increaseCountBy;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance : fb;
    END_VAR

    printf('%d$N', instance.getCount); // CHECK: 0

    instance.increaseCountBy := 5;
    printf('%d$N', instance.getCount); // CHECK: 5

    instance.increaseCountBy := 5;
    printf('%d$N', instance.getCount); // CHECK: 10

    instance.increaseCountBy := 10;
    printf('%d$N', instance.getCount); // CHECK: 20

    instance.increaseCountBy := 80;
    printf('%d$N', instance.getCount); // CHECK: 100
END_FUNCTION