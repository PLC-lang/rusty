PROGRAM mainProg
    VAR
        counter : DINT;

        FB_MA_dint : TEM_MovingAverage_UDINT;
        bytes : ARRAY[0..20] OF BYTE;

        AvgCount : UDINT;
        start : BOOL;
        clear : BOOL;
    END_VAR

    counter := 1234;

    bytes[0] := 1;
    bytes[1] := 2;
    bytes[2] := 3;
    bytes[3] := 4;
    bytes[4] := 5;
    bytes[5] := 6;
    bytes[6] := 7;
    bytes[7] := 8;
    bytes[8] := 9;
    bytes[9] := 10;
    bytes[10] := 11;
    bytes[11] := 12;
    bytes[12] := 13;
    bytes[13] := 14;
    bytes[14] := 15;
    bytes[15] := 16;
    bytes[16] := 17;
    bytes[17] := 18;
    bytes[18] := 19;
    bytes[19] := 20;
    bytes[20] := 21;

    IF start THEN
        counter := counter +1;
    END_IF
    IF clear THEN
        clear := FALSE;
        counter := 0;
    END_IF

    FB_MA_dint(
        diInputValue := counter,
        udiCycles := 1000,
        diAverageValue => AvgCount
    );

    // CHECK: bytes[0]=1
    printf('bytes[0]=%d$N', bytes[0]);
    // CHECK: bytes[1]=2
    printf('bytes[1]=%d$N', bytes[1]);
    // CHECK: bytes[2]=3
    printf('bytes[2]=%d$N', bytes[2]);
    // CHECK: bytes[3]=4
    printf('bytes[3]=%d$N', bytes[3]);
    // CHECK: bytes[4]=5
    printf('bytes[4]=%d$N', bytes[4]);
    // CHECK: bytes[5]=6
    printf('bytes[5]=%d$N', bytes[5]);
    // CHECK: bytes[6]=7
    printf('bytes[6]=%d$N', bytes[6]);
    // CHECK: bytes[7]=8
    printf('bytes[7]=%d$N', bytes[7]);
    // CHECK: bytes[8]=9
    printf('bytes[8]=%d$N', bytes[8]);
    // CHECK: bytes[9]=10
    printf('bytes[9]=%d$N', bytes[9]);
    // CHECK: bytes[10]=11
    printf('bytes[10]=%d$N', bytes[10]);
    // CHECK: bytes[11]=12
    printf('bytes[11]=%d$N', bytes[11]);
    // CHECK: bytes[12]=13
    printf('bytes[12]=%d$N', bytes[12]);
    // CHECK: bytes[13]=14
    printf('bytes[13]=%d$N', bytes[13]);
    // CHECK: bytes[14]=15
    printf('bytes[14]=%d$N', bytes[14]);
    // CHECK: bytes[15]=16
    printf('bytes[15]=%d$N', bytes[15]);
    // CHECK: bytes[16]=17
    printf('bytes[16]=%d$N', bytes[16]);
    // CHECK: bytes[17]=18
    printf('bytes[17]=%d$N', bytes[17]);
    // CHECK: bytes[18]=19
    printf('bytes[18]=%d$N', bytes[18]);
    // CHECK: bytes[19]=20
    printf('bytes[19]=%d$N', bytes[19]);
    // CHECK: bytes[20]=21
    printf('bytes[20]=%d$N', bytes[20]);
END_PROGRAM

FUNCTION main : DINT
    mainProg();
END_FUNCTION
