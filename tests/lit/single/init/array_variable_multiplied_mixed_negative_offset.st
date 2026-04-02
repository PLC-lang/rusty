// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests mixed variable multipliers with negative array bounds.
// [(var1)(10), 11, (var2)(12)] where var1=3, var2=7

FUNCTION main : DINT
VAR
    var1 : DINT := 3;
    var2 : DINT := 7;
    arr : ARRAY [-10..0] OF DINT := [(var1)(10), 11, (var2)(12)];
END_VAR
    // var1=3 elements: arr[-10]=10, arr[-9]=10, arr[-8]=10
    printf('%d$N', arr[-10]);  // CHECK: 10
    printf('%d$N', arr[-9]);   // CHECK: 10
    printf('%d$N', arr[-8]);   // CHECK: 10
    // individual: arr[-7]=11
    printf('%d$N', arr[-7]);   // CHECK: 11
    // var2=7 elements: arr[-6]=12 .. arr[0]=12
    printf('%d$N', arr[-6]);   // CHECK: 12
    printf('%d$N', arr[-3]);   // CHECK: 12
    printf('%d$N', arr[0]);    // CHECK: 12

    main := 0;
END_FUNCTION
