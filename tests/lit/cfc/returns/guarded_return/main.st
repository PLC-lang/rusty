FUNCTION main : DINT
    guarded_return.guard := FALSE;
    guarded_return.result := 0;
    guarded_return();
    // CHECK: result = 42
    printf('result = %d$N', guarded_return.result);

    guarded_return.guard := TRUE;
    guarded_return.result := 7;
    guarded_return();
    // CHECK: result = 7
    printf('result = %d$N', guarded_return.result);
END_FUNCTION
