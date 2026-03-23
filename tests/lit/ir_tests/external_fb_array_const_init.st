// RUN: %COMPILE_IR --generate-external-constructors %s | %CHECK_IR %s
//
// When an {external} function block has an array with a constant initializer,
// compiling with --generate-external-constructors should:
//   1. Produce a .const_init global for the array
//   2. Define (not declare) the ctor
//   3. Use memcpy from the .const_init in the ctor body

{external}
FUNCTION_BLOCK myFb
VAR
    arr : ARRAY[0..4] OF DINT := [1, 2, 3, 4, 5];
END_VAR
END_FUNCTION_BLOCK

PROGRAM Main
VAR
    fb : myFb;
END_VAR
END_PROGRAM

// The const_init global should exist for the array
// CHECK: @.const_init = private unnamed_addr constant [5 x i32] [i32 1, i32 2, i32 3, i32 4, i32 5]

// The ctor for myFb should be defined (has body), not just declared
// CHECK: define void @myFb__ctor(ptr %0)

// The ctor should use memcpy from the const_init
// CHECK: call void @llvm.memcpy{{.*}}@.const_init
