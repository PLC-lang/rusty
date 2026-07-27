FUNCTION main : DINT
    // The jump is not taken, so the assignment runs.
    guarded_jump.cond := FALSE;
    guarded_jump.x := 42;
    guarded_jump.y := 0;
    guarded_jump();
    // CHECK: y = 42
    printf('y = %d$N', guarded_jump.y);

    // The jump is taken, skipping the assignment; y keeps its prior value.
    guarded_jump.cond := TRUE;
    guarded_jump.x := 99;
    guarded_jump.y := 7;
    guarded_jump();
    // CHECK: y = 7
    printf('y = %d$N', guarded_jump.y);
END_FUNCTION
