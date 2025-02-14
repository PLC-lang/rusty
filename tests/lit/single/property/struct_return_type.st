// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE
    Transaction : STRUCT
        id : DINT;
        values : ARRAY[1..10] OF DINT;
    END_STRUCT
END_TYPE

FUNCTION print_txn
    VAR_INPUT {ref}
        value : Transaction;
    END_VAR

    VAR
        idx : DINT;
    END_VAR

    printf('{ id: %d, values: [', value.id);
    FOR idx := 1 TO 10 DO
        IF idx > 1 THEN
            printf(', ');
        END_IF;
        printf('%d', value.values[idx]);
    END_FOR;
    printf('] }$N');
END_FUNCTION

FUNCTION_BLOCK fb
    VAR
        localPrivateVariable : DINT := 5;
        returnValue : Transaction;
    END_VAR

    PROPERTY foo : Transaction
        GET
            printf('Inside getter$N');

            // Uninitialized
            IF returnValue.id = 0 THEN
                returnValue.id := 1;
                returnValue.values := [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
            END_IF

            foo := returnValue;
        END_GET

        SET
            printf('Inside setter$N');

            returnValue := foo;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK


FUNCTION main
    VAR
        instance : fb;
        localTransaction : Transaction;
    END_VAR

    // CHECK: Inside getter
    // CHECK: Inside getter
    // CHECK: { id: 1, values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512] }
    print_txn(instance.foo);

    // CHECK: Inside getter
    // CHECK: Inside setter
    // CHECK: Inside getter
    // CHECK: { id: 69, values: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512] }
    localTransaction := instance.foo;
    localTransaction.id := 69;
    instance.foo := localTransaction;
    print_txn(instance.foo);
END_FUNCTION