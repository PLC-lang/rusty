// Entry point: run the splitter and read both of its outputs (lo := n, hi := n + 100).
FUNCTION main : DINT
    Split.n := 7;
    Split();
    printf('lo = %d$N', Split.outLo);
    printf('hi = %d$N', Split.outHi);
END_FUNCTION
