// RUN: %COMPILE_IR %s | %CHECK_IR %s

// Aggregate VAR_INPUT arguments must be passed by pointer; emitting
// `bitcast ptr -> [N x T]` is invalid IR. See #1718.

FUNCTION foo
   VAR_INPUT
       arr : ARRAY[1..3] OF DINT;
   END_VAR
END_FUNCTION

FUNCTION main
   foo([1, 2, 3]);
END_FUNCTION

// CHECK-LABEL: define {{.*}}@main
// CHECK:       %{{[a-zA-Z0-9_.]+}} = alloca [3 x i32]
// CHECK:       store [3 x i32]
// CHECK:       call {{.*}}@foo(ptr {{.*}})
// CHECK-NOT:   bitcast ptr {{.*}} to [
