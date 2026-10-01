FUNCTION main : DINT
    negative_source.t_inc := 10;
    negative_source.rc := 2;
    negative_source();
    // CHECK: quotient = -5, negated = -10, literal = -5
    printf('quotient = %d, negated = %d, literal = %d$N', negative_source.quotient, negative_source.negated, negative_source.literal);
END_FUNCTION
