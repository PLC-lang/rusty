FUNCTION main : DINT
    function_inout();
    function_inout();
    // CHECK: total = 10, result = 10
    printf('total = %d, result = %d$N', function_inout.total, function_inout.result);
END_FUNCTION
