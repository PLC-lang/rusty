// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// ABS on floats lowers to the llvm.fabs intrinsic.

FUNCTION main

// CHECK: real=3.2 0.0 3.3 0.0
printf('real=%.1f %.1f %.1f %.1f$N', ABS(REAL#-3.2), ABS(REAL#-0.0), ABS(REAL#3.3), ABS(REAL#0.0));

// CHECK: lreal=2.5 0.0 2.6 0.0
printf('lreal=%.1f %.1f %.1f %.1f$N', ABS(LREAL#-2.5), ABS(LREAL#-0.0), ABS(LREAL#2.6), ABS(LREAL#0.0));
END_FUNCTION
