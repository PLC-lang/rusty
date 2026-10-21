// RUN: %COMPILE %s && %RUN | %CHECK %s
// Range labels that stay below or above the signed midpoint of a bit string selector.
FUNCTION branch_byte : DINT
    VAR_INPUT
        x : BYTE;
    END_VAR

    CASE x OF
        16#01..16#64: branch_byte := 1;
        16#81..16#FE: branch_byte := 2;
        16#FF: branch_byte := 3;
    ELSE
        branch_byte := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_dword : DINT
    VAR_INPUT
        x : DWORD;
    END_VAR

    CASE x OF
        16#0000_0001..16#0000_FFFF: branch_dword := 1;
        16#8000_0000..16#FFFF_FFFE: branch_dword := 2;
        16#FFFF_FFFF: branch_dword := 3;
    ELSE
        branch_dword := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    // BYTE
    printf('BYTE 16#32 -> %d$N', branch_byte(16#32));                   // CHECK: BYTE 16#32 -> 1
    printf('BYTE 16#C8 -> %d$N', branch_byte(16#C8));                   // CHECK-NEXT: BYTE 16#C8 -> 2
    printf('BYTE 16#FF -> %d$N', branch_byte(16#FF));                   // CHECK-NEXT: BYTE 16#FF -> 3

    // DWORD
    printf('DWORD 16#10 -> %d$N', branch_dword(16#10));                 // CHECK-NEXT: DWORD 16#10 -> 1
    printf('DWORD 16#8000_0001 -> %d$N', branch_dword(16#8000_0001));   // CHECK-NEXT: DWORD 16#8000_0001 -> 2
    printf('DWORD 16#FFFF_FFFF -> %d$N', branch_dword(16#FFFF_FFFF));   // CHECK-NEXT: DWORD 16#FFFF_FFFF -> 3
END_FUNCTION
