FUNCTION main : DINT
    // Project directory name mixes spaces, parentheses and brackets -- all legal
    // on Windows and Linux. Exercises both plc's path resolution (brackets) and
    // the test shell's quoting (spaces, parens).
    printf('weird=%d$N', 7);
    // CHECK: weird=7
    main := 0;
END_FUNCTION
