FUNCTION main : DINT
    // The cycle points the reference at its source.
    storage_reference.a := 5;
    storage_reference();
    // CHECK: ref: b = 5
    printf('ref: b = %d$N', storage_reference.b);

    // The reference reads through to the source's current value.
    storage_reference.a := 7;
    // CHECK: track: b = 7
    printf('track: b = %d$N', storage_reference.b);
END_FUNCTION
