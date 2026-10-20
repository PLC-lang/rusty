// Sources and sinks naming undeclared variables, also nested in a global access and in an array
// literal. Each unresolved reference is reported at its sink block; a source has no execution
// order and is located at the sink it feeds.
// CHECK: {{.*}}unresolved_reference.cfc.unresolved_reference:1: error[E048]: Could not resolve reference to foo
// CHECK: {{.*}}unresolved_reference.cfc.unresolved_reference:1: error[E048]: Could not resolve reference to asd
// CHECK: {{.*}}unresolved_reference.cfc.unresolved_reference:2: error[E048]: Could not resolve reference to global
// CHECK: {{.*}}unresolved_reference.cfc.unresolved_reference:3: error[E048]: Could not resolve reference to element
// CHECK: error: Compilation aborted due to critical errors
FUNCTION main : DINT
END_FUNCTION
