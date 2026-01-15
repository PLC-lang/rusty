// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// SHR must be usable with integers
FUNCTION main
VAR
    a : INT;
END_VAR
a := SHR(21,2);

// CHECK: 5
printf('%d$N', a);
END_FUNCTION
