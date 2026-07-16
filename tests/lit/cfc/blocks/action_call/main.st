FUNCTION main : DINT
    action_call();
    action_call();
    // CHECK: count = 6
    printf('count = %d$N', action_call.result);
END_FUNCTION
