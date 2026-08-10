// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// ABS with an unsigned argument binds to the ABS__U* builtins and returns the
// value unchanged instead of calling a non-existent library monomorph
// (previously a crash at runtime). Signed and float arguments still call the
// library implementations.

FUNCTION main
VAR
    s   : SINT  := SINT#-128;
    i   : INT   := INT#-3;
    d   : DINT  := DINT#-10;
    l   : LINT  := LINT#-1234567890123;
    u8  : USINT := USINT#200;
    u16 : UINT  := UINT#5;
    u32 : UDINT := UDINT#4000000000;
    u64 : ULINT := ULINT#42;
    r   : REAL  := REAL#-2.0;
    lr  : LREAL := LREAL#-9.5;
END_VAR

// ABS(INT_MIN) wraps to INT_MIN
// CHECK: s=-128
printf('s=%d$N', ABS(s));

// CHECK: i=3
printf('i=%d$N', ABS(i));

// CHECK: d=10
printf('d=%d$N', ABS(d));

// CHECK: l=1234567890123
printf('l=%lld$N', ABS(l));

// CHECK: u8=200
printf('u8=%d$N', ABS(u8));

// CHECK: u16=5
printf('u16=%d$N', ABS(u16));

// CHECK: u32=4000000000
printf('u32=%u$N', ABS(u32));

// CHECK: u64=42
printf('u64=%llu$N', ABS(u64));

// CHECK: r=2.0
printf('r=%.1f$N', ABS(r));

// CHECK: lr=9.5
printf('lr=%.1f$N', ABS(lr));

// explicit calls to the library monomorphs still link against the stdlib
// CHECK: lib=7
printf('lib=%d$N', ABS__DINT(DINT#-7));
END_FUNCTION
