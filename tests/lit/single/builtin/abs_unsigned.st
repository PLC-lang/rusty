// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// ABS of an unsigned value returns the value unchanged instead of calling a
// non-existent monomorph (previously a crash at runtime). Each call reports
// warning E150, see abs_unsigned_warning.st.

FUNCTION main
VAR
    u8  : USINT := USINT#200;
    u16 : UINT  := UINT#5;
    u32 : UDINT := UDINT#4000000000;
    u64 : ULINT := ULINT#42;
END_VAR

// CHECK: u8=200
printf('u8=%d$N', ABS(u8));

// CHECK: u16=5
printf('u16=%d$N', ABS(u16));

// CHECK: u32=4000000000
printf('u32=%u$N', ABS(u32));

// CHECK: u64=42
printf('u64=%llu$N', ABS(u64));
END_FUNCTION
