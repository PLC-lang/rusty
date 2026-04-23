// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Runtime check that trailing defaults fill correctly when combined with mixed
// positional/named args. The resolver's "skip named slots" rule for positionals
// must leave genuinely-omitted trailing slots to pick up the declared default.
//
// Related gap pinned in `mixed_implicit_explicit_non_trailing_default_is_rejected`:
// non-trailing defaults (e.g. `a, b := 1, c`) are NOT accepted. IEC 61131-3
// semantics would suggest they should, but RuSTy rejects them today.

FUNCTION myfunc : DINT
VAR_INPUT
    a : DINT;
    b : DINT := 1000;
    c : DINT := 100;
END_VAR
    myfunc := a * 10000 + b * 10 + c;
END_FUNCTION

FUNCTION main
VAR
    r1, r2, r3, r4 : DINT;
END_VAR
    // one positional, rest defaulted
    r1 := myfunc(5);                 // a=5  b=1000 c=100 -> 60100

    // positional + named, c defaulted
    r2 := myfunc(5, b := 2);         // a=5  b=2    c=100 -> 50120

    // named + positional, c defaulted (positional 2 fills b via skip-named-slots)
    r3 := myfunc(a := 5, 2);         // a=5  b=2    c=100 -> 50120

    // all supplied, defaults overridden
    r4 := myfunc(5, 2, 3);           // a=5  b=2    c=3   -> 50023

    printf('r1=%d$N', r1); // CHECK: r1=60100
    printf('r2=%d$N', r2); // CHECK: r2=50120
    printf('r3=%d$N', r3); // CHECK: r3=50120
    printf('r4=%d$N', r4); // CHECK: r4=50023
END_FUNCTION
