// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE Transaction : STRUCT
    id      : DINT;
    amount  : DINT;
    message : STRING;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        txn     : Transaction := (id := 1, amount := 5, message := 'whats up');
        refTxn  : REFERENCE TO Transaction;
    END_VAR

    refTxn REF= txn;

    // CHECK: 1
    printf('%d$N', refTxn.id);

    // CHECK: 5
    printf('%d$N', refTxn.amount);

    // CHECK: whats up
    printf('%s$N', REF(refTxn.message));
END_FUNCTION