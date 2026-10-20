// RUN: %COMPILE %s && %RUN | %CHECK %s
// A multiplied struct element in a global array literal is expanded into its repeated struct
// initializers, the same as in a local array. A member that does not fit its type is reported
// (E039, silenced here) and wraps; the compiler used to recurse on the raw element until its
// stack overflowed.

TYPE S : STRUCT
    si : SINT;
    d : DINT;
END_STRUCT END_TYPE
TYPE S1 : STRUCT
    a : DINT;
END_STRUCT END_TYPE
TYPE S2 : STRUCT
    a : DINT;
    b : DINT;
END_STRUCT END_TYPE

VAR_GLOBAL CONSTANT
    cs1 : S1 := (a := 11);
    ca : S2 := (a := 1);
    cb : S2 := (b := 2);
END_VAR

VAR_GLOBAL
    g : ARRAY[0..2] OF S := [3((si := 1, d := 2))];
    h : ARRAY[0..1] OF S := [2((si := 300))];
    // each entry of the list is one struct, also a one-member struct a constant folds to; a
    // multiplied element may hold several structs
    k : ARRAY[0..1] OF S1 := [cs1, cs1];
    m : ARRAY[0..3] OF S := [2((si := 1), (si := 2))];
    // each struct constant in a multiplied element stays one struct, also when it sets only
    // some of its members
    n : ARRAY[0..3] OF S2 := [2(ca, cb)];
END_VAR

FUNCTION main : DINT
    printf('%d %d %d %d$N', g[0].si, g[1].d, g[2].si, g[2].d);  // CHECK: 1 2 1 2
    printf('%d %d$N', h[0].si, h[1].si);  // CHECK: 44 44
    printf('%d %d$N', k[0].a, k[1].a);  // CHECK: 11 11
    printf('%d %d %d %d$N', m[0].si, m[1].si, m[2].si, m[3].si);  // CHECK: 1 2 1 2
    printf('%d %d %d %d %d %d$N', n[0].a, n[0].b, n[1].a, n[1].b, n[2].a, n[3].b);  // CHECK: 1 0 0 2 1 2
END_FUNCTION
