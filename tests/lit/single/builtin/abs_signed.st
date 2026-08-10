// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// ABS is a builtin; signed integers lower to the llvm.abs intrinsic with
// is_int_min_poison = false, so ABS(INT_MIN) wraps to INT_MIN.

FUNCTION main
VAR
    min : INT := INT#-32768;
END_VAR

// CHECK: sint=120 0 121 0
printf('sint=%d %d %d %d$N', ABS(SINT#-120), ABS(SINT#-0), ABS(SINT#121), ABS(SINT#0));

// CHECK: int=99 0 98 0
printf('int=%d %d %d %d$N', ABS(INT#-99), ABS(INT#-0), ABS(INT#98), ABS(INT#0));

// CHECK: dint=77 0 78 0
printf('dint=%d %d %d %d$N', ABS(-77), ABS(-0), ABS(78), ABS(0));

// CHECK: lint=88 0 89 0
printf('lint=%lld %lld %lld %lld$N', ABS(LINT#-88), ABS(LINT#-0), ABS(LINT#89), ABS(LINT#0));

// CHECK: big=1234567890123
printf('big=%lld$N', ABS(LINT#-1234567890123));

// CHECK: min=-32768
printf('min=%d$N', ABS(min));

// explicit calls to the library monomorphs still link against the stdlib
// CHECK: lib=7
printf('lib=%d$N', ABS__DINT(DINT#-7));
END_FUNCTION
