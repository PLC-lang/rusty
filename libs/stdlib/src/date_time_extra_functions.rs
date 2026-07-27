use chrono::{Datelike, NaiveDate, TimeZone, Timelike};

const MILLIS_PER_SECOND: u32 = 1_000;
const MILLIS_PER_DAY: u32 = 60 * 60 * 24 * MILLIS_PER_SECOND;
const NANOS_PER_MILLISECOND: i64 = 1_000 * 1_000;
const NANOS_PER_SECOND: i64 = 1_000 * 1_000 * 1_000;

fn dt_from_epoch_seconds(seconds: u32) -> chrono::DateTime<chrono::Utc> {
    chrono::Utc.timestamp_opt(seconds as i64, 0).single().expect("Out of range")
}

fn split_tod_fields(millis: u32) -> (u32, u32, u32, u32) {
    let total_millis = millis % MILLIS_PER_DAY;
    let hour = total_millis / 3_600_000;
    let minute = (total_millis / 60_000) % 60;
    let second = (total_millis / MILLIS_PER_SECOND) % 60;
    let millisecond = total_millis % MILLIS_PER_SECOND;

    (hour, minute, second, millisecond)
}

fn split_ltod_fields(nanos: i64) -> (u32, u32, u32, u32) {
    let millis = (nanos / NANOS_PER_MILLISECOND) as u32;

    split_tod_fields(millis)
}

fn split_ldt_fields(nanos: i64) -> (i32, u32, u32, u32, u32, u32, u32) {
    let dt = chrono::Utc.timestamp_nanos(nanos);

    (dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second(), dt.timestamp_subsec_millis())
}

/// .
/// Concatenates DATE and TOD to DT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE_TOD(in1: u32, in2: u32) -> u32 {
    in1 + in2 / MILLIS_PER_SECOND
}

/// .
/// Concatenates DATE and LTOD to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE_LTOD(in1: u32, in2: i64) -> i64 {
    (in1 as i64) * NANOS_PER_SECOND + in2
}

/// .
/// Concatenates year, month and day of type INT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__INT(in1: i16, in2: i16, in3: i16) -> u32 {
    concat_date(in1.into(), in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type UINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UINT(in1: u16, in2: u16, in3: u16) -> u32 {
    concat_date(in1.into(), in2.into(), in3.into())
}

/// .
/// Concatenates year, month and day of type DINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__DINT(in1: i32, in2: i32, in3: i32) -> u32 {
    concat_date(in1, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type UDINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UDINT(in1: u32, in2: u32, in3: u32) -> u32 {
    concat_date(in1 as i32, in2, in3)
}

/// .
/// Concatenates year, month and day of type LINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__LINT(in1: i64, in2: i64, in3: i64) -> u32 {
    concat_date(in1 as i32, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type ULINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__ULINT(in1: u64, in2: u64, in3: u64) -> u32 {
    concat_date(in1 as i32, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_date(in1: i32, in2: u32, in3: u32) -> u32 {
    let dt = NaiveDate::from_ymd_opt(in1, in2, in3)
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .expect("Invalid parameters, cannot create date");

    dt.and_utc().timestamp() as u32
}

/// .
/// Concatenates hour, minute, second, millisecond of type SINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__SINT(in1: i8, in2: i8, in3: i8, in4: i8) -> u32 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type USINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__USINT(in1: u8, in2: u8, in3: u8, in4: u8) -> u32 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type INT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__INT(in1: i16, in2: i16, in3: i16, in4: i16) -> u32 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UINT(in1: u16, in2: u16, in3: u16, in4: u16) -> u32 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type DINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__DINT(in1: i32, in2: i32, in3: i32, in4: i32) -> u32 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UDINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UDINT(in1: u32, in2: u32, in3: u32, in4: u32) -> u32 {
    concat_tod(in1, in2, in3, in4)
}

/// .
/// Concatenates hour, minute, second, millisecond of type LINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__LINT(in1: i64, in2: i64, in3: i64, in4: i64) -> u32 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type ULINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__ULINT(in1: u64, in2: u64, in3: u64, in4: u64) -> u32 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type SINT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__SINT(in1: i8, in2: i8, in3: i8, in4: i8) -> i64 {
    concat_ltod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type USINT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__USINT(in1: u8, in2: u8, in3: u8, in4: u8) -> i64 {
    concat_ltod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type INT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__INT(in1: i16, in2: i16, in3: i16, in4: i16) -> i64 {
    concat_ltod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UINT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__UINT(in1: u16, in2: u16, in3: u16, in4: u16) -> i64 {
    concat_ltod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type DINT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__DINT(in1: i32, in2: i32, in3: i32, in4: i32) -> i64 {
    concat_ltod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UDINT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__UDINT(in1: u32, in2: u32, in3: u32, in4: u32) -> i64 {
    concat_ltod(in1, in2, in3, in4)
}

/// .
/// Concatenates hour, minute, second, millisecond of type LINT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__LINT(in1: i64, in2: i64, in3: i64, in4: i64) -> i64 {
    concat_ltod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type ULINT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LTOD__ULINT(in1: u64, in2: u64, in3: u64, in4: u64) -> i64 {
    concat_ltod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates year, month, day, hour, minute, second, millisecond of type INT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LDT__INT(
    in1: i16,
    in2: i16,
    in3: i16,
    in4: i16,
    in5: i16,
    in6: i16,
    in7: i16,
) -> i64 {
    concat_ldt(in1.into(), in2 as u32, in3 as u32, in4 as u32, in5 as u32, in6 as u32, in7 as u32)
}

/// .
/// Concatenates year, month, day, hour, minute, second, millisecond of type UINT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LDT__UINT(
    in1: u16,
    in2: u16,
    in3: u16,
    in4: u16,
    in5: u16,
    in6: u16,
    in7: u16,
) -> i64 {
    concat_ldt(in1.into(), in2.into(), in3.into(), in4.into(), in5.into(), in6.into(), in7.into())
}

/// .
/// Concatenates year, month, day, hour, minute, second, millisecond of type DINT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LDT__DINT(
    in1: i32,
    in2: i32,
    in3: i32,
    in4: i32,
    in5: i32,
    in6: i32,
    in7: i32,
) -> i64 {
    concat_ldt(in1, in2 as u32, in3 as u32, in4 as u32, in5 as u32, in6 as u32, in7 as u32)
}

/// .
/// Concatenates year, month, day, hour, minute, second, millisecond of type UDINT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LDT__UDINT(
    in1: u32,
    in2: u32,
    in3: u32,
    in4: u32,
    in5: u32,
    in6: u32,
    in7: u32,
) -> i64 {
    concat_ldt(in1 as i32, in2, in3, in4, in5, in6, in7)
}

/// .
/// Concatenates year, month, day, hour, minute, second, millisecond of type LINT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LDT__LINT(
    in1: i64,
    in2: i64,
    in3: i64,
    in4: i64,
    in5: i64,
    in6: i64,
    in7: i64,
) -> i64 {
    concat_ldt(in1 as i32, in2 as u32, in3 as u32, in4 as u32, in5 as u32, in6 as u32, in7 as u32)
}

/// .
/// Concatenates year, month, day, hour, minute, second, millisecond of type ULINT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_LDT__ULINT(
    in1: u64,
    in2: u64,
    in3: u64,
    in4: u64,
    in5: u64,
    in6: u64,
    in7: u64,
) -> i64 {
    concat_ldt(in1 as i32, in2 as u32, in3 as u32, in4 as u32, in5 as u32, in6 as u32, in7 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_tod(in1: u32, in2: u32, in3: u32, in4: u32) -> u32 {
    NaiveDate::from_ymd_opt(1970, 1, 1)
        .and_then(|date| date.and_hms_milli_opt(in1, in2, in3, in4))
        .expect("Invalid parameters, cannot create TOD");

    ((in1 * 3_600 + in2 * 60 + in3) * MILLIS_PER_SECOND) + in4
}

/// .
/// Concatenates hour, minute, second, millisecond to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_ltod(in1: u32, in2: u32, in3: u32, in4: u32) -> i64 {
    (concat_tod(in1, in2, in3, in4) as i64) * NANOS_PER_MILLISECOND
}

/// .
/// Concatenates year, month, day, hour, minute, second, millisecond to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_ldt(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
) -> i64 {
    let date = concat_date(year, month, day);
    let tod = concat_ltod(hour, minute, second, millisecond);

    CONCAT_DATE_LTOD(date, tod)
}

/// .
/// Splits DATE into year, month, day of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__INT(in1: u32, out1: &mut i16, out2: &mut i16, out3: &mut i16) -> i16 {
    let date = dt_from_epoch_seconds(in1).date_naive();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as i16;
    *out3 = date.day() as i16;

    0
}

/// .
/// Splits DATE into year, month, day of type UINT
/// Panics on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__UINT(in1: u32, out1: &mut u16, out2: &mut u16, out3: &mut u16) -> i16 {
    let date = dt_from_epoch_seconds(in1).date_naive();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as u16;
    *out3 = date.day() as u16;

    0
}

/// .
/// Splits DATE into year, month, day of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__DINT(in1: u32, out1: &mut i32, out2: &mut i32, out3: &mut i32) -> i16 {
    let date = dt_from_epoch_seconds(in1).date_naive();
    *out1 = date.year();
    *out2 = date.month() as i32;
    *out3 = date.day() as i32;

    0
}

/// .
/// Splits DATE into year, month, day of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__UDINT(in1: u32, out1: &mut u32, out2: &mut u32, out3: &mut u32) -> i16 {
    let date = dt_from_epoch_seconds(in1).date_naive();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month();
    *out3 = date.day();

    0
}

/// .
/// Splits DATE into year, month, day of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__LINT(in1: u32, out1: &mut i64, out2: &mut i64, out3: &mut i64) -> i16 {
    let date = dt_from_epoch_seconds(in1).date_naive();
    // if year does not fit in target data type -> panic
    *out1 = date.year().into();
    *out2 = date.month() as i64;
    *out3 = date.day() as i64;

    0
}

/// .
/// Splits DATE into year, month, day of type ULINT
/// Panics on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__ULINT(in1: u32, out1: &mut u64, out2: &mut u64, out3: &mut u64) -> i16 {
    let date = dt_from_epoch_seconds(in1).date_naive();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as u64;
    *out3 = date.day() as u64;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__INT(
    in1: u32,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
) -> i16 {
    let (hour, minute, second, millisecond) = split_tod_fields(in1);
    *out1 = hour as i16;
    *out2 = minute as i16;
    *out3 = second as i16;
    *out4 = millisecond as i16;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__UINT(
    in1: u32,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
) -> i16 {
    let (hour, minute, second, millisecond) = split_tod_fields(in1);
    *out1 = hour as u16;
    *out2 = minute as u16;
    *out3 = second as u16;
    *out4 = millisecond as u16;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__DINT(
    in1: u32,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
) -> i16 {
    let (hour, minute, second, millisecond) = split_tod_fields(in1);
    *out1 = hour as i32;
    *out2 = minute as i32;
    *out3 = second as i32;
    *out4 = millisecond as i32;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__UDINT(
    in1: u32,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
) -> i16 {
    let (hour, minute, second, millisecond) = split_tod_fields(in1);
    *out1 = hour;
    *out2 = minute;
    *out3 = second;
    *out4 = millisecond;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__LINT(
    in1: u32,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
) -> i16 {
    let (hour, minute, second, millisecond) = split_tod_fields(in1);
    *out1 = hour as i64;
    *out2 = minute as i64;
    *out3 = second as i64;
    *out4 = millisecond as i64;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__ULINT(
    in1: u32,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
) -> i16 {
    let (hour, minute, second, millisecond) = split_tod_fields(in1);
    *out1 = hour as u64;
    *out2 = minute as u64;
    *out3 = second as u64;
    *out4 = millisecond as u64;

    0
}

/// .
/// Splits LTOD into hour, minute, second, millisecond of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LTOD__INT(
    in1: i64,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
) -> i16 {
    let (hour, minute, second, millisecond) = split_ltod_fields(in1);
    *out1 = hour as i16;
    *out2 = minute as i16;
    *out3 = second as i16;
    *out4 = millisecond as i16;

    0
}

/// .
/// Splits LTOD into hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LTOD__UINT(
    in1: i64,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
) -> i16 {
    let (hour, minute, second, millisecond) = split_ltod_fields(in1);
    *out1 = hour as u16;
    *out2 = minute as u16;
    *out3 = second as u16;
    *out4 = millisecond as u16;

    0
}

/// .
/// Splits LTOD into hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LTOD__DINT(
    in1: i64,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
) -> i16 {
    let (hour, minute, second, millisecond) = split_ltod_fields(in1);
    *out1 = hour as i32;
    *out2 = minute as i32;
    *out3 = second as i32;
    *out4 = millisecond as i32;

    0
}

/// .
/// Splits LTOD into hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LTOD__UDINT(
    in1: i64,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
) -> i16 {
    let (hour, minute, second, millisecond) = split_ltod_fields(in1);
    *out1 = hour;
    *out2 = minute;
    *out3 = second;
    *out4 = millisecond;

    0
}

/// .
/// Splits LTOD into hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LTOD__LINT(
    in1: i64,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
) -> i16 {
    let (hour, minute, second, millisecond) = split_ltod_fields(in1);
    *out1 = hour as i64;
    *out2 = minute as i64;
    *out3 = second as i64;
    *out4 = millisecond as i64;

    0
}

/// .
/// Splits LTOD into hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LTOD__ULINT(
    in1: i64,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
) -> i16 {
    let (hour, minute, second, millisecond) = split_ltod_fields(in1);
    *out1 = hour as u64;
    *out2 = minute as u64;
    *out3 = second as u64;
    *out4 = millisecond as u64;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__INT(
    in1: u32,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
    out5: &mut i16,
    out6: &mut i16,
    out7: &mut i16,
) -> i16 {
    let dt = dt_from_epoch_seconds(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as i16;
    *out3 = dt.day() as i16;
    *out4 = dt.hour() as i16;
    *out5 = dt.minute() as i16;
    *out6 = dt.second() as i16;
    *out7 = 0;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__UINT(
    in1: u32,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
    out5: &mut u16,
    out6: &mut u16,
    out7: &mut u16,
) -> i16 {
    let dt = dt_from_epoch_seconds(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as u16;
    *out3 = dt.day() as u16;
    *out4 = dt.hour() as u16;
    *out5 = dt.minute() as u16;
    *out6 = dt.second() as u16;
    *out7 = 0;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__DINT(
    in1: u32,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
    out5: &mut i32,
    out6: &mut i32,
    out7: &mut i32,
) -> i16 {
    let dt = dt_from_epoch_seconds(in1);
    *out1 = dt.year();
    *out2 = dt.month() as i32;
    *out3 = dt.day() as i32;
    *out4 = dt.hour() as i32;
    *out5 = dt.minute() as i32;
    *out6 = dt.second() as i32;
    *out7 = 0;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__UDINT(
    in1: u32,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
    out5: &mut u32,
    out6: &mut u32,
    out7: &mut u32,
) -> i16 {
    let dt = dt_from_epoch_seconds(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month();
    *out3 = dt.day();
    *out4 = dt.hour();
    *out5 = dt.minute();
    *out6 = dt.second();
    *out7 = 0;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__LINT(
    in1: u32,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
    out5: &mut i64,
    out6: &mut i64,
    out7: &mut i64,
) -> i16 {
    let dt = dt_from_epoch_seconds(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().into();
    *out2 = dt.month() as i64;
    *out3 = dt.day() as i64;
    *out4 = dt.hour() as i64;
    *out5 = dt.minute() as i64;
    *out6 = dt.second() as i64;
    *out7 = 0;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__ULINT(
    in1: u32,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
    out5: &mut u64,
    out6: &mut u64,
    out7: &mut u64,
) -> i16 {
    let dt = dt_from_epoch_seconds(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as u64;
    *out3 = dt.day() as u64;
    *out4 = dt.hour() as u64;
    *out5 = dt.minute() as u64;
    *out6 = dt.second() as u64;
    *out7 = 0;

    0
}

/// .
/// Splits LDT into year, month, day, hour, minute, second, millisecond of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LDT__INT(
    in1: i64,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
    out5: &mut i16,
    out6: &mut i16,
    out7: &mut i16,
) -> i16 {
    let (year, month, day, hour, minute, second, millisecond) = split_ldt_fields(in1);
    *out1 = year.try_into().unwrap();
    *out2 = month as i16;
    *out3 = day as i16;
    *out4 = hour as i16;
    *out5 = minute as i16;
    *out6 = second as i16;
    *out7 = millisecond as i16;

    0
}

/// .
/// Splits LDT into year, month, day, hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LDT__UINT(
    in1: i64,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
    out5: &mut u16,
    out6: &mut u16,
    out7: &mut u16,
) -> i16 {
    let (year, month, day, hour, minute, second, millisecond) = split_ldt_fields(in1);
    *out1 = year.try_into().unwrap();
    *out2 = month as u16;
    *out3 = day as u16;
    *out4 = hour as u16;
    *out5 = minute as u16;
    *out6 = second as u16;
    *out7 = millisecond as u16;

    0
}

/// .
/// Splits LDT into year, month, day, hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LDT__DINT(
    in1: i64,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
    out5: &mut i32,
    out6: &mut i32,
    out7: &mut i32,
) -> i16 {
    let (year, month, day, hour, minute, second, millisecond) = split_ldt_fields(in1);
    *out1 = year;
    *out2 = month as i32;
    *out3 = day as i32;
    *out4 = hour as i32;
    *out5 = minute as i32;
    *out6 = second as i32;
    *out7 = millisecond as i32;

    0
}

/// .
/// Splits LDT into year, month, day, hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LDT__UDINT(
    in1: i64,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
    out5: &mut u32,
    out6: &mut u32,
    out7: &mut u32,
) -> i16 {
    let (year, month, day, hour, minute, second, millisecond) = split_ldt_fields(in1);
    *out1 = year.try_into().unwrap();
    *out2 = month;
    *out3 = day;
    *out4 = hour;
    *out5 = minute;
    *out6 = second;
    *out7 = millisecond;

    0
}

/// .
/// Splits LDT into year, month, day, hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LDT__LINT(
    in1: i64,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
    out5: &mut i64,
    out6: &mut i64,
    out7: &mut i64,
) -> i16 {
    let (year, month, day, hour, minute, second, millisecond) = split_ldt_fields(in1);
    *out1 = year.into();
    *out2 = month as i64;
    *out3 = day as i64;
    *out4 = hour as i64;
    *out5 = minute as i64;
    *out6 = second as i64;
    *out7 = millisecond as i64;

    0
}

/// .
/// Splits LDT into year, month, day, hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_LDT__ULINT(
    in1: i64,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
    out5: &mut u64,
    out6: &mut u64,
    out7: &mut u64,
) -> i16 {
    let (year, month, day, hour, minute, second, millisecond) = split_ldt_fields(in1);
    *out1 = year.try_into().unwrap();
    *out2 = month as u64;
    *out3 = day as u64;
    *out4 = hour as u64;
    *out5 = minute as u64;
    *out6 = second as u64;
    *out7 = millisecond as u64;

    0
}

/// .
/// Returns day of week for given DATE of type SINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK(in1: u32) -> i8 {
    let date = dt_from_epoch_seconds(in1);
    date.weekday().num_days_from_sunday() as i8
}
