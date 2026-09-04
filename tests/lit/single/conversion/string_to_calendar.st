// STRING_TO_DATE, STRING_TO_DT, and STRING_TO_TOD accept valid calendar literals.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    // Valid prefixed literals parse for each target type.
    printf('date_valid=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-02-29')))); // CHECK: date_valid=2024-02-29
    printf('dt_valid=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-12:00:00')))); // CHECK-NEXT: dt_valid=2024-01-01-12:00:00
    printf('tod_valid=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:00:00')))); // CHECK-NEXT: tod_valid=12:00:00

    // Empty input returns each type's zero value.
    printf('date_empty=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('')))); // CHECK-NEXT: date_empty=1970-01-01
    printf('dt_empty=%s$N', REF(DT_TO_STRING(STRING_TO_DT('')))); // CHECK-NEXT: dt_empty=1970-01-01-00:00:00
    printf('tod_empty=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('')))); // CHECK-NEXT: tod_empty=00:00:00

    // Malformed input returns each type's zero value.
    printf('date_garbage=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('not a date')))); // CHECK-NEXT: date_garbage=1970-01-01
    printf('dt_garbage=%s$N', REF(DT_TO_STRING(STRING_TO_DT('$00')))); // CHECK-NEXT: dt_garbage=1970-01-01-00:00:00
    printf('tod_garbage=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('bad')))); // CHECK-NEXT: tod_garbage=00:00:00

    // Each conversion rejects a literal prefix for a different calendar type.
    printf('date_wrong_prefix=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('TOD#12:45:00')))); // CHECK-NEXT: date_wrong_prefix=1970-01-01
    printf('dt_wrong_prefix=%s$N', REF(DT_TO_STRING(STRING_TO_DT('D#2024-01-01')))); // CHECK-NEXT: dt_wrong_prefix=1970-01-01-00:00:00
    printf('tod_wrong_prefix=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('DT#2024-01-01-12:00:00')))); // CHECK-NEXT: tod_wrong_prefix=00:00:00

    // Out-of-range date and time fields are rejected.
    printf('date_invalid_day=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-02-30')))); // CHECK-NEXT: date_invalid_day=1970-01-01
    printf('dt_invalid_hour=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-25:00:00')))); // CHECK-NEXT: dt_invalid_hour=1970-01-01-00:00:00
    printf('tod_invalid_minute=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:60:00')))); // CHECK-NEXT: tod_invalid_minute=00:00:00

    // Values outside the representable range return the zero value.
    printf('date_low_range=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#1969-12-31')))); // CHECK-NEXT: date_low_range=1970-01-01
    printf('dt_low_range=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#1969-12-31-23:59:59')))); // CHECK-NEXT: dt_low_range=1970-01-01-00:00:00
    printf('tod_high_range=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#25:00:00')))); // CHECK-NEXT: tod_high_range=00:00:00
    printf('date_high_range=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2106-02-08')))); // CHECK-NEXT: date_high_range=1970-01-01
    printf('dt_high_range=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2106-02-08-00:00:00')))); // CHECK-NEXT: dt_high_range=1970-01-01-00:00:00
    printf('tod_low_range=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#-1:00:00')))); // CHECK-NEXT: tod_low_range=00:00:00

    // DATE rejects fractions, DT drops them, and TOD preserves milliseconds.
    printf('date_fraction=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-01-01.500')))); // CHECK-NEXT: date_fraction=1970-01-01
    printf('dt_fraction=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-12:00:00.500')))); // CHECK-NEXT: dt_fraction=2024-01-01-12:00:00
    printf('tod_fraction=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:00:00.500')))); // CHECK-NEXT: tod_fraction=12:00:00.500

    // The type-specific ISO forms work without their literal prefixes.
    printf('date_unprefixed=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('2024-01-01')))); // CHECK-NEXT: date_unprefixed=2024-01-01
    printf('dt_unprefixed_date=%s$N', REF(DT_TO_STRING(STRING_TO_DT('2024-01-01')))); // CHECK-NEXT: dt_unprefixed_date=2024-01-01-00:00:00
    printf('dt_unprefixed=%s$N', REF(DT_TO_STRING(STRING_TO_DT('2024-01-01T12:00:00')))); // CHECK-NEXT: dt_unprefixed=2024-01-01-12:00:00
    printf('tod_unprefixed=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('12:00:00')))); // CHECK-NEXT: tod_unprefixed=12:00:00

    // Surrounding whitespace is ignored for every target type.
    printf('date_trimmed=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE(' D#2024-01-01 ')))); // CHECK-NEXT: date_trimmed=2024-01-01
    printf('dt_trimmed=%s$N', REF(DT_TO_STRING(STRING_TO_DT('$TDT#2024-01-01-12:00:00$R$N')))); // CHECK-NEXT: dt_trimmed=2024-01-01-12:00:00
    printf('tod_trimmed=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('$TTOD#12:00:00$T')))); // CHECK-NEXT: tod_trimmed=12:00:00

    // Whitespace within a literal is rejected for every target type.
    printf('date_internal_space=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D# 2024-01-01')))); // CHECK-NEXT: date_internal_space=1970-01-01
    printf('dt_internal_space=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01 12:00:00')))); // CHECK-NEXT: dt_internal_space=1970-01-01-00:00:00
    printf('tod_internal_space=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12: 00:00')))); // CHECK-NEXT: tod_internal_space=00:00:00
END_FUNCTION
