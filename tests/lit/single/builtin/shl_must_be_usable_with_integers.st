// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// SHL must be usable with integers
FUNCTION main
VAR
    a : INT;
END_VAR
a := SHL(21,2);

// CHECK: 84
printf('%d$N', a);
END_FUNCTION
