// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    gl_BOOL : BOOL;
    gl_TIME : TIME;
    gl_LTIME : LTIME;
END_VAR

TYPE
    MyBoolArray : ARRAY[1..2] OF BOOL := [TRUE, FALSE];

    MyStruct : STRUCT
        boolField : BOOL;
        timeField : TIME;
        ltimeField : LTIME;
    END_STRUCT;

    MySecStruct : STRUCT
        structField : MYSTRUCT;
    END_STRUCT;
END_TYPE

FUNCTION funcInvert : BOOL
    VAR_INPUT
        inVal : BOOL;
    END_VAR
    funcInvert := NOT inVal;
END_FUNCTION

FUNCTION setTimeTo : TIME
    VAR_INPUT
        inTime : TIME;
    END_VAR
    setTimeTo := inTime;
END_FUNCTION

FUNCTION setLTimeTo : LTIME
    VAR_INPUT
        inLTime : LTIME;
    END_VAR
    setLTimeTo := inLTime;
END_FUNCTION

FUNCTION_BLOCK fb_Copy
    VAR_INPUT
        inVal : BOOL;
        inTime : TIME;
        inLTime : LTIME;
    END_VAR
    VAR_OUTPUT
        outVal : BOOL;
        outTime : TIME;
        outLTime : LTIME;
    END_VAR

    outVal := inVal;
    outTime := inTime;
    outLTime := inLTime;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        myTP : TP;
        myTP2 : tP;
        myTON : TON;
        myTON2: ToN;
        myTOF : TOF;
        myTOF2 : tOF;
        myTP_TIME : TP_TIME;
        myTP_TIME2 : tP_tIME;
        myTON_TIME : TON_TIME;
        myTON_TIME2 : tON_tIME;
        myTOF_TIME : TOF_TIME;
        myTOF_TIME2 : TOF_TIMe;
        myTP_LTIME : TP_LTIME;
        myTP_LTIME2 : TP_LTiME;
        myTON_LTIME : TON_LTIME;
        myTON_LTIME2 : TON_LtIME;
        myTOF_LTIME : TOF_LTIME;
        myTOF_LTIME2 : TOf_LTIME;

        outVal : BOOL;
        outTime : TIME;
        outLTime : LTIME;
        structVar : MyStruct;
        secStructVar : MySecStruct;
        myFB : fb_Copy;
        myByteArray : ARRAY[0..1] OF BYTE := [2#00000101, 2#00000000];
        myArray : myBoolArray;
        myTimeArray : ARRAY[0..1] OF TIME := [T#10s, T#0s];
        myLTimeArray : ARRAY[0..1] OF LTIME := [LT#10s, LT#0s];
    END_VAR
    // my(IN (Start/Boolean), PT (Preset Time/Time), Q (Output/Boolean), ET (Elapsed Time/Time))
    // Below 3 calls are three different function block calls with same parameters
    myTP(IN:=funcInvert(FALSE), PT:=T#5s, Q=>outVal, ET=>outTime);
    PRINTF('TP function block formal call with IN=TRUE and PT=5s. Output Q = %d and ET = %d$N', outVal, outTime);
    // CHECK: TP function block formal call with IN=TRUE and PT=5s. Output Q = 1 and ET = 0

    myTP_TIME(IN:=funcInvert(FALSE), PT:=T#5s, Q=>outVal, ET=>outTime);
    PRINTF('TP_TIME function block formal call with IN=TRUE and PT=5s. Output Q = %d and ET = %d$N', outVal, outTime);
    // CHECK: TP_TIME function block formal call with IN=TRUE and PT=5s. Output Q = 1 and ET = 0

    myTP_LTIME(IN:=funcInvert(FALSE), PT:=LT#5s, Q=>outVal, ET=>outLTime);
    PRINTF('TP_LTIME function block formal call with IN=TRUE and PT=5s. Output Q = %d and ET = %lld$N', outVal, outLTime);
    // CHECK: TP_LTIME function block formal call with IN=TRUE and PT=5s. Output Q = 1 and ET = 0

    // TODO: These ones are causing a panic
/*
    // Below 3 calls are three different function block calls with same parameters
    myTP2 (IN := structVar.boolField , PT := myTimeArray[0] , Q => myFB.inVal , ET => secStructVar.structField.timeField);
    PRINTF('TP function block formal call with IN=FALSE and PT as array of time. Output Q = %d and ET = %d$N', myFB.inVal, secStructVar.structField.timeField);
    // HECK-NEXT: TP function block formal call with IN=FALSE and PT as array of time. Output Q = 0 and ET = 0

    myTP_TIME2 (IN := structVar.boolField , PT := myTimeArray[0] , Q => myFB.inVal , ET => secStructVar.structField.timeField);
    PRINTF('TP_TIME function block formal call with IN=FALSE and PT as array of time. Output Q = %d and ET = %d$N', myFB.inVal, secStructVar.structField.timeField);
    // HECK-NEXT: TP_TIME function block formal call with IN=FALSE and PT as array of time. Output Q = 0 and ET = 0

    myTP_LTIME2 (IN := structVar.boolField , PT := myLTimeArray[0] , Q => myFB.inVal , ET => secStructVar.structField.ltimeField);
    PRINTF('TP_LTIME function block formal call with IN=FALSE and PT as array of time. Output Q = %d and ET = %lld$N', myFB.inVal, secStructVar.structField.ltimeField);
    // HECK-NEXT: TP_LTIME function block formal call with IN=FALSE and PT as array of time. Output Q = 0 and ET = 0
*/

    // Below 3 calls are three different function block calls with same parameters
    myTON (FALSE, setTimeTo(T#5s),                    outVal, myTimeArray[1]);
    PRINTF('TON function block informal call with IN=FALSE and PT=5s. Output Q = %d and ET = %d$N', outVal, myTimeArray[1]);
    // CHECK-NEXT: TON function block informal call with IN=FALSE and PT=5s. Output Q = 0 and ET = 0

    myTON_TIME (FALSE, setTimeTo(T#5s),                    outVal, myTimeArray[1]);
    PRINTF('TON_TIME function block informal call with IN=FALSE and PT=5s. Output Q = %d and ET = %d$N', outVal, myTimeArray[1]);
    // CHECK-NEXT: TON_TIME function block informal call with IN=FALSE and PT=5s. Output Q = 0 and ET = 0

    myTON_LTIME (FALSE, setLTimeTo(LT#5s),                    outVal, myLTimeArray[1]);
    PRINTF('TON_LTIME function block informal call with IN=FALSE and PT=5s. Output Q = %d and ET = %lld$N', outVal, myLTimeArray[1]);
    // CHECK-NEXT: TON_LTIME function block informal call with IN=FALSE and PT=5s. Output Q = 0 and ET = 0

    // Below 3 calls are three different function block calls with same parameters
    myTON2 {info 'with comments'} (IN := myTON2.Q (* FALSE *), PT := myFB.outTime (* T#0s *) {info 'no output parameters'});
    PRINTF('TON function block formal call with comments and some parameters as variables. Output Q = %d and ET = %d$N', myTON2.Q, myTON2.ET);
    // CHECK-NEXT: TON function block formal call with comments and some parameters as variables. Output Q = 0 and ET = 0

    myTON_TIME2 {info 'with comments'} (IN := myTON2.Q (* FALSE *), PT := myFB.outTime (* T#0s *) {info 'no output parameters'});
    PRINTF('TON_TIME function block formal call with comments and some parameters as variables. Output Q = %d and ET = %d$N', myTON2.Q, myTON2.ET);
    // CHECK-NEXT: TON_TIME function block formal call with comments and some parameters as variables. Output Q = 0 and ET = 0

    myTON_LTIME2 {info 'with comments'} (IN := myTON2.Q (* FALSE *), PT := myFB.outLTime (* LT#0s *) {info 'no output parameters'});
    PRINTF('TON_LTIME function block formal call with comments and some parameters as variables. Output Q = %d and ET = %lld$N', myTON2.Q, myTON2.ET);
    // CHECK-NEXT: TON_LTIME function block formal call with comments and some parameters as variables. Output Q = 0 and ET = 0

    // Below 3 calls are three different function block calls with same parameters
    myTOF((*IN*) myByteArray[0].%X0, {info 'PT'} gl_Time, (*Q*) gl_BOOL, (*ET*) gl_Time);
    PRINTF('TOF function block informal call with IN as byte array element and global variable for PT. Output Q = %d and ET = %d$N', gl_BOOL, gl_Time);
    // CHECK-NEXT: TOF function block informal call with IN as byte array element and global variable for PT. Output Q = 1 and ET = 0

    myTOF_TIME((*IN*) myByteArray[0].%X0, {info 'PT'} gl_Time, (*Q*) gl_BOOL, (*ET*) gl_Time);
    PRINTF('TOF_TIME function block informal call with IN as byte array element and global variable for PT. Output Q = %d and ET = %d$N', gl_BOOL, gl_Time);
    // CHECK-NEXT: TOF_TIME function block informal call with IN as byte array element and global variable for PT. Output Q = 1 and ET = 0

    myTOF_LTIME((*IN*) myByteArray[0].%X0, {info 'PT'} gl_LTime, (*Q*) gl_BOOL, (*ET*) gl_LTime);
    PRINTF('TOF_LTIME function block informal call with IN as byte array element and global variable for PT. Output Q = %d and ET = %lld$N', gl_BOOL, gl_LTime);
    // CHECK-NEXT: TOF_LTIME function block informal call with IN as byte array element and global variable for PT. Output Q = 1 and ET = 0

    // TODO: These ones fail the "CHECK" command
/*
    // Below 3 call sequences (formal - informal - formal) are three different function block calls with same parameters
    myTOF2 (ET=>myTOF2.PT,PT:=structVar.timeField,IN:=myFB.outVal,Q=>myArray[1]);
    PRINTF('TOF function block formal call with IN as fb output and PT as struct field. Output Q = %d and ET = %d$N', myArray[1], myTOF2.PT);
    // HECK-NEXT: TOF function block formal call with IN as fb output and PT as struct field. Output Q = 0 and ET = 0

    myTOF2 (TRUE, T#5s, outVal, outTime );
    PRINTF('TOF function block informal call with IN=TRUE and PT=5s. Output Q = %d and ET = %d$N', outVal, outTime);
    // HECK-NEXT: TOF function block informal call with IN=TRUE and PT=5s. Output Q = 1 and ET = 0

    myTOF2 (PT:=myTOF2.PT, IN:=funcInvert(FALSE));
    PRINTF('TOF function block formal call with PT as previous ET and IN=TRUE. Output Q = %d and ET = %d$N', myTOF2.Q, myTOF2.PT);
    // HECK-NEXT: TOF function block formal call with PT as previous ET and IN=TRUE. Output Q = 1 and ET = 5000

    myTOF_TIME2 (ET=>myTOF_TIME2.PT,PT:=structVar.timeField,IN:=myFB.outVal,Q=>myArray[1]);
    PRINTF('TOF_TIME function block formal call with IN as fb output and PT as struct field. Output Q = %d and ET = %d$N', myArray[1], myTOF_TIME2.PT);
    // HECK-NEXT: TOF_TIME function block formal call with IN as fb output and PT as struct field. Output Q = 0 and ET = 0

    myTOF_TIME2 (TRUE, T#5s, outVal, outTime );
    PRINTF('TOF_TIME function block informal call with IN=TRUE and PT=5s. Output Q = %d and ET = %d$N', outVal, outTime);
    // HECK-NEXT: TOF_TIME function block informal call with IN=TRUE and PT=5s. Output Q = 1 and ET = 0

    myTOF_TIME2 (PT:=myTOF_TIME2.PT, IN:=funcInvert(FALSE));
    PRINTF('TOF_TIME function block formal call with PT as previous ET and IN=TRUE. Output Q = %d and ET = %d$N', myTOF_TIME2.Q, myTOF_TIME2.PT);
    // HECK-NEXT: TOF_TIME function block formal call with PT as previous ET and IN=TRUE. Output Q = 1 and ET = 5000

    myTOF_LTIME2 (ET=>myTOF_LTIME2.PT,PT:=structVar.ltimeField,IN:=myFB.outVal,Q=>myArray[1]);
    PRINTF('TOF_LTIME function block formal call with IN as fb output and PT as struct field. Output Q = %d and ET = %lld$N', myArray[1], myTOF_LTIME2.PT);
    // HECK-NEXT: TOF_LTIME function block formal call with IN as fb output and PT as struct field. Output Q = 0 and ET = 0

    myTOF_LTIME2 (TRUE, LT#5s, outVal, outLTime );
    PRINTF('TOF_LTIME function block informal call with IN=TRUE and PT=5s. Output Q = %d and ET = %lld$N', outVal, outLTime);
    // HECK-NEXT: TOF_LTIME function block informal call with IN=TRUE and PT=5s. Output Q = 1 and ET = 0

    myTOF_LTIME2 (PT:=myTOF_LTIME2.PT, IN:=funcInvert(FALSE));
    PRINTF('TOF_LTIME function block formal call with PT as previous ET and IN=TRUE. Output Q = %d and ET = %lld$N', myTOF_LTIME2.Q, myTOF_LTIME2.PT);
    // HECK-NEXT: TOF_LTIME function block formal call with PT as previous ET and IN=TRUE. Output Q = 1 and ET = 5000000000

    // Test calls with binary and unary operations
    myTP(IN:=TRUE AND TRUE, PT:=T#5s + T#1s, Q=>outVal, ET=>outTime);
    PRINTF('TP function block formal call with IN=TRUE AND TRUE and PT=5s + 1s. Output Q = %d and ET = %d$N', outVal, outTime);
    // HECK-NEXT: TP function block formal call with IN=TRUE AND TRUE and PT=5s + 1s. Output Q = 1 and ET = 0
*/

    myTON_TIME(FALSE OR FALSE, T#10s + T#1s, outVal, myTimeArray[1]);
    PRINTF('TON_TIME function block informal call with IN=FALSE OR FALSE and PT=10s + 1s. Output Q = %d and ET = %d$N', outVal, myTimeArray[1]);
    // CHECK-NEXT: TON_TIME function block informal call with IN=FALSE OR FALSE and PT=10s + 1s. Output Q = 0 and ET = 0

    myTOF_LTIME(IN := TRUE XOR FALSE, PT:=LT#10s + LT#1s);
    PRINTF('TOF_LTIME function block formal call with IN=TRUE XOR FALSE and PT=10s + 1s. Output Q = %d and ET = %lld$N', myTOF_LTIME.Q, myTOF_LTIME.PT);
    // CHECK-NEXT: TOF_LTIME function block formal call with IN=TRUE XOR FALSE and PT=10s + 1s. Output Q = 1 and ET = 11000000000
END_FUNCTION

// UNSUPPORTED: rusty
// error: PRG-3997 - Timer function block returns panic messages
