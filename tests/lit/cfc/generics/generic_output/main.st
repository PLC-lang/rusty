FUNCTION main : DINT
    generic_output.x := 21;
    generic_output();
    // CHECK: y = 42
    printf('y = %d$N', generic_output.y);
END_FUNCTION
