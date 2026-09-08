// A negated inout pin transpiles like an input pin, but an inout passes by
// reference and a NOT expression has no storage behind it, so validation
// rejects the call.
// CHECK: {{.*}}function_inout_negated.cfc.function_inout_negated:0:1: error[E031]: Expected a reference for parameter acc because their type is InOut
// CHECK: error: Compilation aborted due to critical errors
FUNCTION main : DINT
END_FUNCTION
