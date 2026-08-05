FUNCTION main : DINT
    generic_feedback.seed := 5;
    generic_feedback();
    generic_feedback();
    generic_feedback();
    // CHECK: acc = 15
    printf('acc = %d$N', generic_feedback.acc);
END_FUNCTION
