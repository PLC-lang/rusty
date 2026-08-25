// Full roundtrip regression coverage for STRING_TO_* conversion behavior.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    // Never faults and rejected input returns zero-value defaults.
    IF STRING_TO_UDINT('') = UDINT#0 THEN printf('udint_empty_zero_ok$N'); END_IF                     // CHECK: udint_empty_zero_ok
    IF STRING_TO_DATE('not a date') = DATE#1970-01-01 THEN printf('date_garbage_zero_ok$N'); END_IF  // CHECK: date_garbage_zero_ok
    IF STRING_TO_DT('$00') = DT#1970-01-01-00:00:00 THEN printf('dt_garbage_zero_ok$N'); END_IF       // CHECK: dt_garbage_zero_ok
    IF STRING_TO_TOD('bad') = TOD#00:00:00 THEN printf('tod_garbage_zero_ok$N'); END_IF               // CHECK: tod_garbage_zero_ok
    IF STRING_TO_LTIME('!!!') = LTIME#0s THEN printf('ltime_garbage_zero_ok$N'); END_IF               // CHECK: ltime_garbage_zero_ok
    IF STRING_TO_BOOL('garbage') = FALSE THEN printf('bool_garbage_false_ok$N'); END_IF               // CHECK: bool_garbage_false_ok

    // BOOL behavior (D3).
    IF STRING_TO_BOOL('1') THEN printf('bool_1_true_ok$N'); END_IF                                     // CHECK: bool_1_true_ok
    IF STRING_TO_BOOL('TRUE') THEN printf('bool_TRUE_true_ok$N'); END_IF                               // CHECK: bool_TRUE_true_ok
    IF STRING_TO_BOOL('true') THEN printf('bool_true_true_ok$N'); END_IF                               // CHECK: bool_true_true_ok
    IF STRING_TO_BOOL('  true  ') THEN printf('bool_trimmed_true_ok$N'); END_IF                        // CHECK: bool_trimmed_true_ok
    IF NOT STRING_TO_BOOL('0') THEN printf('bool_0_false_ok$N'); END_IF                                // CHECK: bool_0_false_ok
    IF NOT STRING_TO_BOOL('TRUEX') THEN printf('bool_suffix_reject_ok$N'); END_IF                      // CHECK: bool_suffix_reject_ok

    // One parse rule across integer widths (D2), including all requested widths.
    IF STRING_TO_BYTE('16#FF') = BYTE#16#FF THEN printf('byte_hex_ok$N'); END_IF                       // CHECK: byte_hex_ok
    IF STRING_TO_WORD('16#FF') = WORD#16#00FF THEN printf('word_hex_ok$N'); END_IF                     // CHECK: word_hex_ok
    IF STRING_TO_DWORD('16#FF') = DWORD#16#000000FF THEN printf('dword_hex_ok$N'); END_IF              // CHECK: dword_hex_ok
    IF STRING_TO_LWORD('16#FF') = LWORD#16#00000000000000FF THEN printf('lword_hex_ok$N'); END_IF      // CHECK: lword_hex_ok
    IF STRING_TO_SINT('-1') = SINT#-1 THEN printf('sint_neg_ok$N'); END_IF                              // CHECK: sint_neg_ok
    IF STRING_TO_USINT('12abc') = USINT#0 THEN printf('usint_partial_reject_ok$N'); END_IF             // CHECK: usint_partial_reject_ok
    IF STRING_TO_INT('-1') = INT#-1 THEN printf('int_neg_ok$N'); END_IF                                 // CHECK: int_neg_ok
    IF STRING_TO_UINT('12abc') = UINT#0 THEN printf('uint_partial_reject_ok$N'); END_IF                // CHECK: uint_partial_reject_ok
    IF STRING_TO_UDINT('12abc') = UDINT#0 THEN printf('udint_partial_reject_ok$N'); END_IF             // CHECK: udint_partial_reject_ok
    IF STRING_TO_ULINT('12abc') = ULINT#0 THEN printf('ulint_partial_reject_ok$N'); END_IF             // CHECK: ulint_partial_reject_ok
    IF STRING_TO_UDINT('1.9') = UDINT#1 THEN printf('udint_fraction_trunc_ok$N'); END_IF               // CHECK: udint_fraction_trunc_ok
    IF STRING_TO_ULINT('1.9') = ULINT#1 THEN printf('ulint_fraction_trunc_ok$N'); END_IF               // CHECK: ulint_fraction_trunc_ok
    IF STRING_TO_UDINT('  12  ') = UDINT#12 THEN printf('udint_trim_ok$N'); END_IF                     // CHECK: udint_trim_ok
    IF STRING_TO_ULINT('  12  ') = ULINT#12 THEN printf('ulint_trim_ok$N'); END_IF                     // CHECK: ulint_trim_ok
    IF STRING_TO_BYTE('-1') = BYTE#0 THEN printf('byte_negative_reject_ok$N'); END_IF                  // CHECK: byte_negative_reject_ok

    // Fractional durations (D4), negative duration rejection (D5), and 67ms fallback removal (D6).
    IF STRING_TO_TIME('T#1.5s') = TIME#1s500ms THEN printf('time_fraction_1_5s_ok$N'); END_IF         // CHECK: time_fraction_1_5s_ok
    IF STRING_TO_TIME('T#0.5s') = TIME#500ms THEN printf('time_fraction_0_5s_ok$N'); END_IF            // CHECK: time_fraction_0_5s_ok
    IF STRING_TO_TIME('T#2.75s') = TIME#2s750ms THEN printf('time_fraction_2_75s_ok$N'); END_IF        // CHECK: time_fraction_2_75s_ok
    IF STRING_TO_TIME('T#1,5s') = TIME#1s500ms THEN printf('time_fraction_comma_ok$N'); END_IF         // CHECK: time_fraction_comma_ok
    IF STRING_TO_TIME('T#1.0004ms') = TIME#1ms THEN printf('time_fraction_resolution_ok$N'); END_IF    // CHECK: time_fraction_resolution_ok
    IF STRING_TO_LTIME('LTIME#1.5s') = LTIME#1s500ms THEN printf('ltime_fraction_ok$N'); END_IF        // CHECK: ltime_fraction_ok
    IF STRING_TO_TIME('T#-1s') = TIME#0s THEN printf('time_negative_reject_ok$N'); END_IF              // CHECK: time_negative_reject_ok
    IF STRING_TO_LTIME('LTIME#-1s') = LTIME#0s THEN printf('ltime_negative_reject_ok$N'); END_IF       // CHECK: ltime_negative_reject_ok
    IF STRING_TO_TIME('') = TIME#0s THEN printf('time_empty_zero_ok$N'); END_IF                         // CHECK: time_empty_zero_ok
    IF STRING_TO_TIME('abc') = TIME#0s THEN printf('time_garbage_zero_ok$N'); END_IF                    // CHECK: time_garbage_zero_ok
    IF STRING_TO_TIME('1s') = TIME#0s THEN printf('time_missing_prefix_zero_ok$N'); END_IF              // CHECK: time_missing_prefix_zero_ok
    IF STRING_TO_TIME('LTIME#1s') = TIME#0s THEN printf('time_wrong_prefix_zero_ok$N'); END_IF          // CHECK: time_wrong_prefix_zero_ok
    IF STRING_TO_TIME('TOD#12:45:00') = TIME#0s THEN printf('time_wrong_formatter_reject_ok$N'); END_IF // CHECK: time_wrong_formatter_reject_ok
    IF STRING_TO_TIME('T#67ms') = TIME#67ms THEN printf('time_67ms_explicit_ok$N'); END_IF              // CHECK: time_67ms_explicit_ok
    IF STRING_TO_LTIME('T#1s') = LTIME#0s THEN printf('ltime_wrong_formatter_reject_ok$N'); END_IF       // CHECK: ltime_wrong_formatter_reject_ok

    // Date/time validity and range behavior (D7, D8).
    IF STRING_TO_DATE('D#2024-02-30') = DATE#1970-01-01 THEN printf('date_impossible_reject_ok$N'); END_IF      // CHECK: date_impossible_reject_ok
    IF STRING_TO_DATE('D#2024-13-01') = DATE#1970-01-01 THEN printf('date_month_reject_ok$N'); END_IF           // CHECK: date_month_reject_ok
    IF STRING_TO_DATE('D#2023-02-29') = DATE#1970-01-01 THEN printf('date_nonleap_reject_ok$N'); END_IF         // CHECK: date_nonleap_reject_ok
    IF STRING_TO_DATE('D#2024-02-29') = DATE#2024-02-29 THEN printf('date_leap_accept_ok$N'); END_IF            // CHECK: date_leap_accept_ok
    IF STRING_TO_DATE('TOD#12:45:00') = DATE#1970-01-01 THEN printf('date_wrong_formatter_reject_ok$N'); END_IF // CHECK: date_wrong_formatter_reject_ok
    IF STRING_TO_DT('DT#2024-01-01-25:00:00') = DT#1970-01-01-00:00:00 THEN printf('dt_hour_reject_ok$N'); END_IF // CHECK: dt_hour_reject_ok
    IF STRING_TO_DT('TOD#12:45:00') = DT#1970-01-01-00:00:00 THEN printf('dt_wrong_formatter_reject_ok$N'); END_IF // CHECK: dt_wrong_formatter_reject_ok
    IF STRING_TO_TOD('TOD#12:60:00') = TOD#00:00:00 THEN printf('tod_minute_reject_ok$N'); END_IF               // CHECK: tod_minute_reject_ok
    IF STRING_TO_TOD('TOD#25:00:00') = TOD#00:00:00 THEN printf('tod_hour_reject_ok$N'); END_IF                 // CHECK: tod_hour_reject_ok
    IF STRING_TO_TOD('D#2024-01-01') = TOD#00:00:00 THEN printf('tod_wrong_formatter_reject_ok$N'); END_IF    // CHECK: tod_wrong_formatter_reject_ok

    IF STRING_TO_DATE('D#1969-12-31') = DATE#1970-01-01 THEN printf('date_low_range_reject_ok$N'); END_IF       // CHECK: date_low_range_reject_ok
    IF STRING_TO_DATE('D#2106-02-08') = DATE#1970-01-01 THEN printf('date_high_range_reject_ok$N'); END_IF      // CHECK: date_high_range_reject_ok
    IF STRING_TO_DATE('D#2106-02-07') = DATE#2106-02-07 THEN printf('date_high_range_accept_ok$N'); END_IF      // CHECK: date_high_range_accept_ok
    IF STRING_TO_DT('DT#1969-12-31-23:59:59') = DT#1970-01-01-00:00:00 THEN printf('dt_low_range_reject_ok$N'); END_IF // CHECK: dt_low_range_reject_ok

    // Unprefixed ISO acceptance (D9).
    IF STRING_TO_DATE('2024-01-01') = STRING_TO_DATE('D#2024-01-01') THEN printf('date_unprefixed_ok$N'); END_IF              // CHECK: date_unprefixed_ok
    IF STRING_TO_TOD('12:00:00') = TOD#12:00:00 THEN printf('tod_unprefixed_ok$N'); END_IF                                    // CHECK: tod_unprefixed_ok
    IF STRING_TO_TOD('12:00:00.500') = TOD#12:00:00.500 THEN printf('tod_unprefixed_fraction_ok$N'); END_IF                   // CHECK: tod_unprefixed_fraction_ok
    IF STRING_TO_DT('2024-01-01-12:00:00') = DT#2024-01-01-12:00:00 THEN printf('dt_unprefixed_dash_ok$N'); END_IF            // CHECK: dt_unprefixed_dash_ok
    IF STRING_TO_DT('2024-01-01T12:00:00') = DT#2024-01-01-12:00:00 THEN printf('dt_unprefixed_t_ok$N'); END_IF               // CHECK: dt_unprefixed_t_ok

    // Whitespace handling parity (D12).
    IF STRING_TO_DATE('D#2024-01-01 ') = DATE#2024-01-01 THEN printf('date_trailing_space_ok$N'); END_IF                      // CHECK: date_trailing_space_ok
    IF STRING_TO_DATE('D#2024-01-01$R$N') = DATE#2024-01-01 THEN printf('date_trailing_crlf_ok$N'); END_IF                    // CHECK: date_trailing_crlf_ok
    IF STRING_TO_INT('$N12') = INT#12 THEN printf('int_leading_lf_ok$N'); END_IF                                               // CHECK: int_leading_lf_ok
    IF STRING_TO_ULINT('12 ') = ULINT#12 THEN printf('ulint_trailing_space_ok$N'); END_IF                                      // CHECK: ulint_trailing_space_ok
    IF STRING_TO_TOD('TOD#12:00:00$T') = TOD#12:00:00 THEN printf('tod_trailing_tab_ok$N'); END_IF                            // CHECK: tod_trailing_tab_ok
    IF STRING_TO_TIME('T#1h 30m') = TIME#1h30m THEN printf('time_internal_space_ok$N'); END_IF                                 // CHECK: time_internal_space_ok
    IF STRING_TO_DATE('D# 2024-01-01') = DATE#1970-01-01 THEN printf('date_internal_space_reject_ok$N'); END_IF               // CHECK: date_internal_space_reject_ok
END_FUNCTION
