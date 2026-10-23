// RUN: %COMPILE %s && %RUN | %CHECK %s
// A BYTE and a WORD operand keep the wider bit string type and compare unsigned, so a value from the signed midpoint on compares as large.
FUNCTION main : DINT
    VAR
        b5 : BYTE := 16#05;
        b2 : BYTE := 16#02;
        wmax : WORD := 16#FFFF;
    END_VAR

    printf('BYTE 5 < WORD max = %d$N', b5 < wmax);          // CHECK: BYTE 5 < WORD max = 1
    printf('BYTE 5 > WORD max = %d$N', b5 > wmax);          // CHECK-NEXT: BYTE 5 > WORD max = 0
    printf('WORD max > BYTE 2 = %d$N', wmax > b2);          // CHECK-NEXT: WORD max > BYTE 2 = 1
    printf('WORD max < BYTE 2 = %d$N', wmax < b2);          // CHECK-NEXT: WORD max < BYTE 2 = 0
END_FUNCTION
