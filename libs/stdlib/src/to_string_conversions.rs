//! `*_TO_STRING` conversions for BOOL, integer and bit-string widths, and date/time types.

use chrono::TimeZone;
use std::io::Write;

const STRING_CAPACITY: usize = 2048;
const NANOS_PER_MILLISECOND: u64 = 1_000_000;
const NANOS_PER_SECOND: u64 = 1_000_000_000;

/// # Safety
/// `dest` must have room for `STRING_CAPACITY` bytes.
unsafe fn write_terminated(dest: *mut u8, args: std::fmt::Arguments) {
    let content = core::slice::from_raw_parts_mut(dest, STRING_CAPACITY - 1);
    let mut cursor = std::io::Cursor::new(content);
    let _ = cursor.write_fmt(args);
    *dest.add(cursor.position() as usize) = 0;
}

macro_rules! to_string_ext {
    ($name:ident, $ty:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(input: $ty, dest: *mut u8) -> i32 {
            write_terminated(dest, format_args!("{input}"));
            0
        }
    };
}

to_string_ext!(BYTE_TO_STRING_EXT, u8);
to_string_ext!(WORD_TO_STRING_EXT, u16);
to_string_ext!(LWORD_TO_STRING_EXT, u64);
to_string_ext!(SINT_TO_STRING_EXT, i8);
to_string_ext!(INT_TO_STRING_EXT, i16);
to_string_ext!(LINT_TO_STRING_EXT, i64);

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LREAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    if input.abs() < 1e14 {
        write_terminated(dest, format_args!("{input:.6}"));
    } else {
        write_terminated(dest, format_args!("{input:.6e}"));
    }
    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn REAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    if input.abs() < 1e6 {
        write_terminated(dest, format_args!("{input:.6}"));
    } else {
        write_terminated(dest, format_args!("{input:.6e}"));
    }
    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn BOOL_TO_STRING(dest: *mut u8, input: bool) {
    write_terminated(dest, format_args!("{}", if input { "TRUE" } else { "FALSE" }));
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
unsafe fn write_duration_to_string(input_nanos: u64, prefix: &str, zero_unit: &str, dest: *mut u8) {
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
    write_terminated(dest, format_args!("{value}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TIME_TO_STRING(dest: *mut u8, input: i32) {
    write_duration_to_string((input as u32 as u64) * NANOS_PER_MILLISECOND, "T#", "ms", dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LTIME_TO_STRING(dest: *mut u8, input: i64) {
    write_duration_to_string(input as u64, "LTIME#", "ns", dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
unsafe fn write_date_time_to_string(input_nanos: i64, prefix: &str, dest: *mut u8) {
    let datetime = chrono::Utc.timestamp_nanos(input_nanos);
    write_terminated(dest, format_args!("{prefix}{}-{}", datetime.date_naive(), datetime.time()));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DT_TO_STRING(dest: *mut u8, input: i32) {
    write_date_time_to_string((input as u32 as i64) * NANOS_PER_SECOND as i64, "DT#", dest);
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
    write_terminated(dest, format_args!("{prefix}{date}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DATE_TO_STRING(dest: *mut u8, input: i32) {
    write_date_to_string((input as u32 as i64) * NANOS_PER_SECOND as i64, "D#", dest);
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
    write_terminated(dest, format_args!("{prefix}{time}"));
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TOD_TO_STRING(dest: *mut u8, input: i32) {
    write_time_of_day_to_string((input as u32 as i64) * NANOS_PER_MILLISECOND as i64, "TOD#", dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LTOD_TO_STRING(dest: *mut u8, input: i64) {
    write_time_of_day_to_string(input, "", dest);
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
        unsafe { WORD_TO_STRING_EXT(u16::MAX, dest.as_mut_ptr()) };
        assert_eq!("65535", terminated_str(&dest));
        unsafe { LWORD_TO_STRING_EXT(u64::MAX, dest.as_mut_ptr()) };
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
}
