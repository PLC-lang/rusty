FUNCTION main : DINT
    evaluation_order();
    // CHECK: a = 2, b = 1
    printf('a = %d, b = %d$N', evaluation_order.a, evaluation_order.b);
END_FUNCTION
