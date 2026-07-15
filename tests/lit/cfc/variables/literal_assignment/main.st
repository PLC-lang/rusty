FUNCTION main : DINT
    literal_assignment();
    // CHECK: x = 42
    printf('x = %d$N', literal_assignment.x);
END_FUNCTION
