FUNCTION main : DINT
    // Nothing happens while the input is FALSE; b keeps its initial FALSE.
    storage_set.a := FALSE;
    storage_set();
    // CHECK: idle: b = 0
    printf('idle: b = %d$N', storage_set.b);

    // A TRUE input latches the sink to TRUE.
    storage_set.a := TRUE;
    storage_set();
    // CHECK: set: b = 1
    printf('set: b = %d$N', storage_set.b);

    // The latch holds after the input drops.
    storage_set.a := FALSE;
    storage_set();
    // CHECK: hold: b = 1
    printf('hold: b = %d$N', storage_set.b);
END_FUNCTION
