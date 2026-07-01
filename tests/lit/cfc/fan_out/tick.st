// A side-effecting producer: each call bumps the global `calls` and returns its input.
// Used to prove a fanned-out block output is evaluated exactly once.
VAR_GLOBAL
    calls : DINT;
END_VAR

FUNCTION Tick : DINT
VAR_INPUT
    seed : DINT;
END_VAR
    calls := calls + 1;
    Tick := seed;
END_FUNCTION
