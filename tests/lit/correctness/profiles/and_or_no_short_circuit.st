// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that AND/OR evaluate both sides under the default (codesys) profile.

VAR_GLOBAL
    call_count : DINT;
END_VAR

FUNCTION side_effect : BOOL
VAR_INPUT
    val : BOOL;
END_VAR
    call_count := call_count + 1;
    side_effect := val;
END_FUNCTION

FUNCTION main
    // AND: left is FALSE, right should still be evaluated (no short-circuit)
    call_count := 0;
    IF side_effect(FALSE) AND side_effect(TRUE) THEN
        printf('unreachable$N');
    END_IF;
    // CHECK: and_both_called=2
    printf('and_both_called=%d$N', call_count);

    // OR: left is TRUE, right should still be evaluated (no short-circuit)
    call_count := 0;
    IF side_effect(TRUE) OR side_effect(FALSE) THEN
        printf('or_branch_taken$N');
    END_IF;
    // CHECK: or_branch_taken
    // CHECK: or_both_called=2
    printf('or_both_called=%d$N', call_count);

    // AND_THEN: left is FALSE, right should NOT be evaluated (short-circuit)
    call_count := 0;
    IF side_effect(FALSE) AND_THEN side_effect(TRUE) THEN
        printf('unreachable$N');
    END_IF;
    // CHECK: and_then_short_circuit=1
    printf('and_then_short_circuit=%d$N', call_count);

    // OR_ELSE: left is TRUE, right should NOT be evaluated (short-circuit)
    call_count := 0;
    IF side_effect(TRUE) OR_ELSE side_effect(FALSE) THEN
        printf('or_else_branch_taken$N');
    END_IF;
    // CHECK: or_else_branch_taken
    // CHECK: or_else_short_circuit=1
    printf('or_else_short_circuit=%d$N', call_count);
END_FUNCTION
