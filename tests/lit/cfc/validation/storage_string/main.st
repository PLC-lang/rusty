// A storage-mode latch stores a BOOL constant, so a sink without a numeric
// engraving is rejected by the main pipeline's validation of the generated
// code; no CFC-specific validation is involved.
// CHECK: {{.*}}error[E037]: Invalid assignment: cannot assign 'BOOL' to 'STRING'
// CHECK: error: Compilation aborted due to critical errors
FUNCTION main : DINT
END_FUNCTION
