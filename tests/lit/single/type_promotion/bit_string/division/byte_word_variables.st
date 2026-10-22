// RUN: %COMPILE %s && %RUN | %CHECK %s
// Two BYTE or WORD operands keep their own width, and their division and MOD are unsigned.
FUNCTION main : DINT
    VAR
        b : BYTE := 16#FF;
        b2 : BYTE := 16#02;
        b7 : BYTE := 16#07;
        w : WORD := 16#FFFF;
        w2 : WORD := 16#0002;
        w7 : WORD := 16#0007;
        rb : BYTE;
        rw : WORD;
    END_VAR

    rb := b / b2;
    printf('BYTE 16#FF / 2 = %u$N', rb);                    // CHECK: BYTE 16#FF / 2 = 127
    rb := b MOD b7;
    printf('BYTE 16#FF MOD 7 = %u$N', rb);                  // CHECK-NEXT: BYTE 16#FF MOD 7 = 3
    rw := w / w2;
    printf('WORD 16#FFFF / 2 = %u$N', rw);                  // CHECK-NEXT: WORD 16#FFFF / 2 = 32767
    rw := w MOD w7;
    printf('WORD 16#FFFF MOD 7 = %u$N', rw);                // CHECK-NEXT: WORD 16#FFFF MOD 7 = 1
END_FUNCTION
