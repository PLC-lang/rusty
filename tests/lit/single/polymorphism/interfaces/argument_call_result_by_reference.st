// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// Scenario: A call result is not a variable, so it cannot be passed to a by-reference interface
// parameter, exactly like for any other by-reference parameter. The lowerer must not hide the call
// result behind a generated fat pointer that would point at the result's temporary.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Consumer
    VAR_INPUT
        in1 : REFERENCE TO IA;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION producer : FbA
END_FUNCTION

FUNCTION producerArray : ARRAY[0..1] OF FbA
END_FUNCTION

FUNCTION consumerInOut
    VAR_IN_OUT
        inout1 : IA;
    END_VAR
END_FUNCTION

VAR_GLOBAL
    globalConsumer : Consumer;
END_VAR

FUNCTION main
    // CHECK-DAG: {{.*}}argument_call_result_by_reference.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E031]: Expected a reference for parameter in1 because their type is REFERENCE TO
    globalConsumer(producer());

    // CHECK-DAG: {{.*}}argument_call_result_by_reference.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E031]: Expected a reference for parameter inout1 because their type is InOut
    consumerInOut(producer());

    // An element of a call result lives in the result's temporary as well.
    // CHECK-DAG: {{.*}}argument_call_result_by_reference.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E031]: Expected a reference for parameter in1 because their type is REFERENCE TO
    globalConsumer(producerArray()[0]);

    // CHECK: {{.*}}Compilation aborted due to critical errors
END_FUNCTION
