// Calls C23 variadic functions that declare no named parameter and read their arguments with the
// single-argument `va_start`. A PLC declaration whose only parameter is `args : ...` produces the
// matching `declare void @f(...)`, and the implicit terminator ends the list in both cases.

FUNCTION main : DINT
VAR
    a : STRING := 'alpha';
    b : STRING := 'beta';
    count : DINT;
END_VAR
    // The terminator is the only argument this call carries.
    OnlyVariadic();
    // CHECK: onlyvariadic:

    OnlyVariadic(a, b);
    // CHECK: onlyvariadic: alpha beta

    count := OnlyVariadicCount();
    printf('before terminator: %d$N', count);
    // CHECK: before terminator: 0

    count := OnlyVariadicCount(a, b);
    printf('before terminator: %d$N', count);
    // CHECK: before terminator: 2
END_FUNCTION
