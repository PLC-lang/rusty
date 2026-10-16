// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// Scenario: The value of a REF= must be a reference. A short-circuit expression whose call needs a fat
// pointer preamble is not rewritten into a temporary there, so the validator still rejects it instead of
// letting a pointer keep the address of a BOOL temporary.

INTERFACE IA
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
END_FUNCTION_BLOCK

FUNCTION consume : BOOL
    VAR_INPUT
        in1 : IA;
    END_VAR

    consume := TRUE;
END_FUNCTION

VAR_GLOBAL
    ptr : POINTER TO BOOL;
    enabled : BOOL;
    instance : FbA;
END_VAR

FUNCTION bind
    // CHECK: {{.*}}argument_short_circuit_in_ref_assignment.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E098]: Invalid assignment, expected a reference
    ptr REF= enabled AND_THEN consume(instance);

    // CHECK: {{.*}}Compilation aborted due to critical errors
END_FUNCTION
