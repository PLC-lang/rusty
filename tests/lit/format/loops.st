// RUN: %PLC format --stdout %s | %CHECK %s
PROGRAM p
VAR
i:INT;
END_VAR
FOR i:=1 TO 10 BY 1 DO
i:=i+1;
END_FOR
WHILE i<100 DO
i:=i*2;
END_WHILE
REPEAT
i:=i-1;
UNTIL i<=0
END_REPEAT
END_PROGRAM
// CHECK: PROGRAM p
// CHECK-NEXT:     VAR
// CHECK-NEXT:         i: INT;
// CHECK-NEXT:     END_VAR
// CHECK-NEXT:     FOR i := 1 TO 10 BY 1 DO
// CHECK-NEXT:         i := i + 1;
// CHECK-NEXT:     END_FOR
// CHECK-NEXT:     WHILE i < 100 DO
// CHECK-NEXT:         i := i * 2;
// CHECK-NEXT:     END_WHILE
// CHECK-NEXT:     REPEAT
// CHECK-NEXT:         i := i - 1;
// CHECK-NEXT:     UNTIL i <= 0
// CHECK-NEXT:     END_REPEAT
// CHECK-NEXT: END_PROGRAM
