// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// Scenario: A sized variadic whose element type is REFERENCE TO an interface takes neither a concrete
// instance nor an interface reference. The fat pointer is passed by value into a pointer-sized element,
// so the lowerer rejects the argument instead of generating code that overruns the argument array.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
    END_METHOD
END_FUNCTION_BLOCK

TYPE RefIA : REFERENCE TO IA; END_TYPE

FUNCTION consume : DINT
    VAR_INPUT
        rest : {sized} RefIA...;
    END_VAR

    consume := 1;
END_FUNCTION

FUNCTION main
    VAR
        instance : FbA;
        reference : IA;
        result : DINT;
    END_VAR

    // CHECK-DAG: {{.*}}argument_variadic_reference_to_element.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E159]: Parameter rest is a sized variadic of REFERENCE TO IA and cannot take an argument of type FbA
    result := consume(instance);

    // CHECK-DAG: {{.*}}argument_variadic_reference_to_element.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E159]: Parameter rest is a sized variadic of REFERENCE TO IA and cannot take an argument of type IA
    result := consume(reference);

    // CHECK: {{.*}}Compilation aborted due to critical errors
END_FUNCTION
