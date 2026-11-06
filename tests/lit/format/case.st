// RUN: %PLC format --stdout %s | %CHECK %s
PROGRAM p
VAR
x:INT;
END_VAR
CASE x OF
1:x:=1;
2,3:x:=2;
ELSE
x:=0;
END_CASE
END_PROGRAM
// CHECK: PROGRAM p
// CHECK-NEXT:     VAR
// CHECK-NEXT:         x: INT;
// CHECK-NEXT:     END_VAR
// CHECK-NEXT:     CASE x OF
// CHECK-NEXT:         1: x := 1;
// CHECK-NEXT:         2, 3: x := 2;
// CHECK-NEXT:         ELSE
// CHECK-NEXT:         x := 0;
// CHECK-NEXT:     END_CASE
// CHECK-NEXT: END_PROGRAM
