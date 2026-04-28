// RUN: (%COMPILE -Onone %s && %RUN) | %CHECK %s
// Verifies that a boolean XOR expression involving a direct bit-access is
// correctly promoted to i32 when passed as a variadic argument.
// Without the promotion the i1 result can be clobbered by the subsequent MUX
// call, causing the same expression to print different values before and after.

FUNCTION foo
    VAR
        vTrue  : BOOL := TRUE;
        vFalse : BOOL := FALSE;
        arrLword : LWORD := 16#abcdefabcdefabcd;
        // bit 27 of arrLword = 1, so:
        //   bit27 XOR TRUE  = 0
        //   bit27 XOR FALSE = 1
    END_VAR

    // Simplified bit-access via %x27 (same as %d0.%w1.%B1.%x3)
    printf('before mux xor TRUE: %u$N', arrLword.%x27 XOR vTrue);   // CHECK: before mux xor TRUE: 0
    MUX(0, 255, 255);
    printf('after mux xor TRUE: %u$N',  arrLword.%x27 XOR vTrue);   // CHECK: after mux xor TRUE: 0

    printf('before mux xor FALSE: %u$N', arrLword.%x27 XOR vFalse); // CHECK: before mux xor FALSE: 1
    MUX(0, 255, 255);
    printf('after mux xor FALSE: %u$N',  arrLword.%x27 XOR vFalse); // CHECK: after mux xor FALSE: 1

    // Chain-access form from the original bug report
    printf('chain before mux: %u$N', arrLword.%d0.%w1.%B1.%x3 XOR vTrue);  // CHECK: chain before mux: 0
    MUX(0, 255, 255);
    printf('chain after mux: %u$N',  arrLword.%d0.%w1.%B1.%x3 XOR vTrue);  // CHECK: chain after mux: 0

END_FUNCTION

FUNCTION main : DINT
    foo();
END_FUNCTION
