// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION VlaInit
VAR_IN_OUT
	arr	: ARRAY [*] OF BYTE;
	len	: ULINT;
END_VAR

VAR_TEMP
	i	: ULINT;
END_VAR

FOR i := 0 TO len - 1 DO
	arr[i] := i;
    printf('arr[%d] = %d$N', i, arr[i]);
END_FOR

END_FUNCTION

FUNCTION main : DINT
VAR
	arr1	: ARRAY [0..9] OF BYTE;
	arr2	: ARRAY [0..19] OF BYTE;
	l	: ULINT;
END_VAR

l := SIZEOF(arr1);
VlaInit(arr := arr1, len := l);
// CHECK: arr[0] = 0
//CHECK-NEXT: arr[1] = 1
//CHECK-NEXT: arr[2] = 2
//CHECK-NEXT: arr[3] = 3
//CHECK-NEXT: arr[4] = 4
//CHECK-NEXT: arr[5] = 5
//CHECK-NEXT: arr[6] = 6
//CHECK-NEXT: arr[7] = 7
//CHECK-NEXT: arr[8] = 8
//CHECK-NEXT: arr[9] = 9

l := SIZEOF(arr2);
VlaInit(arr := arr2, len := l);
//     CHECK: arr[0] = 0
//CHECK-NEXT: arr[1] = 1
//CHECK-NEXT: arr[2] = 2
//CHECK-NEXT: arr[3] = 3
//CHECK-NEXT: arr[4] = 4
//CHECK-NEXT: arr[5] = 5
//CHECK-NEXT: arr[6] = 6
//CHECK-NEXT: arr[7] = 7
//CHECK-NEXT: arr[8] = 8
//CHECK-NEXT: arr[9] = 9
//     CHECK: arr[10] = 10
//CHECK-NEXT: arr[11] = 11
//CHECK-NEXT: arr[12] = 12
//CHECK-NEXT: arr[13] = 13
//CHECK-NEXT: arr[14] = 14
//CHECK-NEXT: arr[15] = 15
//CHECK-NEXT: arr[16] = 16
//CHECK-NEXT: arr[17] = 17
//CHECK-NEXT: arr[18] = 18
//CHECK-NEXT: arr[19] = 19
END_FUNCTION
