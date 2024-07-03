// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// CHECK: 1
// CHECK: 5
// CHECK: whats up
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
    printf('%d$N', refTxn.id);
    printf('%d$N', refTxn.amount);
    printf('%s$N', REF(refTxn.message));
END_FUNCTION