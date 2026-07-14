FUNCTION main : DINT
    program_call();
    program_call();
    // CHECK: total = 6, result = 12
    printf('total = %d, result = %d$N', program_call.runningTotal, program_call.result);
END_FUNCTION
