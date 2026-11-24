// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// A FOR loop uses its counter, step and end expressions in more than one place. Each use must be
// resolved on its own: the counter in `ud + 1` must not take the LINT type that the comparison with
// the end value gives to the counter, or codegen emits an `add` of an i64 and an i32 operand.

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
// CHECK: %[[SUM:.*]] = add i32 %{{.*}}, 1
// CHECK: %[[WIDE:.*]] = zext i32 %[[SUM]] to i64
// CHECK: icmp sgt i64 %[[WIDE]], %{{.*}}
