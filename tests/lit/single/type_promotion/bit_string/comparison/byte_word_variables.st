// RUN: %COMPILE %s && %RUN | %CHECK %s
// Two BYTE or WORD operands keep their own width and compare unsigned, so a value from the signed midpoint on compares as large.
FUNCTION main : DINT
    VAR
        bhi : BYTE := 16#FF;
        blo : BYTE := 16#01;
        bmid : BYTE := 16#80;
        bbelow : BYTE := 16#7F;
        whi : WORD := 16#FFFF;
        wlo : WORD := 16#0001;
        wmid : WORD := 16#8000;
        wbelow : WORD := 16#7FFF;
    END_VAR

    // BYTE
    printf('BYTE hi > lo = %d$N', bhi > blo);               // CHECK: BYTE hi > lo = 1
    printf('BYTE lo < hi = %d$N', blo < bhi);               // CHECK-NEXT: BYTE lo < hi = 1
    printf('BYTE hi <= lo = %d$N', bhi <= blo);             // CHECK-NEXT: BYTE hi <= lo = 0
    printf('BYTE mid > below = %d$N', bmid > bbelow);       // CHECK-NEXT: BYTE mid > below = 1
    printf('BYTE hi > BYTE#1 = %d$N', bhi > BYTE#1);        // CHECK-NEXT: BYTE hi > BYTE#1 = 1

    // WORD
    printf('WORD hi > lo = %d$N', whi > wlo);               // CHECK-NEXT: WORD hi > lo = 1
    printf('WORD lo < hi = %d$N', wlo < whi);               // CHECK-NEXT: WORD lo < hi = 1
    printf('WORD hi <= lo = %d$N', whi <= wlo);             // CHECK-NEXT: WORD hi <= lo = 0
    printf('WORD mid > below = %d$N', wmid > wbelow);       // CHECK-NEXT: WORD mid > below = 1
    printf('WORD hi > WORD#1 = %d$N', whi > WORD#1);        // CHECK-NEXT: WORD hi > WORD#1 = 1
END_FUNCTION
