// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// A project that declares no `FB_EXIT` gets no destructors and no entry in `llvm.global_dtors`.

FUNCTION_BLOCK Plain
    VAR
        value: DINT;
    END_VAR

    METHOD FB_INIT
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM prog
    VAR
        plain: Plain;
    END_VAR
END_PROGRAM

FUNCTION work: DINT
    VAR
        plain: Plain;
    END_VAR
END_FUNCTION

// CHECK: @llvm.global_ctors
// CHECK-NOT: llvm.global_dtors
// CHECK-NOT: __dtor
