FUNCTION main : DINT
    // Preset the sink; a FALSE input must not touch it.
    storage_reset.b := TRUE;
    storage_reset.a := FALSE;
    storage_reset();
    // CHECK: idle: b = 1
    printf('idle: b = %d$N', storage_reset.b);

    // A TRUE input resets the sink to FALSE.
    storage_reset.a := TRUE;
    storage_reset();
    // CHECK: reset: b = 0
    printf('reset: b = %d$N', storage_reset.b);

    // The latch holds after the input drops.
    storage_reset.a := FALSE;
    storage_reset();
    // CHECK: hold: b = 0
    printf('hold: b = %d$N', storage_reset.b);
END_FUNCTION
