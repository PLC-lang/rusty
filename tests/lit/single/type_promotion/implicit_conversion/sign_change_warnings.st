// RUN: %PLC --check --error-format clang %s 2>&1 | %CHECK %s
// Implicit conversions that change the sign are reported where lowering has already rewritten the call.
FUNCTION_BLOCK Adder
    METHOD plus : UDINT
        VAR_INPUT amount : UDINT; END_VAR
        plus := amount;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Stepper
    VAR_INPUT step : UDINT; END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Producer
    VAR_OUTPUT out : DINT; END_VAR
END_FUNCTION_BLOCK

FUNCTION takesUdint : UDINT
    VAR_INPUT x : UDINT; END_VAR
    takesUdint := x;
END_FUNCTION

FUNCTION returnsUdint : UDINT
    VAR dint_ : DINT; END_VAR
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    returnsUdint := dint_;
END_FUNCTION

FUNCTION main : DINT
    VAR
        adder : Adder;
        stepper : Stepper;
        producer : Producer;
        vDint : DINT;
        vUdint : UDINT;
        vUlint : ULINT;
        vLint : LINT;
        vByte : BYTE;
        vSint : SINT;
    END_VAR

    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    vUdint := vDint;
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'LINT' to 'ULINT' may change the sign.
    vUlint := vLint;
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'SINT' to 'BYTE' may change the sign.
    vByte := vSint;
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    vUdint := takesUdint(vDint);
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    vUdint := adder.plus(vDint);
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    stepper(step := vDint);
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    producer(out => vUdint);
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    vUdint := vUdint + vDint;
    // CHECK: {{.*}}:[[@LINE+1]]:{{[0-9]+}}:{{.*}}warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    IF vDint < vUdint THEN
        vDint := 0;
    END_IF
END_FUNCTION
