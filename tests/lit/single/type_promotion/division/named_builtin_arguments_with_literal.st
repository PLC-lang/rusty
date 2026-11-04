// RUN: %COMPILE %s 2> %t.err && %RUN | %CHECK %s
// RUN: not grep -E "Implicit conversion from|Operand of type" %t.err
// An untyped integer literal passed by name to a builtin operator takes the type of the other argument, as
// it does next to the operator, so the operation on a UDINT stays unsigned.
FUNCTION main : DINT
    VAR
        a : UDINT := 4000000000;
        q : UDINT;
    END_VAR

    q := DIV(IN1 := a, IN2 := 2);
    printf('DIV(IN1 := 4e9, IN2 := 2) = %u$N', q);          // CHECK: DIV(IN1 := 4e9, IN2 := 2) = 2000000000
    q := DIV(IN2 := 2, IN1 := a);
    printf('DIV(IN2 := 2, IN1 := 4e9) = %u$N', q);          // CHECK-NEXT: DIV(IN2 := 2, IN1 := 4e9) = 2000000000
    q := MOD(IN1 := a, IN2 := 7);
    printf('MOD(IN1 := 4e9, IN2 := 7) = %u$N', q);          // CHECK-NEXT: MOD(IN1 := 4e9, IN2 := 7) = 3
    q := SUB(IN1 := a, IN2 := 1);
    printf('SUB(IN1 := 4e9, IN2 := 1) = %u$N', q);          // CHECK-NEXT: SUB(IN1 := 4e9, IN2 := 1) = 3999999999
    q := a / 2;
    printf('4e9 / 2 = %u$N', q);                            // CHECK-NEXT: 4e9 / 2 = 2000000000
END_FUNCTION
