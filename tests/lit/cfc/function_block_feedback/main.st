// Entry point: run the network three cycles and observe the feedback loop integrating
// (each cycle feeds the previous cycle's out1 back into in1, so y counts up).
FUNCTION main : DINT
    mainProgram();
    // CHECK: y = 1
    printf('y = %d$N', mainProgram.y);

    mainProgram();
    // CHECK: y = 2
    printf('y = %d$N', mainProgram.y);

    mainProgram();
    // CHECK: y = 3
    printf('y = %d$N', mainProgram.y);
END_FUNCTION
