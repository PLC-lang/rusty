// RUN: not %COMPILE %s --error-format clang 2>&1 | %CHECK %s
//
// A parse error is reported once, in the clang format. The abort that follows does not repeat it.

FUNCTION foo : BOOL
VAR_INPUT
    in : BOOL;
    // CHECK: {{.*}}reported_once_clang.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}: error[E006]: Missing expected Token KeywordColon or KeywordComma
    in2
    // CHECK-NEXT: {{.*}}reported_once_clang.st:[[@LINE+4]]:1:{{.*}}: error[E006]: Missing expected Token KeywordColon
    // CHECK-NEXT: {{.*}}reported_once_clang.st:[[@LINE+3]]:1:{{.*}}: error[E007]: Unexpected token: expected DataTypeDefinition but found KeywordEndVar
    // CHECK-NEXT: {{.*}}reported_once_clang.st:[[@LINE+2]]:1:{{.*}}: error[E006]: Missing expected Token [KeywordSemicolon]
    // CHECK-NEXT: {{.*}}reported_once_clang.st:[[@LINE+1]]:1:{{.*}}: error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_VAR'
END_VAR
;
END_FUNCTION

// CHECK-NEXT: error: Compilation aborted due to critical parse errors
// CHECK-NOT: {{.*}} at: {{.*}}
