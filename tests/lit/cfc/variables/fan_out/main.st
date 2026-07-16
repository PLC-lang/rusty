FUNCTION main : DINT
    fan_out();
    // CHECK: x = 5, y = 5
    printf('x = %d, y = %d$N', fan_out.x, fan_out.y);
END_FUNCTION
