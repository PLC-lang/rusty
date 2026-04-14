// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests AND_THEN / OR_ELSE keywords under the default (codesys) profile.
// AND_THEN and OR_ELSE always short-circuit, even when AND/OR do not.

VAR_GLOBAL
    call_count : DINT;
END_VAR

FUNCTION track : BOOL
VAR_INPUT
    val : BOOL;
END_VAR
    call_count := call_count + 1;
    track := val;
END_FUNCTION

FUNCTION main
    VAR
        result : BOOL;
    END_VAR

    // --- AND_THEN short-circuits when left is FALSE ---
    call_count := 0;
    result := track(FALSE) AND_THEN track(TRUE);
    // CHECK: and_then_false: result=FALSE calls=1
    printf('and_then_false: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- AND_THEN evaluates right when left is TRUE ---
    call_count := 0;
    result := track(TRUE) AND_THEN track(TRUE);
    // CHECK: and_then_true: result=TRUE calls=2
    printf('and_then_true: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- AND_THEN returns FALSE when right is FALSE ---
    call_count := 0;
    result := track(TRUE) AND_THEN track(FALSE);
    // CHECK: and_then_right_false: result=FALSE calls=2
    printf('and_then_right_false: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- OR_ELSE short-circuits when left is TRUE ---
    call_count := 0;
    result := track(TRUE) OR_ELSE track(FALSE);
    // CHECK: or_else_true: result=TRUE calls=1
    printf('or_else_true: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- OR_ELSE evaluates right when left is FALSE ---
    call_count := 0;
    result := track(FALSE) OR_ELSE track(TRUE);
    // CHECK: or_else_false: result=TRUE calls=2
    printf('or_else_false: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- OR_ELSE returns FALSE when both are FALSE ---
    call_count := 0;
    result := track(FALSE) OR_ELSE track(FALSE);
    // CHECK: or_else_both_false: result=FALSE calls=2
    printf('or_else_both_false: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- Contrast: AND does NOT short-circuit (codesys default) ---
    call_count := 0;
    result := track(FALSE) AND track(TRUE);
    // CHECK: and_eager: result=FALSE calls=2
    printf('and_eager: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- Contrast: OR does NOT short-circuit (codesys default) ---
    call_count := 0;
    result := track(TRUE) OR track(FALSE);
    // CHECK: or_eager: result=TRUE calls=2
    printf('or_eager: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);
END_FUNCTION
