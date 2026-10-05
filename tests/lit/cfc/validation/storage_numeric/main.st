// A storage-mode sink on a numeric variable stores the BOOL source value into it. The generated
// `IF a THEN ds := TRUE; END_IF` and `IF a THEN dr := FALSE; END_IF` are invalid assignments,
// reported once per sink block.
// CHECK: {{.*}}storage_numeric.cfc.storage_numeric:0: error[E037]: Invalid assignment: cannot assign 'BOOL' to 'DINT'
// CHECK: {{.*}}storage_numeric.cfc.storage_numeric:1: error[E037]: Invalid assignment: cannot assign 'BOOL' to 'DINT'
// CHECK: error: Compilation aborted due to critical errors
FUNCTION main : DINT
END_FUNCTION
