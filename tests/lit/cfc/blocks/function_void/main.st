FUNCTION main : DINT
    function_void();
    // CHECK: result = 6
    printf('result = %d$N', function_void.localOut);
END_FUNCTION
