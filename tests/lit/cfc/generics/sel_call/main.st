FUNCTION main : DINT
    sel_call.cond := FALSE;
    sel_call.in0 := 10;
    sel_call.in1 := 20;
    sel_call();
    // CHECK: first = 10
    printf('first = %d$N', sel_call.result);

    sel_call.cond := TRUE;
    sel_call();
    // CHECK: second = 20
    printf('second = %d$N', sel_call.result);
END_FUNCTION
