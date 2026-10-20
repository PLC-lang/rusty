// A source and a sink naming undeclared variables. Each unresolved reference is reported at the
// sink block; the source has no execution order and is located at the sink it feeds.
// CHECK: {{.*}}unresolved_reference.cfc.unresolved_reference:1: error[E048]: Could not resolve reference to foo
// CHECK: {{.*}}unresolved_reference.cfc.unresolved_reference:1: error[E048]: Could not resolve reference to asd
// CHECK: error: Compilation aborted due to critical errors
FUNCTION main : DINT
END_FUNCTION
