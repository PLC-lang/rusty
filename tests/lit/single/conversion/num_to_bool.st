// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// Every *_TO_BOOL conversion is FALSE for exactly zero and TRUE for everything else. It is not a
// lowest-bit test, a fraction is not truncated first, there is no epsilon, and NaN and the
// infinities are TRUE.

FUNCTION main
VAR
    umax : ULINT := 0;
    ninf : REAL;
END_VAR
umax := umax - 1;   // 2^64 - 1
ninf := -INF_REAL;

// signed integers
printf('%d %d %d %d %d$N', SINT_TO_BOOL(SINT#-128), SINT_TO_BOOL(SINT#0), INT_TO_BOOL(INT#2), INT_TO_BOOL(INT#256), INT_TO_BOOL(INT#-1));  // CHECK: 1 0 1 1 1
printf('%d %d %d %d$N',    INT_TO_BOOL(INT#0), INT_TO_BOOL(INT#1), DINT_TO_BOOL(DINT#65536), LINT_TO_BOOL(LINT#4294967296));                  // CHECK: 0 1 1 1

// unsigned integers
printf('%d %d %d %d$N', USINT_TO_BOOL(USINT#255), USINT_TO_BOOL(USINT#0), UINT_TO_BOOL(UINT#256), UDINT_TO_BOOL(UDINT#4294967295));  // CHECK: 1 0 1 1
printf('%d %d %d$N',    ULINT_TO_BOOL(umax), ULINT_TO_BOOL(ULINT#4294967296), ULINT_TO_BOOL(ULINT#0));                             // CHECK: 1 1 0

// reals
printf('%d %d %d %d$N', REAL_TO_BOOL(REAL#0.5), REAL_TO_BOOL(REAL#1.0E-30), REAL_TO_BOOL(REAL#0.0), REAL_TO_BOOL(-REAL#0.0));  // CHECK: 1 1 0 0
printf('%d %d %d$N',    REAL_TO_BOOL(NAN_REAL), REAL_TO_BOOL(INF_REAL), REAL_TO_BOOL(ninf));                                    // CHECK: 1 1 1
printf('%d %d %d$N',    LREAL_TO_BOOL(LREAL#-0.5), LREAL_TO_BOOL(LREAL#0.0), LREAL_TO_BOOL(NAN_LREAL));                          // CHECK: 1 0 1

// bit strings, a set bit above bit 0 is TRUE as well
printf('%d %d %d %d %d$N', BYTE_TO_BOOL(BYTE#2), BYTE_TO_BOOL(BYTE#0), WORD_TO_BOOL(WORD#256), DWORD_TO_BOOL(DWORD#65536), LWORD_TO_BOOL(LWORD#4294967296));  // CHECK: 1 0 1 1 1

// the generic form calls the named one
printf('%d %d %d %d$N', TO_BOOL(INT#256), TO_BOOL(REAL#0.5), TO_BOOL(ULINT#4294967296), TO_BOOL(BYTE#2));  // CHECK: 1 1 1 1
END_FUNCTION
