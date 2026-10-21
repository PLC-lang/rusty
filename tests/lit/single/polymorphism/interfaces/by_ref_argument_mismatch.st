// RUN: rm -f %t.out
// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK --implicit-check-not='{{.*}}__FATPOINTER{{.*}}' %s
// RUN: test ! -e %t.out
// Scenario: a by-reference interface parameter binds the caller's fat pointer, so its argument must
// be a variable of exactly that interface. Implementing instances, child interfaces and unrelated
// interfaces are rejected, for positional and named arguments and through interface dispatch.

INTERFACE IA
    METHOD foo : DINT
    END_METHOD
END_INTERFACE

INTERFACE IB EXTENDS IA
    METHOD bar : DINT
    END_METHOD
END_INTERFACE

INTERFACE IC
    METHOD baz : DINT
    END_METHOD
END_INTERFACE

INTERFACE IM
    METHOD m : DINT
        VAR_IN_OUT
            x : IA;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo : DINT
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION f_inout : DINT
    VAR_IN_OUT
        x : IA;
    END_VAR
END_FUNCTION

FUNCTION f_out : DINT
    VAR_OUTPUT
        x : IA;
    END_VAR
END_FUNCTION

FUNCTION f_ref : DINT
    VAR_INPUT
        x : REFERENCE TO IA;
    END_VAR
END_FUNCTION

FUNCTION main : DINT
    VAR
        a : FbA;
        ib : IB;
        ic : IC;
        im : IM;
    END_VAR
    // CHECK-DAG: {{.*}}by_ref_argument_mismatch.st:[[@LINE+1]]:13:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    f_inout(a);
    // CHECK-DAG: {{.*}}by_ref_argument_mismatch.st:[[@LINE+1]]:18:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    f_inout(x := a);
    // CHECK-DAG: {{.*}}by_ref_argument_mismatch.st:[[@LINE+1]]:16:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    f_out(x => a);
    // CHECK-DAG: {{.*}}by_ref_argument_mismatch.st:[[@LINE+1]]:11:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    f_ref(a);
    // CHECK-DAG: {{.*}}by_ref_argument_mismatch.st:[[@LINE+1]]:10:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    im.m(a);
    // CHECK-DAG: {{.*}}by_ref_argument_mismatch.st:[[@LINE+1]]:13:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'IB'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    f_inout(ib);
    // CHECK-DAG: {{.*}}by_ref_argument_mismatch.st:[[@LINE+1]]:13:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'IC'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    f_inout(ic);
END_FUNCTION
// CHECK: error: Compilation aborted due to critical errors
