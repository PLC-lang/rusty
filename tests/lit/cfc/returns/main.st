// Entry point: exercise conditional, unconditional, and negated CFC returns.
//   Guard:    out := 1; IF skip THEN RETURN; END_IF; out := 2;
//   Always:   out := 1; RETURN; out := 2;   (the second assignment is dead)
//   GuardNot: out := 1; IF NOT keep THEN RETURN; END_IF; out := 2;  (negated condition)
FUNCTION main : DINT
    // Conditional return taken -> stops at out := 1
    Guard.skip := TRUE;
    Guard();
    printf('guard_true = %d$N', Guard.out);

    // Conditional return not taken -> falls through to out := 2
    Guard.skip := FALSE;
    Guard();
    printf('guard_false = %d$N', Guard.out);

    // Unconditional return -> always stops at out := 1
    Always();
    printf('always = %d$N', Always.out);

    // Negated conditional return -> fires when the condition is FALSE.
    // keep=TRUE  => NOT keep is FALSE => return skipped => out := 2
    GuardNot.keep := TRUE;
    GuardNot();
    printf('guard_not_keep = %d$N', GuardNot.out);

    // keep=FALSE => NOT keep is TRUE => return fires => stops at out := 1
    GuardNot.keep := FALSE;
    GuardNot();
    printf('guard_not_drop = %d$N', GuardNot.out);
END_FUNCTION
