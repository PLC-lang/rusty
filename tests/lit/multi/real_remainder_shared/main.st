FUNCTION main : DINT
    printf('REAL: %.2f$N', REAL_TO_LREAL(remainderReal(7.5, 2.0)));
    printf('LREAL: %.2f$N', remainderLreal(-7.5, 2.0));
END_FUNCTION
