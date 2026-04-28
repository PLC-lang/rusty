// RUN: rm -f %T/%basename_t.out
// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// RUN: test ! -e %T/%basename_t.out

// Regression test: diagnostics emitted by lowering participants must not produce a binary.
// Lowerers aggregate diagnostics and let the main validation step decide whether to abort,
// so validation errors found during lowering must still stop the pipeline before codegen.

// CHECK: {{.*}}direct_interface_call_does_not_emit_output.st:{{[0-9]+}}:{{[0-9]+}}:{{.*}}error[E129]: Interfaces cannot be called directly
// CHECK: Compilation aborted due to critical errors

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION main
    VAR
        refIA : IA;
    END_VAR

    refIA();
END_FUNCTION
