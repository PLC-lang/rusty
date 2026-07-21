FUNCTION main : DINT
    function_feedback.seed := 5;
    function_feedback();
    function_feedback();
    // return feeds its own in1: cycle 1 => 0 + 5, cycle 2 => 5 + 5
    // CHECK: acc = 10
    printf('acc = %d$N', function_feedback.acc);
END_FUNCTION
