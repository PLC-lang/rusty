// RUN: (%COMPILE %s --profile standard && %RUN) | %CHECK %s
// Tests AND_THEN / OR_ELSE under the standard profile.
// Under standard, AND/OR already short-circuit, so AND_THEN/OR_ELSE behave identically.

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

    // --- AND_THEN short-circuits (same as AND under standard) ---
    call_count := 0;
    result := track(FALSE) AND_THEN track(TRUE);
    // CHECK: and_then: result=FALSE calls=1
    printf('and_then: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- AND also short-circuits under standard ---
    call_count := 0;
    result := track(FALSE) AND track(TRUE);
    // CHECK: and: result=FALSE calls=1
    printf('and: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- OR_ELSE short-circuits (same as OR under standard) ---
    call_count := 0;
    result := track(TRUE) OR_ELSE track(FALSE);
    // CHECK: or_else: result=TRUE calls=1
    printf('or_else: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);

    // --- OR also short-circuits under standard ---
    call_count := 0;
    result := track(TRUE) OR track(FALSE);
    // CHECK: or: result=TRUE calls=1
    printf('or: result=%s calls=%d$N', SEL(result, 'FALSE', 'TRUE'), call_count);
END_FUNCTION
