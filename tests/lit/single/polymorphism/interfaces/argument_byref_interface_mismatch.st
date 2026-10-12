// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// Scenario: A VAR_OUTPUT, VAR_IN_OUT or REFERENCE TO interface parameter is storage the callee writes
// through, so an interface-typed argument must be the exact parameter interface. A child or parent
// interface variable is rejected instead of being upcast into a temporary that would never reach the
// caller, or downcast into storage the callee cannot fill.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

INTERFACE IB EXTENDS IA
    METHOD bar
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbB IMPLEMENTS IB
    METHOD foo
    END_METHOD

    METHOD bar
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION consumerIA
    VAR_OUTPUT
        out1 : IA;
    END_VAR
    VAR_IN_OUT
        inout1 : IA;
    END_VAR
END_FUNCTION

FUNCTION consumerIB
    VAR_IN_OUT
        inout1 : IB;
    END_VAR
END_FUNCTION

FUNCTION consumerRef
    VAR_INPUT
        ref1 : REFERENCE TO IA;
    END_VAR
END_FUNCTION

FUNCTION_BLOCK FbConsumer
    VAR_OUTPUT
        out1 : IA;
    END_VAR
    VAR_IN_OUT
        inout1 : IA;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbConsumerChild EXTENDS FbConsumer
    VAR_INPUT
        in1 : DINT;
    END_VAR
END_FUNCTION_BLOCK

PROGRAM progConsumer
    VAR_OUTPUT
        out1 : IA;
    END_VAR
END_PROGRAM

FUNCTION main
    VAR
        instance : FbB;
        refIA : IA;
        refIB : IB;
        fbConsumer : FbConsumer;
        fbConsumerChild : FbConsumerChild;
    END_VAR

    refIB := instance;
    refIA := instance;

    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
    consumerIA(out1 => refIB, inout1 := refIB);

    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
    consumerIA(refIB, refIB);

    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IB but got IA
    consumerIB(refIA);

    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter ref1 is passed by reference, it requires an argument of type IA but got IB
    consumerRef(refIB);

    // Function block and program outputs are plain members rather than pointers, but the call still
    // writes through them, so they take the exact interface as well.
    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
    fbConsumer(out1 => refIB, inout1 := refIB);

    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
    fbConsumer(refIB, refIB);

    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
    progConsumer(out1 => refIB);

    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
    progConsumer(refIB);

    // Parameters inherited from a parent function block are named after the declaring parameter.
    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
    // CHECK-DAG: {{.*}}argument_byref_interface_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
    fbConsumerChild(refIB, refIB, 1);

    // CHECK: {{.*}}Compilation aborted due to critical errors
END_FUNCTION
