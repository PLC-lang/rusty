FUNCTION main : DINT
    // The negated return fires on a false guard, skipping the assignment.
    negated_return.guard := FALSE;
    negated_return.result := 0;
    negated_return();
    // CHECK: result = 0
    printf('result = %d$N', negated_return.result);

    // A true guard does not return early, so the assignment runs.
    negated_return.guard := TRUE;
    negated_return.result := 0;
    negated_return();
    // CHECK: result = 42
    printf('result = %d$N', negated_return.result);
END_FUNCTION
