// Entry point: seed the program's `input`, run it once, and observe that the compound
// sink target wrote the function's result into the array element (41+1 = 42).
FUNCTION main : DINT
    mainProgram.input := 41;
    mainProgram();

    // CHECK: results[1] = 42
    printf('results[1] = %d$N', mainProgram.results[1]);
END_FUNCTION
