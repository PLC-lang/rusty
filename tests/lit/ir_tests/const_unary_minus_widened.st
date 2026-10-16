// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// A unary minus on a constant is folded by the constant evaluator. The folded literal is typed
// by the negation, which is widened to DINT, and not by the constant it came from. Without this
// the initializer of a program member was an i8 inside an i32 slot, which LLVM rejects.

VAR_GLOBAL CONSTANT
    c1 : USINT := 200;
    ci : INT := -32768;
END_VAR

VAR_GLOBAL
    g : DINT := -c1;
    gl : LINT := -ci;
END_VAR

PROGRAM prg
    VAR
        y : DINT := -c1;
        z : DINT := -ci;
    END_VAR
END_PROGRAM

// CHECK: @g = global i32 -200
// CHECK: @gl = global i64 32768
// CHECK: @prg_instance = global %prg { i32 -200, i32 32768 }
