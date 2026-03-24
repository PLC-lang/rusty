// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// SHR on unsigned BYTE must use logical shift (zero-fill), not arithmetic shift
FUNCTION main
VAR
    b : BYTE;
    s : SINT;
END_VAR
b := SHR(BYTE#251, SINT#1);

// CHECK: 125
printf('%d$N', b);

s := SHR(SINT#-4, SINT#1);

// CHECK: -2
printf('%d$N', s);
END_FUNCTION
