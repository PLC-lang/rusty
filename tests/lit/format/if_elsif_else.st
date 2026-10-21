// RUN: %PLC format --stdout %s | %CHECK %s
IF   x>0 THEN y:=1;ELSIF x<0 THEN y:=2;ELSE y  :=   0  ;END_IF
// CHECK: IF x > 0 THEN
// CHECK-NEXT:     y := 1;
// CHECK-NEXT: ELSIF x < 0 THEN
// CHECK-NEXT:     y := 2;
// CHECK-NEXT: ELSE
// CHECK-NEXT:     y := 0;
// CHECK-NEXT: END_IF
