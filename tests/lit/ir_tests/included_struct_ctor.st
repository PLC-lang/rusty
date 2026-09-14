// RUN: %COMPILE_IR -i %p/include/included_types.st %s | %CHECK_IR %s

PROGRAM Main
VAR
    s : IncludedStruct;
END_VAR
END_PROGRAM

// The declaration may precede or follow Main__ctor depending on the unit order
// CHECK-NOT: define{{.*}}@IncludedStruct__ctor
// CHECK-DAG: define void @Main__ctor(ptr %0) {
// CHECK-DAG: call void @IncludedStruct__ctor(ptr %s)
// CHECK-DAG: declare void @IncludedStruct__ctor(ptr)
// CHECK-NOT: define{{.*}}@IncludedStruct__ctor
