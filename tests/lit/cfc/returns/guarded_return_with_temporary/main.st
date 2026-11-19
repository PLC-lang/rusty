FUNCTION_BLOCK Guard
    METHOD FB_EXIT
        printf('exit$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main: DINT
    guarded_return_with_temporary.guard := FALSE;
    guarded_return_with_temporary.result := 0;
    guarded_return_with_temporary();
    // CHECK: exit
    // CHECK-NEXT: result = 42
    printf('result = %d$N', guarded_return_with_temporary.result);

    guarded_return_with_temporary.guard := TRUE;
    guarded_return_with_temporary.result := 7;
    guarded_return_with_temporary();
    // CHECK-NEXT: exit
    // CHECK-NEXT: result = 7
    printf('result = %d$N', guarded_return_with_temporary.result);
    // CHECK-NOT: exit
END_FUNCTION
