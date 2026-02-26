// RUN: %COMPILE_IR -i %p/include/included_types.st %s | %CHECK_IR %s

PROGRAM Main
VAR
    s : IncludedStruct;
END_VAR
END_PROGRAM

// CHECK: define void @Main__ctor(ptr %0) {
// CHECK: declare void @IncludedStruct__ctor(ptr)
// CHECK-NOT: define{{.*}}@IncludedStruct__ctor
