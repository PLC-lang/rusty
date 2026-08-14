// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Program retain variables with inline type definitions: the extracted inline type
// and the generated retain pointer type must not clash (they both used to be named
// `__<pou>_<var>`), and initializers must land on the generated retain globals.
PROGRAM mainProg
    VAR RETAIN
        arr : ARRAY[0..2] OF DINT := [11, 22, 33];
        sub : DINT(0..100) := 50;
        str : STRING[20] := 'start';
        col : (RED, GREEN, BLUE) := GREEN;
        mat : ARRAY[0..1, 0..2] OF DINT;
        nst : ARRAY[0..1] OF ARRAY[0..2] OF DINT;
        rec : STRUCT a : DINT; b : DINT; END_STRUCT
    END_VAR

    arr[0] := arr[0] + 1;
    sub := sub + 1;
    mat[1, 2] := 42;
    nst[1][2] := 43;
    rec.a := rec.a + 1;
END_PROGRAM

FUNCTION main : DINT
    mainProg();
    printf('%d %d %d$N', mainProg.arr[0], mainProg.arr[1], mainProg.arr[2]); //CHECK: 12 22 33
    printf('%d$N', __mainProg_arr__retain[0]);                               //CHECK-NEXT: 12
    printf('%d %d$N', mainProg.sub, __mainProg_sub__retain);                 //CHECK-NEXT: 51 51
    printf('%s %s$N', REF(mainProg.str), REF(__mainProg_str__retain));       //CHECK-NEXT: start start
    printf('%d %d$N', mainProg.col, __mainProg_col__retain);                 //CHECK-NEXT: 1 1
    printf('%d %d$N', mainProg.mat[1, 2], __mainProg_mat__retain[1, 2]);     //CHECK-NEXT: 42 42
    printf('%d %d$N', mainProg.nst[1][2], __mainProg_nst__retain[1][2]);     //CHECK-NEXT: 43 43
    printf('%d %d$N', mainProg.rec.a, __mainProg_rec__retain.a);             //CHECK-NEXT: 1 1
END_FUNCTION
