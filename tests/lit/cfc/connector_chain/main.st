// Entry point: `src` is routed to `out` through two chained connector/continuation hops
// (x then y), so out == src after resolving the chain transitively.
FUNCTION main : DINT
    Chain.src := 99;
    Chain();
    printf('out = %d$N', Chain.out);
END_FUNCTION
