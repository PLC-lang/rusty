// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Container : STRUCT
    data : ARRAY[0..2] OF STRING;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        container : Container := (data := ['one', 'two', 'three']);
    END_VAR

    // CHECK: one
    printf('%s$N', container.data[0]);

    // CHECK: two
    printf('%s$N', container.data[1]);

    // CHECK: three
    printf('%s$N', container.data[2]);
END_FUNCTION
