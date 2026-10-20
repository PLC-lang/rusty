FUNCTION main : DINT
    expression_pin();
    expression_pin();
    // CHECK: a = 4, b = 44
    printf('a = %d, b = %d$N', expression_pin.outA, expression_pin.outB);
END_FUNCTION
