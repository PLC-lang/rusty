// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test various integer types work correctly
TYPE ByteEnum : BYTE (a := 1, b := 2);
END_TYPE

TYPE IntEnum : INT (x := 100, y := 200);
END_TYPE

TYPE DwordEnum : DWORD (big := 16#FFFF0000, small := 16#00000000);
END_TYPE

TYPE LintEnum : LINT (huge := 9223372036854775807, semi_huge := 123421341234);  // Max LINT
END_TYPE

PROGRAM prog
VAR
    be : ByteEnum := b;
    ie : IntEnum := y;
    de : DwordEnum := big;
    le : LintEnum := huge;
END_VAR
    // CHECK: 2
    printf('%d$N', be);  // BYTE

    // CHECK: 200
    printf('%d$N', ie);  // INT

    // CHECK: -65536
    printf('%d$N', de);  // DWORD (as signed)

    // CHECK: 9223372036854775807
    printf('%lld$N', le);  // LINT
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
