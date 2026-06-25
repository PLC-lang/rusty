// Entry point: seed the accumulator's in-out `sum`, run it once, and observe that the
// by-reference pin mutated `sum` in place (5 -> 5+10 = 15), and the return matches.
FUNCTION main : DINT
    Accumulator.value := 10;
    Accumulator.sum := 5;
    Accumulator();
    printf('sum = %d$N', Accumulator.sum);
    printf('result = %d$N', Accumulator.result);
END_FUNCTION
