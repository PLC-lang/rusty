// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// Pin the IR shape that the #1576 fix (commit 2b3388f) put in place: when
// calling an action that inherits a `VAR_IN_OUT` from its parent FB, the
// caller must (a) GEP to the FB's in-out slot and (b) store the local's
// address there before invoking the action. Skipping either step leaves the
// pointer NULL and the action segfaults at runtime — runtime coverage lives
// in `tests/lit/single/actions/action_call_with_var_in_out.st`; this test
// is the structural guardrail.

PROGRAM mainProg
VAR
    fb : fb_t;
    localBool : BOOL;
END_VAR
    fb.foo(myInOut := localBool);
END_PROGRAM

FUNCTION_BLOCK fb_t
VAR_IN_OUT
    myInOut : BOOL;
END_VAR
END_FUNCTION_BLOCK

ACTIONS
    ACTION foo
        myInOut := myInOut;
    END_ACTION
END_ACTIONS

// CHECK-LABEL: define void @mainProg(
// CHECK: %fb = getelementptr inbounds nuw %mainProg, {{.*}}, i32 0, i32 0
// CHECK: %localBool = getelementptr inbounds nuw %mainProg, {{.*}}, i32 0, i32 1
// CHECK: getelementptr inbounds %fb_t, ptr %fb, i32 0, i32 1
// CHECK-NEXT: store ptr %localBool, ptr {{%[0-9]+}}, align 8
// CHECK-NEXT: call void @fb_t__foo(ptr %fb)
