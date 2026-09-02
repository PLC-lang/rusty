//! `*_TO_STRING` conversions for BOOL, integer and bit-string widths, and date/time types.

use chrono::TimeZone;
use std::io::Write;

const STRING_CAPACITY: usize = 2048;
const BOOL_STRING_LENGTH: usize = 5;
const BYTE_STRING_LENGTH: usize = 3;
const USINT_STRING_LENGTH: usize = 3;
const WORD_STRING_LENGTH: usize = 5;
const UINT_STRING_LENGTH: usize = 5;
const DWORD_STRING_LENGTH: usize = 10;
const UDINT_STRING_LENGTH: usize = 10;
const LWORD_STRING_LENGTH: usize = 20;
const ULINT_STRING_LENGTH: usize = 20;
const SINT_STRING_LENGTH: usize = 4;
const INT_STRING_LENGTH: usize = 6;
const DINT_STRING_LENGTH: usize = 11;
const LINT_STRING_LENGTH: usize = 20;
const TIME_STRING_LENGTH: usize = 19;
const LTIME_STRING_LENGTH: usize = 37;
const DATE_STRING_LENGTH: usize = 12;
const DT_STRING_LENGTH: usize = 22;
const TOD_STRING_LENGTH: usize = 16;
const STRING_TERMINATOR_LENGTH: usize = 1;
const NANOS_PER_MILLISECOND: u64 = 1_000_000;
const NANOS_PER_SECOND: u64 = 1_000_000_000;

/// Formats `args` into `dest` and appends the null terminator the IEC string layout requires.
/// Output that does not fit is truncated to `capacity - 1` content bytes. Returns the number of
/// content bytes written, excluding the terminator. Writers must not rely on the destination being
/// zero-initialized; the buffer may hold arbitrary bytes.
///
/// # Safety
/// `dest` must have room for `capacity` bytes.
unsafe fn write_terminated(dest: *mut u8, capacity: usize, args: std::fmt::Arguments) -> usize {
    let content = core::slice::from_raw_parts_mut(dest, capacity - 1);
    let mut cursor = std::io::Cursor::new(content);
    // An error here means the output was cut off at the end of the buffer.
    let _ = cursor.write_fmt(args);
    let written = cursor.position() as usize;
    *dest.add(written) = 0;
    written
}

macro_rules! to_string_ext {
    ($name:ident, $ty:ty, $capacity:expr) => {
        /// # Safety
        /// `dest` must have room for the conversion result and its null terminator.
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(input: $ty, dest: *mut u8) -> i32 {
            write_terminated(dest, $capacity, format_args!("{input}"));
            0
        }
    };
}

to_string_ext!(BYTE_TO_STRING_EXT, u8, BYTE_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(USINT_TO_STRING_EXT, u8, USINT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(WORD_TO_STRING_EXT, u16, WORD_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(UINT_TO_STRING_EXT, u16, UINT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(DWORD_TO_STRING_EXT, u32, DWORD_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(UDINT_TO_STRING_EXT, u32, UDINT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(LWORD_TO_STRING_EXT, u64, LWORD_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(ULINT_TO_STRING_EXT, u64, ULINT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(SINT_TO_STRING_EXT, i8, SINT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(INT_TO_STRING_EXT, i16, INT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(DINT_TO_STRING_EXT, i32, DINT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);
to_string_ext!(LINT_TO_STRING_EXT, i64, LINT_STRING_LENGTH + STRING_TERMINATOR_LENGTH);

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LREAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    if input.abs() < 1e14 {
        write_terminated(dest, STRING_CAPACITY, format_args!("{input:.6}"));
    } else {
        write_terminated(dest, STRING_CAPACITY, format_args!("{input:.6e}"));
    }
    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn REAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    if input.abs() < 1e6 {
        write_terminated(dest, STRING_CAPACITY, format_args!("{input:.6}"));
    } else {
        write_terminated(dest, STRING_CAPACITY, format_args!("{input:.6e}"));
    }
    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn BOOL_TO_STRING(dest: *mut u8, input: bool) {
    write_terminated(
        dest,
        BOOL_STRING_LENGTH + STRING_TERMINATOR_LENGTH,
        format_args!("{}", if input { "TRUE" } else { "FALSE" }),
    );
}

fn duration_components(timestamp_nanos: u64) -> [(u64, &'static str); 7] {
    let days = timestamp_nanos / (24 * 60 * 60 * NANOS_PER_SECOND);
    let remainder = timestamp_nanos % (24 * 60 * 60 * NANOS_PER_SECOND);
    let hours = remainder / (60 * 60 * NANOS_PER_SECOND);
    let remainder = remainder % (60 * 60 * NANOS_PER_SECOND);
    let minutes = remainder / (60 * NANOS_PER_SECOND);
    let remainder = remainder % (60 * NANOS_PER_SECOND);
    let seconds = remainder / NANOS_PER_SECOND;
    let remainder = remainder % NANOS_PER_SECOND;
    let millis = remainder / NANOS_PER_MILLISECOND;
    let remainder = remainder % NANOS_PER_MILLISECOND;
    let micros = remainder / 1_000;
    let nanos = remainder % 1_000;
    [(days, "d"), (hours, "h"), (minutes, "m"), (seconds, "s"), (millis, "ms"), (micros, "us"), (nanos, "ns")]
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
fn format_duration(input_nanos: u64, prefix: &str, zero_unit: &str) -> String {
    let mut value = String::from(prefix);
    for (amount, unit) in duration_components(input_nanos) {
        if amount != 0 {
            value.push_str(&amount.to_string());
            value.push_str(unit);
        }
    }
    if input_nanos == 0 {
        value.push('0');
        value.push_str(zero_unit);
    }
    value
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TIME_TO_STRING(dest: *mut u8, input: i32) {
    let input_nanos = (input as u32 as u64) * NANOS_PER_MILLISECOND;
    let value = format_duration(input_nanos, "T#", "ms");
    write_terminated(dest, TIME_STRING_LENGTH + STRING_TERMINATOR_LENGTH, format_args!("{value}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LTIME_TO_STRING(dest: *mut u8, input: i64) {
    let value = format_duration(input as u64, "LTIME#", "ns");
    write_terminated(dest, LTIME_STRING_LENGTH + STRING_TERMINATOR_LENGTH, format_args!("{value}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
unsafe fn write_date_time_to_string(input_nanos: i64, prefix: &str, dest: *mut u8) {
    let datetime = chrono::Utc.timestamp_nanos(input_nanos);
    write_terminated(
        dest,
        STRING_CAPACITY,
        format_args!("{prefix}{}-{}", datetime.date_naive(), datetime.time()),
    );
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DT_TO_STRING(dest: *mut u8, input: i32) {
    let datetime = chrono::Utc.timestamp_nanos((input as u32 as i64) * NANOS_PER_SECOND as i64);
    write_terminated(
        dest,
        DT_STRING_LENGTH + STRING_TERMINATOR_LENGTH,
        format_args!("DT#{}-{}", datetime.date_naive(), datetime.time()),
    );
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LDT_TO_STRING(dest: *mut u8, input: i64) {
    write_date_time_to_string(input, "", dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
unsafe fn write_date_to_string(input_nanos: i64, prefix: &str, dest: *mut u8) {
    let date = chrono::Utc.timestamp_nanos(input_nanos).date_naive();
    write_terminated(dest, STRING_CAPACITY, format_args!("{prefix}{date}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DATE_TO_STRING(dest: *mut u8, input: i32) {
    let date = chrono::Utc.timestamp_nanos((input as u32 as i64) * NANOS_PER_SECOND as i64).date_naive();
    write_terminated(dest, DATE_STRING_LENGTH + STRING_TERMINATOR_LENGTH, format_args!("D#{date}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LDATE_TO_STRING(dest: *mut u8, input: i64) {
    write_date_to_string(input, "", dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
unsafe fn write_time_of_day_to_string(input_nanos: i64, prefix: &str, dest: *mut u8) {
    let time = chrono::Utc.timestamp_nanos(input_nanos).time();
    write_terminated(dest, STRING_CAPACITY, format_args!("{prefix}{time}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TOD_TO_STRING(dest: *mut u8, input: i32) {
    let time = chrono::Utc.timestamp_nanos((input as u32 as i64) * NANOS_PER_MILLISECOND as i64).time();
    write_terminated(dest, TOD_STRING_LENGTH + STRING_TERMINATOR_LENGTH, format_args!("TOD#{time}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LTOD_TO_STRING(dest: *mut u8, input: i64) {
    write_time_of_day_to_string(input, "LTOD#", dest);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn terminated_str(buffer: &[u8]) -> &str {
        let length = buffer.iter().position(|byte| *byte == 0).unwrap();
        std::str::from_utf8(&buffer[..length]).unwrap()
    }

    #[test]
    fn formats_requested_values() {
        let mut dest = [0_u8; STRING_CAPACITY];
        unsafe { BOOL_TO_STRING(dest.as_mut_ptr(), true) };
        assert_eq!("TRUE", terminated_str(&dest));
        unsafe { BYTE_TO_STRING_EXT(u8::MAX, dest.as_mut_ptr()) };
        assert_eq!("255", terminated_str(&dest));
        unsafe { USINT_TO_STRING_EXT(u8::MAX, dest.as_mut_ptr()) };
        assert_eq!("255", terminated_str(&dest));
        unsafe { WORD_TO_STRING_EXT(u16::MAX, dest.as_mut_ptr()) };
        assert_eq!("65535", terminated_str(&dest));
        unsafe { UINT_TO_STRING_EXT(u16::MAX, dest.as_mut_ptr()) };
        assert_eq!("65535", terminated_str(&dest));
        unsafe { DWORD_TO_STRING_EXT(u32::MAX, dest.as_mut_ptr()) };
        assert_eq!("4294967295", terminated_str(&dest));
        unsafe { UDINT_TO_STRING_EXT(u32::MAX, dest.as_mut_ptr()) };
        assert_eq!("4294967295", terminated_str(&dest));
        unsafe { LWORD_TO_STRING_EXT(u64::MAX, dest.as_mut_ptr()) };
        assert_eq!("18446744073709551615", terminated_str(&dest));
        unsafe { ULINT_TO_STRING_EXT(u64::MAX, dest.as_mut_ptr()) };
        assert_eq!("18446744073709551615", terminated_str(&dest));
        unsafe { SINT_TO_STRING_EXT(i8::MIN, dest.as_mut_ptr()) };
        assert_eq!("-128", terminated_str(&dest));
        unsafe { TIME_TO_STRING(dest.as_mut_ptr(), u32::MAX as i32) };
        assert_eq!("T#49d17h2m47s295ms", terminated_str(&dest));
        unsafe { LTIME_TO_STRING(dest.as_mut_ptr(), -1) };
        assert_eq!("LTIME#213503d23h34m33s709ms551us615ns", terminated_str(&dest));
        unsafe { DATE_TO_STRING(dest.as_mut_ptr(), u32::MAX as i32) };
        assert_eq!("D#2106-02-07", terminated_str(&dest));
        unsafe { DT_TO_STRING(dest.as_mut_ptr(), u32::MAX as i32) };
        assert_eq!("DT#2106-02-07-06:28:15", terminated_str(&dest));
        unsafe { TOD_TO_STRING(dest.as_mut_ptr(), 86_399_999) };
        assert_eq!("TOD#23:59:59.999", terminated_str(&dest));
    }

    #[test]
    fn conversions_terminate_results_in_dirty_buffers() {
        let mut dest = [0xAA_u8; STRING_CAPACITY];
        let dest_ptr = dest.as_mut_ptr();

        unsafe { BYTE_TO_STRING_EXT(42, dest_ptr) };
        assert_eq!("42", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LWORD_TO_STRING_EXT(7, dest_ptr) };
        assert_eq!("7", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LINT_TO_STRING_EXT(-12, dest_ptr) };
        assert_eq!("-12", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LREAL_TO_STRING_EXT(2.5, dest_ptr) };
        assert_eq!("2.500000", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LTIME_TO_STRING(dest_ptr, (2 * 3_600 + 3) * NANOS_PER_SECOND as i64) };
        assert_eq!("LTIME#2h3s", terminated_str(&dest));
    }

    #[test]
    fn byte_to_string_conversion() {
        let mut dest = [0_u8; STRING_CAPACITY];

        unsafe { BYTE_TO_STRING_EXT(0b1010_1010, dest.as_mut_ptr()) };

        assert_eq!("170", terminated_str(&dest));
    }

    #[test]
    fn lword_to_string_conversion() {
        let mut dest = [0_u8; STRING_CAPACITY];
        let input = 0xFF_00_FF_00_00_FF_00_FF_u64;

        unsafe { LWORD_TO_STRING_EXT(input, dest.as_mut_ptr()) };

        assert_eq!("18374966855153418495", terminated_str(&dest));
    }

    #[test]
    fn lint_to_string_conversion() {
        let mut dest = [0_u8; STRING_CAPACITY];

        unsafe { LINT_TO_STRING_EXT(100_200_300_400_500, dest.as_mut_ptr()) };

        assert_eq!("100200300400500", terminated_str(&dest));
    }

    #[test]
    fn lreal_to_string_conversion() {
        let mut dest = [0_u8; STRING_CAPACITY];

        unsafe { LREAL_TO_STRING_EXT(10_230.2321123121, dest.as_mut_ptr()) };
        assert_eq!("10230.232112", terminated_str(&dest));

        unsafe { LREAL_TO_STRING_EXT(-10_230.2321123121, dest.as_mut_ptr()) };
        assert_eq!("-10230.232112", terminated_str(&dest));

        unsafe { LREAL_TO_STRING_EXT(99_999_999_999_999.25, dest.as_mut_ptr()) };
        assert_eq!("99999999999999.250000", terminated_str(&dest));

        unsafe { LREAL_TO_STRING_EXT(123_456_789_123_456.13, dest.as_mut_ptr()) };
        assert_eq!("1.234568e14", terminated_str(&dest));
    }

    #[test]
    fn lreal_to_string_uses_scientific_notation_for_huge_negative_values() {
        let mut dest = [0xAA_u8; STRING_CAPACITY];

        unsafe { LREAL_TO_STRING_EXT(-1.0e300, dest.as_mut_ptr()) };
        assert_eq!("-1.000000e300", terminated_str(&dest));

        unsafe { LREAL_TO_STRING_EXT(-99_999_999_999_999.25, dest.as_mut_ptr()) };
        assert_eq!("-99999999999999.250000", terminated_str(&dest));

        unsafe { LREAL_TO_STRING_EXT(f64::INFINITY, dest.as_mut_ptr()) };
        assert_eq!("inf", terminated_str(&dest));

        unsafe { LREAL_TO_STRING_EXT(f64::NEG_INFINITY, dest.as_mut_ptr()) };
        assert_eq!("-inf", terminated_str(&dest));

        unsafe { LREAL_TO_STRING_EXT(f64::NAN, dest.as_mut_ptr()) };
        assert_eq!("NaN", terminated_str(&dest));
    }

    #[test]
    fn real_to_string_uses_scientific_notation_by_magnitude() {
        let mut dest = [0xAA_u8; STRING_CAPACITY];

        unsafe { REAL_TO_STRING_EXT(-1.5e7, dest.as_mut_ptr()) };
        assert_eq!("-1.500000e7", terminated_str(&dest));

        unsafe { REAL_TO_STRING_EXT(1.5e7, dest.as_mut_ptr()) };
        assert_eq!("1.500000e7", terminated_str(&dest));

        unsafe { REAL_TO_STRING_EXT(-999_999.25, dest.as_mut_ptr()) };
        assert_eq!("-999999.250000", terminated_str(&dest));

        unsafe { REAL_TO_STRING_EXT(f64::INFINITY, dest.as_mut_ptr()) };
        assert_eq!("inf", terminated_str(&dest));

        unsafe { REAL_TO_STRING_EXT(f64::NEG_INFINITY, dest.as_mut_ptr()) };
        assert_eq!("-inf", terminated_str(&dest));

        unsafe { REAL_TO_STRING_EXT(f64::NAN, dest.as_mut_ptr()) };
        assert_eq!("NaN", terminated_str(&dest));
    }

    #[test]
    fn write_terminated_truncates_overlong_output() {
        let mut dest = [0xAA_u8; 16];

        let written = unsafe { write_terminated(dest.as_mut_ptr(), 16, format_args!("{:x<30}", "abc")) };

        assert_eq!(15, written);
        assert_eq!("abcxxxxxxxxxxxx", terminated_str(&dest));
    }

    #[test]
    fn date_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 2, 123_456_789))
            .unwrap();
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();
        let mut dest = [0_u8; STRING_CAPACITY];
        let dest_ptr = dest.as_mut_ptr();

        unsafe { LDATE_TO_STRING(dest_ptr, timestamp) };
        assert_eq!("1982-12-15", terminated_str(&dest));
    }

    #[test]
    fn dt_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 2, 123_456_789))
            .unwrap();
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();
        let mut dest = [0_u8; STRING_CAPACITY];

        unsafe { LDT_TO_STRING(dest.as_mut_ptr(), timestamp) };
        assert_eq!("1982-12-15-10:10:02.123456789", terminated_str(&dest));
    }

    #[test]
    fn tod_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 2, 123_456_789))
            .unwrap();
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();
        let mut dest = [0_u8; STRING_CAPACITY];

        unsafe { LTOD_TO_STRING(dest.as_mut_ptr(), timestamp) };
        assert_eq!("LTOD#10:10:02.123456789", terminated_str(&dest));
    }

    #[test]
    fn time_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 2, 123_456_789))
            .unwrap();
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();
        let mut dest = [0_u8; STRING_CAPACITY];

        unsafe { LTIME_TO_STRING(dest.as_mut_ptr(), timestamp) };
        assert_eq!("LTIME#4731d10h10m2s123ms456us789ns", terminated_str(&dest));
    }

    #[test]
    fn long_temporal_extremes_do_not_panic() {
        let mut dest = [0_u8; STRING_CAPACITY];

        for input in [i64::MIN, i64::MAX] {
            unsafe { LTIME_TO_STRING(dest.as_mut_ptr(), input) };
            unsafe { LDT_TO_STRING(dest.as_mut_ptr(), input) };
            unsafe { LDATE_TO_STRING(dest.as_mut_ptr(), input) };
            unsafe { LTOD_TO_STRING(dest.as_mut_ptr(), input) };
        }
    }
}
