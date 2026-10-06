FUNCTION main : DINT
VAR
    x : DINT := 2;
    r : REFERENCE TO DINT;
END_VAR
    r REF= x;
    r REF= increment(r);
    printf('%d$N', r);
    printf('%d$N', increment_twice());
END_FUNCTION
