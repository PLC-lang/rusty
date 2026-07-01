// Entry point: run the CFC program and print the no-input function's result.
FUNCTION main : DINT
    Offset();
    printf('offset = %d$N', Offset.result);
END_FUNCTION
