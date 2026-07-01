// Entry point: drive the CFC function block and print its output.
FUNCTION main : DINT
VAR
    c : Computer;
END_VAR
    c(a := 20, b := 22);
    printf('result = %d$N', c.result);
END_FUNCTION
