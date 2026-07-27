// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// Structural pin for the PRG-4521 fix: constant expressions inside nested
// array initializers are const-folded, so the initializer lowers to a folded
// constant global copied via memcpy. Previously the raw expression tree
// reached codegen's literal generator and failed with "Cannot generate
// Literal for BinaryExpression". Runtime coverage lives in
// tests/lit/single/init/array_const_expression_init.st.

PROGRAM mainProg
VAR
    mArr : ARRAY[0..1] OF ARRAY[0..1] OF UDINT := [[UDINT#0 + UDINT#1, UDINT#0], [UDINT#0, UDINT#2 - UDINT#1]];
END_VAR
    ;
END_PROGRAM

// The type's __init global is fully folded to [1, 0], [0, 1].
// CHECK: @__mainProg.mArr__init = unnamed_addr constant [2 x [2 x i32]] {{\[}}[2 x i32] [i32 1, i32 0], [2 x i32] [i32 0, i32 1]]

// The ctor assignment is folded as well and copied via memcpy.
// CHECK: @.const_init = private unnamed_addr constant [2 x [2 x i32]] {{\[}}[2 x i32] [i32 1, i32 0], [2 x i32] [i32 0, i32 1]]
// CHECK: define void @mainProg__ctor(ptr %0)
// CHECK: call void @llvm.memcpy{{.*}}@.const_init
