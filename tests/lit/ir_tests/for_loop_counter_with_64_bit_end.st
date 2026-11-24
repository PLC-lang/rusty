// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// A FOR loop uses its counter, step and end expressions in more than one place. Each use must be
// resolved on its own: the counter that is copied to the LINT temporary `__next_0` is loaded as an
// i32 and widened, the next value is added and compared in i64, and the store back to the counter
// truncates it to i32.

FUNCTION main : DINT
VAR
    ud : UDINT;
    end_l : LINT := 5;
    n : DINT;
END_VAR
    FOR ud := 1 TO end_l DO
        n := n + 1;
    END_FOR
END_FUNCTION

// CHECK-LABEL: define i32 @main()
// CHECK: %__next_0 = alloca i64
// CHECK: %[[COUNTER:.*]] = load i32, ptr %ud
// CHECK: %[[WIDE:.*]] = zext i32 %[[COUNTER]] to i64
// CHECK: store {{(volatile )?}}i64 %[[WIDE]], ptr %__next_0
// CHECK: add i64 %{{.*}}, 1
// CHECK: icmp sgt i64 %{{.*}}, %{{.*}}
// CHECK: %[[NEXT:.*]] = trunc i64 %{{.*}} to i32
// CHECK: store {{(volatile )?}}i32 %[[NEXT]], ptr %ud
