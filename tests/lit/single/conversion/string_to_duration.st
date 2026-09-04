// STRING_TO_TIME and STRING_TO_LTIME parse valid duration literals.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    printf('time_fraction_1_5s=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1.5s')))); // CHECK: time_fraction_1_5s=T#1s500ms
    printf('time_fraction_0_5s=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#0.5s')))); // CHECK-NEXT: time_fraction_0_5s=T#500ms
    printf('time_fraction_2_75s=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#2.75s')))); // CHECK-NEXT: time_fraction_2_75s=T#2s750ms
    printf('time_fraction_comma=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1,5s')))); // CHECK-NEXT: time_fraction_comma=T#1s500ms
    printf('time_fraction_resolution=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1.0004ms')))); // CHECK-NEXT: time_fraction_resolution=T#1ms
    printf('ltime_fraction=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('LTIME#1.5s')))); // CHECK-NEXT: ltime_fraction=LTIME#1s500ms
    printf('time_compound=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('TIME#1d2h3m4s5ms')))); // CHECK-NEXT: time_compound=T#1d2h3m4s5ms
    printf('time_case_insensitive=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('time#1s')))); // CHECK-NEXT: time_case_insensitive=T#1s
    printf('ltime_nanoseconds=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('LT#1ns')))); // CHECK-NEXT: ltime_nanoseconds=LTIME#1ns
    printf('time_negative=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#-1s')))); // CHECK-NEXT: time_negative=T#0ms
    printf('ltime_negative=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('LTIME#-1s')))); // CHECK-NEXT: ltime_negative=LTIME#0ns
    printf('time_empty=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('')))); // CHECK-NEXT: time_empty=T#0ms
    printf('time_garbage=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('abc')))); // CHECK-NEXT: time_garbage=T#0ms
    printf('time_missing_prefix=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('1s')))); // CHECK-NEXT: time_missing_prefix=T#0ms
    printf('time_wrong_prefix=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('LTIME#1s')))); // CHECK-NEXT: time_wrong_prefix=T#0ms
    printf('time_wrong_formatter=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('TOD#12:45:00')))); // CHECK-NEXT: time_wrong_formatter=T#0ms
    printf('time_67ms=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#67ms')))); // CHECK-NEXT: time_67ms=T#67ms
    printf('ltime_wrong_formatter=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('T#1s')))); // CHECK-NEXT: ltime_wrong_formatter=LTIME#0ns
    printf('time_missing_unit=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1000')))); // CHECK-NEXT: time_missing_unit=T#0ms
    printf('time_internal_space=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1h 30m')))); // CHECK-NEXT: time_internal_space=T#1h30m
END_FUNCTION
