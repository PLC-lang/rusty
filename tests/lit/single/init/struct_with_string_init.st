// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE Transaction : STRUCT
    id      : DINT;
    amount  : DINT;
    message : STRING;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        txn     : Transaction := (id := 1, amount := 5, message := 'whats up');
    END_VAR

    // CHECK: 1
    printf('%d$N', txn.id);

    // CHECK: 5
    printf('%d$N', txn.amount);

    // CHECK: whats up
    printf('%s$N', txn.message);
END_FUNCTION
