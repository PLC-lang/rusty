// EN and ENO exist only at the call site: nothing is injected into the callee,
// so a POU body referencing ENO fails to resolve like any unknown identifier.
// CHECK: {{.*}}error[E048]: Could not resolve reference to ENO
// CHECK: error: Compilation aborted due to critical errors
FUNCTION main : DINT
END_FUNCTION
