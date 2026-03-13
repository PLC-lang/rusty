// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// When an {external} function block has an array with a constant initializer,
// compiling WITHOUT --generate-external-constructors should:
//   1. NOT produce any .const_init or __init globals for the external FB
//   2. Declare (not define) the ctor

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

// The ctor for myFb should be declared (external), not defined
// CHECK: declare void @myFb__ctor(ptr)

// No .const_init or __myFb.arr__init globals should be present
// CHECK-NOT: @.const_init
// CHECK-NOT: @__myFb.arr__init
