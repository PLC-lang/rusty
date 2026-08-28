// Full roundtrip regression coverage for STRING_TO_* conversion behavior.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    // Never faults and rejected input returns zero-value defaults.
    printf('udint_empty=%d$N', STRING_TO_UDINT(''));                                      // CHECK: udint_empty=0
    printf('date_garbage=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('not a date'))));       // CHECK-NEXT: date_garbage=1970-01-01
    printf('dt_garbage=%s$N', REF(DT_TO_STRING(STRING_TO_DT('$00'))));                    // CHECK-NEXT: dt_garbage=1970-01-01-00:00:00
    printf('tod_garbage=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('bad'))));                 // CHECK-NEXT: tod_garbage=00:00:00
    printf('ltime_garbage=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('!!!'))));           // CHECK-NEXT: ltime_garbage=
    printf('bool_garbage=%d$N', STRING_TO_BOOL('garbage'));                                // CHECK-NEXT: bool_garbage=0

    // BOOL behavior (D3).
    printf('bool_1=%d$N', STRING_TO_BOOL('1'));                                           // CHECK-NEXT: bool_1=1
    printf('bool_TRUE=%d$N', STRING_TO_BOOL('TRUE'));                                     // CHECK-NEXT: bool_TRUE=1
    printf('bool_true=%d$N', STRING_TO_BOOL('true'));                                     // CHECK-NEXT: bool_true=1
    printf('bool_trimmed=%d$N', STRING_TO_BOOL('  true  '));                              // CHECK-NEXT: bool_trimmed=1
    printf('bool_0=%d$N', STRING_TO_BOOL('0'));                                           // CHECK-NEXT: bool_0=0
    printf('bool_suffix=%d$N', STRING_TO_BOOL('TRUEX'));                                  // CHECK-NEXT: bool_suffix=0

    // One parse rule across integer widths (D2), including all requested widths.
    printf('byte_hex=%d$N', STRING_TO_BYTE('16#FF'));                                      // CHECK-NEXT: byte_hex=255
    printf('word_hex=%d$N', STRING_TO_WORD('16#FF'));                                      // CHECK-NEXT: word_hex=255
    printf('dword_hex=%d$N', STRING_TO_DWORD('16#FF'));                                   // CHECK-NEXT: dword_hex=255
    printf('lword_hex=%d$N', STRING_TO_LWORD('16#FF'));                                   // CHECK-NEXT: lword_hex=255
    printf('sint_neg=%d$N', STRING_TO_SINT('-1'));                                        // CHECK-NEXT: sint_neg=-1
    printf('usint_partial=%d$N', STRING_TO_USINT('12abc'));                               // CHECK-NEXT: usint_partial=0
    printf('int_neg=%d$N', STRING_TO_INT('-1'));                                          // CHECK-NEXT: int_neg=-1
    printf('uint_partial=%d$N', STRING_TO_UINT('12abc'));                                 // CHECK-NEXT: uint_partial=0
    printf('udint_partial=%d$N', STRING_TO_UDINT('12abc'));                               // CHECK-NEXT: udint_partial=0
    printf('ulint_partial=%d$N', STRING_TO_ULINT('12abc'));                               // CHECK-NEXT: ulint_partial=0
    printf('udint_fraction=%d$N', STRING_TO_UDINT('1.9'));                                // CHECK-NEXT: udint_fraction=1
    printf('ulint_fraction=%d$N', STRING_TO_ULINT('1.9'));                                // CHECK-NEXT: ulint_fraction=1
    printf('udint_trim=%d$N', STRING_TO_UDINT('  12  '));                                 // CHECK-NEXT: udint_trim=12
    printf('ulint_trim=%d$N', STRING_TO_ULINT('  12  '));                                 // CHECK-NEXT: ulint_trim=12
    printf('byte_negative=%d$N', STRING_TO_BYTE('-1'));                                   // CHECK-NEXT: byte_negative=0

    // Fractional durations (D4), negative duration rejection (D5), and 67ms fallback removal (D6).
    printf('time_fraction_1_5s=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1.5s'))));       // CHECK-NEXT: time_fraction_1_5s=1s500ms
    printf('time_fraction_0_5s=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#0.5s'))));       // CHECK-NEXT: time_fraction_0_5s=500ms
    printf('time_fraction_2_75s=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#2.75s'))));     // CHECK-NEXT: time_fraction_2_75s=2s750ms
    printf('time_fraction_comma=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1,5s'))));      // CHECK-NEXT: time_fraction_comma=1s500ms
    printf('time_fraction_resolution=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1.0004ms')))); // CHECK-NEXT: time_fraction_resolution=1ms
    printf('ltime_fraction=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('LTIME#1.5s'))));     // CHECK-NEXT: ltime_fraction=1s500ms
    printf('time_negative=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#-1s'))));             // CHECK-NEXT: time_negative=
    printf('ltime_negative=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('LTIME#-1s'))));      // CHECK-NEXT: ltime_negative=
    printf('time_empty=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME(''))));                     // CHECK-NEXT: time_empty=
    printf('time_garbage=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('abc'))));                // CHECK-NEXT: time_garbage=
    printf('time_missing_prefix=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('1s'))));          // CHECK-NEXT: time_missing_prefix=
    printf('time_wrong_prefix=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('LTIME#1s'))));      // CHECK-NEXT: time_wrong_prefix=
    printf('time_wrong_formatter=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('TOD#12:45:00')))); // CHECK-NEXT: time_wrong_formatter=
    printf('time_67ms=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#67ms'))));                // CHECK-NEXT: time_67ms=67ms
    printf('ltime_wrong_formatter=%s$N', REF(LTIME_TO_STRING(STRING_TO_LTIME('T#1s'))));   // CHECK-NEXT: ltime_wrong_formatter=

    // Date/time validity and range behavior (D7, D8).
    printf('date_impossible=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-02-30'))));              // CHECK-NEXT: date_impossible=1970-01-01
    printf('date_month=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-13-01'))));                    // CHECK-NEXT: date_month=1970-01-01
    printf('date_nonleap=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2023-02-29'))));                  // CHECK-NEXT: date_nonleap=1970-01-01
    printf('date_leap=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-02-29'))));                     // CHECK-NEXT: date_leap=2024-02-29
    printf('date_wrong_formatter=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('TOD#12:45:00'))));          // CHECK-NEXT: date_wrong_formatter=1970-01-01
    printf('dt_hour=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#2024-01-01-25:00:00'))));                 // CHECK-NEXT: dt_hour=1970-01-01-00:00:00
    printf('dt_wrong_formatter=%s$N', REF(DT_TO_STRING(STRING_TO_DT('TOD#12:45:00'))));                // CHECK-NEXT: dt_wrong_formatter=1970-01-01-00:00:00
    printf('tod_minute=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:60:00'))));                      // CHECK-NEXT: tod_minute=00:00:00
    printf('tod_hour=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#25:00:00'))));                        // CHECK-NEXT: tod_hour=00:00:00
    printf('tod_wrong_formatter=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('D#2024-01-01'))));             // CHECK-NEXT: tod_wrong_formatter=00:00:00

    printf('date_low_range=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#1969-12-31'))));                // CHECK-NEXT: date_low_range=1970-01-01
    printf('date_high_range=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2106-02-08'))));               // CHECK-NEXT: date_high_range=1970-01-01
    printf('date_high_range_accept=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2106-02-07'))));        // CHECK-NEXT: date_high_range_accept=2106-02-07
    printf('dt_low_range=%s$N', REF(DT_TO_STRING(STRING_TO_DT('DT#1969-12-31-23:59:59'))));            // CHECK-NEXT: dt_low_range=1970-01-01-00:00:00

    // Unprefixed ISO acceptance (D9).
    printf('date_unprefixed=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('2024-01-01'))));                 // CHECK-NEXT: date_unprefixed=2024-01-01
    printf('tod_unprefixed=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('12:00:00'))));                      // CHECK-NEXT: tod_unprefixed=12:00:00
    printf('tod_unprefixed_fraction=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('12:00:00.500'))));         // CHECK-NEXT: tod_unprefixed_fraction=12:00:00.500
    printf('dt_unprefixed_dash=%s$N', REF(DT_TO_STRING(STRING_TO_DT('2024-01-01-12:00:00'))));         // CHECK-NEXT: dt_unprefixed_dash=2024-01-01-12:00:00
    printf('dt_unprefixed_t=%s$N', REF(DT_TO_STRING(STRING_TO_DT('2024-01-01T12:00:00'))));            // CHECK-NEXT: dt_unprefixed_t=2024-01-01-12:00:00

    // Whitespace handling parity (D12).
    printf('date_trailing_space=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-01-01 '))));           // CHECK-NEXT: date_trailing_space=2024-01-01
    printf('date_trailing_crlf=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D#2024-01-01$R$N'))));          // CHECK-NEXT: date_trailing_crlf=2024-01-01
    printf('int_leading_lf=%d$N', STRING_TO_INT('$N12'));                                               // CHECK-NEXT: int_leading_lf=12
    printf('ulint_trailing_space=%d$N', STRING_TO_ULINT('12 '));                                       // CHECK-NEXT: ulint_trailing_space=12
    printf('tod_trailing_tab=%s$N', REF(TOD_TO_STRING(STRING_TO_TOD('TOD#12:00:00$T'))));               // CHECK-NEXT: tod_trailing_tab=12:00:00
    printf('time_internal_space=%s$N', REF(TIME_TO_STRING(STRING_TO_TIME('T#1h 30m'))));                // CHECK-NEXT: time_internal_space=1h30m
    printf('date_internal_space=%s$N', REF(DATE_TO_STRING(STRING_TO_DATE('D# 2024-01-01'))));           // CHECK-NEXT: date_internal_space=1970-01-01
END_FUNCTION
