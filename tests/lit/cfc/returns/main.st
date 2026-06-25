// Entry point: exercise conditional and unconditional CFC returns.
//   Guard:  out := 1; IF skip THEN RETURN; END_IF; out := 2;
//   Always: out := 1; RETURN; out := 2;   (the second assignment is dead)
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
END_FUNCTION
