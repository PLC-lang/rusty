// RUN: %PLC format --stdout %s | %CHECK %s
PROGRAM p
VAR
x:INT; // the counter


// leading comment for y
y:BOOL;
END_VAR
END_PROGRAM
// CHECK: PROGRAM p
// CHECK-NEXT:     VAR
// CHECK-NEXT:         x: INT;  // the counter
// CHECK-EMPTY:
// CHECK-NEXT:         // leading comment for y
// CHECK-NEXT:         y: BOOL;
// CHECK-NEXT:     END_VAR
// CHECK-NEXT: END_PROGRAM
