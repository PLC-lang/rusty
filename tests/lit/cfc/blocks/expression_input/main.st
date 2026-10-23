FUNCTION main : DINT
    expression_input();
    expression_input();
    // CHECK: a = 6, b = 20, sum = 26
    printf('a = %d, b = %d, sum = %d$N', expression_input.outA, expression_input.outB, expression_input.sum);
END_FUNCTION
