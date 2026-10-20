FUNCTION main : DINT
    method_call();
    method_call();
    // CHECK: result = 6, doubled = 12
    printf('result = %d, doubled = %d$N', method_call.result, method_call.dbl);
END_FUNCTION
