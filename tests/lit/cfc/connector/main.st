// Entry point: `src` is routed to `out` through a connector/continuation pair named
// "x" (a named virtual wire), so out == src.
FUNCTION main : DINT
    Wire.src := 42;
    Wire();
    printf('out = %d$N', Wire.out);
END_FUNCTION
