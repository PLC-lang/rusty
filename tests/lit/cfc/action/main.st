// Entry point: run the CFC program, which fires the action and copies the mutated
// internal state to `out`. Observing 5 (not the default 0) proves the action ran.
FUNCTION main : DINT
    Runner();
    printf('state = %d$N', Runner.out);
END_FUNCTION
