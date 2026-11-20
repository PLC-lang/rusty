// RUN: not %COMPILE %s 2>&1 | %CHECK %s
//
// A parse error is reported once, in the rich format. The abort that follows does not repeat it
// in a "Details:" block. The rich reporter always writes color codes, so the error line allows them.

FUNCTION foo : BOOL
VAR_INPUT
    in : BOOL;
    in2
END_VAR
;
END_FUNCTION

// CHECK: {{.*}}error[E006]{{.*}}: Missing expected Token KeywordColon or KeywordComma{{.*}}
// CHECK: error: Compilation aborted due to critical parse errors.
// CHECK-NEXT: Hint: You can use `plc explain <ErrorCode>` for more information
