FUNCTION main : DINT
    // The ST compiler accepts BOOL stores into numerics, so a numeric sink
    // latches the literal 1 (Set) or 0 (Reset). Preset both to prove the
    // FALSE input writes nothing.
    storage_numeric.ds := 40;
    storage_numeric.dr := 42;
    storage_numeric.a := FALSE;
    storage_numeric();
    // CHECK: idle: ds = 40, dr = 42
    printf('idle: ds = %d, dr = %d$N', storage_numeric.ds, storage_numeric.dr);

    // A TRUE input engraves the constants.
    storage_numeric.a := TRUE;
    storage_numeric();
    // CHECK: latched: ds = 1, dr = 0
    printf('latched: ds = %d, dr = %d$N', storage_numeric.ds, storage_numeric.dr);
END_FUNCTION
