// STRING_TO_DATE, STRING_TO_DT, and STRING_TO_TOD accept exactly the calendar literals of their type.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    // Valid prefixed literals parse for each target type.
    printf('date_valid=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-02-29')))); // CHECK: date_valid=D#2024-02-29
    printf('dt_valid=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-12:00:00')))); // CHECK-NEXT: dt_valid=DT#2024-01-01-12:00:00
    printf('tod_valid=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:00:00')))); // CHECK-NEXT: tod_valid=TOD#12:00:00

    // Empty input returns each type's zero value.
    printf('date_empty=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('')))); // CHECK-NEXT: date_empty=D#1970-01-01
    printf('dt_empty=%s$N', REF(DT_TO_STRING(STRING_TO_DT('')))); // CHECK-NEXT: dt_empty=DT#1970-01-01-00:00:00
    printf('tod_empty=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('')))); // CHECK-NEXT: tod_empty=TOD#00:00:00

    // Malformed input returns each type's zero value.
    printf('date_garbage=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('not a date')))); // CHECK-NEXT: date_garbage=D#1970-01-01
    printf('dt_garbage=%s$N', REF(DT_TO_STRING(STRING_TO_DT('$00')))); // CHECK-NEXT: dt_garbage=DT#1970-01-01-00:00:00
    printf('tod_garbage=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('bad')))); // CHECK-NEXT: tod_garbage=TOD#00:00:00

    // Each conversion rejects a literal prefix for a different calendar type.
    printf('date_wrong_prefix=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('TOD#12:45:00')))); // CHECK-NEXT: date_wrong_prefix=D#1970-01-01
    printf('dt_wrong_prefix=%s$N', REF(DT_TO_STRING(STRING_TO_DT('D#2024-01-01')))); // CHECK-NEXT: dt_wrong_prefix=DT#1970-01-01-00:00:00
    printf('tod_wrong_prefix=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('DT#2024-01-01-12:00:00')))); // CHECK-NEXT: tod_wrong_prefix=TOD#00:00:00

    // Out-of-range date and time fields are rejected.
    printf('date_invalid_day=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-02-30')))); // CHECK-NEXT: date_invalid_day=D#1970-01-01
    printf('dt_invalid_hour=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-25:00:00')))); // CHECK-NEXT: dt_invalid_hour=DT#1970-01-01-00:00:00
    printf('tod_invalid_minute=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:60:00')))); // CHECK-NEXT: tod_invalid_minute=TOD#00:00:00

    // Impossible time-of-day fields return the zero value.
    printf('tod_high_range=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#25:00:00')))); // CHECK-NEXT: tod_high_range=TOD#00:00:00
    printf('tod_low_range=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#-1:00:00')))); // CHECK-NEXT: tod_low_range=TOD#00:00:00

    // Dates before the epoch and after 2106 are representable by the 64-bit types.
    printf('date_low_range=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#1969-12-31')))); // CHECK-NEXT: date_low_range=D#1969-12-31
    printf('dt_low_range=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#1969-12-31-23:59:59')))); // CHECK-NEXT: dt_low_range=DT#1969-12-31-23:59:59
    printf('date_high_range=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2106-02-08')))); // CHECK-NEXT: date_high_range=D#2106-02-08
    printf('dt_high_range=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2106-02-08-00:00:00')))); // CHECK-NEXT: dt_high_range=DT#2106-02-08-00:00:00

    // Dates outside the 64-bit nanosecond range return the zero value.
    printf('date_out_of_range=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#9999-12-31')))); // CHECK-NEXT: date_out_of_range=D#1970-01-01
    printf('dt_out_of_range=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#9999-12-31-00:00:00')))); // CHECK-NEXT: dt_out_of_range=DT#1970-01-01-00:00:00

    // DATE rejects fractions, DT and TOD preserve milliseconds.
    printf('date_fraction=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-01-01.500')))); // CHECK-NEXT: date_fraction=D#1970-01-01
    printf('dt_fraction=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-12:00:00.500')))); // CHECK-NEXT: dt_fraction=DT#2024-01-01-12:00:00.500
    printf('tod_fraction=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:00:00.500')))); // CHECK-NEXT: tod_fraction=TOD#12:00:00.500

    // The literal prefix is required; bare ISO forms are rejected.
    printf('date_unprefixed=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('2024-01-01')))); // CHECK-NEXT: date_unprefixed=D#1970-01-01
    printf('dt_unprefixed_date=%s$N', REF(DT_TO_STRING(STRING_TO_DT('2024-01-01')))); // CHECK-NEXT: dt_unprefixed_date=DT#1970-01-01-00:00:00
    printf('dt_unprefixed=%s$N', REF(DT_TO_STRING(STRING_TO_DT('2024-01-01T12:00:00')))); // CHECK-NEXT: dt_unprefixed=DT#1970-01-01-00:00:00
    printf('tod_unprefixed=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('12:00:00')))); // CHECK-NEXT: tod_unprefixed=TOD#00:00:00

    // The long spelling of each prefix is accepted, case-insensitively.
    printf('date_long_prefix=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('date#2024-01-01')))); // CHECK-NEXT: date_long_prefix=D#2024-01-01
    printf('dt_long_prefix=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DATE_AND_TIME#2024-01-01-12:00:00')))); // CHECK-NEXT: dt_long_prefix=DT#2024-01-01-12:00:00
    printf('tod_long_prefix=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('time_of_day#12:00:00')))); // CHECK-NEXT: tod_long_prefix=TOD#12:00:00

    // The 64-bit literal prefixes belong to LDATE, LDT and LTOD and are rejected here.
    printf('date_ldate_prefix=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('LDATE#2024-01-01')))); // CHECK-NEXT: date_ldate_prefix=D#1970-01-01
    printf('dt_ldt_prefix=%s$N', REF(DT_TO_STRING(STRING_TO_DT('LDT#2024-01-01-12:00:00')))); // CHECK-NEXT: dt_ldt_prefix=DT#1970-01-01-00:00:00
    printf('tod_ltod_prefix=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('LTOD#12:00:00')))); // CHECK-NEXT: tod_ltod_prefix=TOD#00:00:00

    // Nothing may follow the literal, not even comment-like text.
    printf('date_trailing=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-01-01//x')))); // CHECK-NEXT: date_trailing=D#1970-01-01
    printf('dt_trailing=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-12:00:00(*x*)')))); // CHECK-NEXT: dt_trailing=DT#1970-01-01-00:00:00
    printf('tod_trailing=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:00:00{x}')))); // CHECK-NEXT: tod_trailing=TOD#00:00:00

    // Fractions truncate: no rounding into the next second or day.
    printf('tod_truncated=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#23:59:59.9999999999')))); // CHECK-NEXT: tod_truncated=TOD#23:59:59.999999999
    printf('dt_truncated=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-23:59:59.9999999999')))); // CHECK-NEXT: dt_truncated=DT#2024-01-01-23:59:59.999999999

    // Single-digit fields and optional seconds follow the literal grammar.
    printf('date_short_fields=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-1-1')))); // CHECK-NEXT: date_short_fields=D#2024-01-01
    printf('dt_no_seconds=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-12:00')))); // CHECK-NEXT: dt_no_seconds=DT#2024-01-01-12:00:00
    printf('tod_no_seconds=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:00')))); // CHECK-NEXT: tod_no_seconds=TOD#12:00:00

    // Surrounding whitespace is ignored for every target type.
    printf('date_trimmed=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE(' D#2024-01-01 ')))); // CHECK-NEXT: date_trimmed=D#2024-01-01
    printf('dt_trimmed=%s$N', REF(DT_TO_STRING(STRING_TO_DT('$TDT#2024-01-01-12:00:00$R$N')))); // CHECK-NEXT: dt_trimmed=DT#2024-01-01-12:00:00
    printf('tod_trimmed=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('$TTOD#12:00:00$T')))); // CHECK-NEXT: tod_trimmed=TOD#12:00:00

    // Whitespace within a literal is rejected for every target type.
    printf('date_internal_space=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D# 2024-01-01')))); // CHECK-NEXT: date_internal_space=D#1970-01-01
    printf('dt_internal_space=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01 12:00:00')))); // CHECK-NEXT: dt_internal_space=DT#1970-01-01-00:00:00
    printf('tod_internal_space=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12: 00:00')))); // CHECK-NEXT: tod_internal_space=TOD#00:00:00
END_FUNCTION
