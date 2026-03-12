// RUN: %COMPILE_IR -i %p/include/included_types.st %s | %CHECK_IR %s

PROGRAM Main
VAR
    s : IncludedStruct;
END_VAR
END_PROGRAM

// CHECK-DAG: define void @Main__ctor(ptr %0) {
// CHECK-DAG: call void @IncludedStruct__ctor(ptr %s)
// CHECK-DAG: declare void @IncludedStruct__ctor(ptr)
//
// Ensure IncludedStruct's ctor is only declared (external), not defined in this module.
// CHECK-NOT: define void @IncludedStruct__ctor
