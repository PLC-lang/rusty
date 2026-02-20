// RUN: %IR_COMPILE -i %p/include/included_types.st %s 2>/dev/null | %CHECK %s

PROGRAM Main
VAR
    s : IncludedStruct;
END_VAR
END_PROGRAM

// CHECK: declare void @IncludedStruct__ctor(ptr)
// CHECK-NOT: define{{.*}}@IncludedStruct__ctor
// CHECK: define void @Main__ctor(ptr %0) {
