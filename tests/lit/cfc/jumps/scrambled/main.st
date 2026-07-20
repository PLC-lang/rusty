FUNCTION main : DINT
    // No guard taken: every assignment runs.
    scrambled.g1 := FALSE; scrambled.g2 := FALSE; scrambled.g3 := FALSE;
    scrambled.x := 5;
    scrambled.a := 0; scrambled.b := 0; scrambled.c := 0;
    scrambled();
    // CHECK: a=5 b=5 c=5
    printf('a=%d b=%d c=%d$N', scrambled.a, scrambled.b, scrambled.c);

    // g1 jumps to `mid`, skipping `a := x`; b and c still run.
    scrambled.g1 := TRUE; scrambled.g2 := FALSE; scrambled.g3 := FALSE;
    scrambled.x := 9;
    scrambled.a := 1; scrambled.b := 1; scrambled.c := 1;
    scrambled();
    // CHECK: a=1 b=9 c=9
    printf('a=%d b=%d c=%d$N', scrambled.a, scrambled.b, scrambled.c);

    // g2 jumps to `end` after `a := x`, skipping b and c.
    scrambled.g1 := FALSE; scrambled.g2 := TRUE; scrambled.g3 := FALSE;
    scrambled.x := 7;
    scrambled.a := 2; scrambled.b := 2; scrambled.c := 2;
    scrambled();
    // CHECK: a=7 b=2 c=2
    printf('a=%d b=%d c=%d$N', scrambled.a, scrambled.b, scrambled.c);

    // g3 jumps to `end` after b, skipping only c.
    scrambled.g1 := FALSE; scrambled.g2 := FALSE; scrambled.g3 := TRUE;
    scrambled.x := 3;
    scrambled.a := 4; scrambled.b := 4; scrambled.c := 4;
    scrambled();
    // CHECK: a=3 b=3 c=4
    printf('a=%d b=%d c=%d$N', scrambled.a, scrambled.b, scrambled.c);
END_FUNCTION
