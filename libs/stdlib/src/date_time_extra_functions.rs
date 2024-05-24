use chrono::{Datelike, NaiveDate, TimeZone, Timelike};

/// .
/// Concatenates DATE and TOD to DT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE_TOD(in1: i64, in2: i64) -> i64 {
    let date = chrono::Utc.timestamp_nanos(in1).date_naive();
    let tod = chrono::Utc.timestamp_nanos(in2);
    let hour = tod.hour();
    let min = tod.minute();
    let sec = tod.second();
    let nano = tod.timestamp_subsec_nanos();

    date.and_hms_nano_opt(hour, min, sec, nano)
        .expect("Invalid input")
        .and_utc()
        .timestamp_nanos_opt()
        .expect("Out of range, cannot create Date")
}

/// .
/// Concatenates year, month and day of type INT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__INT(in1: i16, in2: i16, in3: i16) -> i64 {
    concat_date(in1.into(), in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type UINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UINT(in1: u16, in2: u16, in3: u16) -> i64 {
    concat_date(in1.into(), in2.into(), in3.into())
}

/// .
/// Concatenates year, month and day of type DINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__DINT(in1: i32, in2: i32, in3: i32) -> i64 {
    concat_date(in1, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type UDINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UDINT(in1: u32, in2: u32, in3: u32) -> i64 {
    concat_date(in1 as i32, in2, in3)
}

/// .
/// Concatenates year, month and day of type LINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__LINT(in1: i64, in2: i64, in3: i64) -> i64 {
    concat_date(in1 as i32, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type ULINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__ULINT(in1: u64, in2: u64, in3: u64) -> i64 {
    concat_date(in1 as i32, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_date(in1: i32, in2: u32, in3: u32) -> i64 {
    let dt = NaiveDate::from_ymd_opt(in1, in2, in3)
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .expect("Invalid parameters, cannot create date");

    dt.and_utc().timestamp_nanos_opt().expect("Out of range, cannot create date")
}

/// .
/// Concatenates hour, minute, second, millisecond of type SINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__SINT(in1: i8, in2: i8, in3: i8, in4: i8) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type USINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__USINT(in1: u8, in2: u8, in3: u8, in4: u8) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type INT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__INT(in1: i16, in2: i16, in3: i16, in4: i16) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UINT(in1: u16, in2: u16, in3: u16, in4: u16) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type DINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__DINT(in1: i32, in2: i32, in3: i32, in4: i32) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UDINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UDINT(in1: u32, in2: u32, in3: u32, in4: u32) -> i64 {
    concat_tod(in1, in2, in3, in4)
}

/// .
/// Concatenates hour, minute, second, millisecond of type LINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__LINT(in1: i64, in2: i64, in3: i64, in4: i64) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type ULINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__ULINT(in1: u64, in2: u64, in3: u64, in4: u64) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_tod(in1: u32, in2: u32, in3: u32, in4: u32) -> i64 {
    let dt = NaiveDate::from_ymd_opt(1970, 1, 1)
        .and_then(|date| date.and_hms_milli_opt(in1, in2, in3, in4))
        .expect("Invalid parameters, cannot create TOD");

    dt.and_utc().timestamp_nanos_opt().expect("Out of range, cannot create TOD")
}

/// .
/// Splits DATE into year, month, day of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__INT(in1: i64, out1: &mut i16, out2: &mut i16, out3: &mut i16) -> i16 {
    let date = chrono::Utc.timestamp_nanos(in1).date_naive();
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
pub extern "C" fn SPLIT_DATE__UINT(in1: i64, out1: &mut u16, out2: &mut u16, out3: &mut u16) -> i16 {
    let date = chrono::Utc.timestamp_nanos(in1).date_naive();
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
pub extern "C" fn SPLIT_DATE__DINT(in1: i64, out1: &mut i32, out2: &mut i32, out3: &mut i32) -> i16 {
    let date = chrono::Utc.timestamp_nanos(in1).date_naive();
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
pub extern "C" fn SPLIT_DATE__UDINT(in1: i64, out1: &mut u32, out2: &mut u32, out3: &mut u32) -> i16 {
    let date = chrono::Utc.timestamp_nanos(in1).date_naive();
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
pub extern "C" fn SPLIT_DATE__LINT(in1: i64, out1: &mut i64, out2: &mut i64, out3: &mut i64) -> i16 {
    let date = chrono::Utc.timestamp_nanos(in1).date_naive();
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
pub extern "C" fn SPLIT_DATE__ULINT(in1: i64, out1: &mut u64, out2: &mut u64, out3: &mut u64) -> i16 {
    let date = chrono::Utc.timestamp_nanos(in1).date_naive();
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
    in1: i64,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
) -> i16 {
    let tod = chrono::Utc.timestamp_nanos(in1);
    *out1 = tod.hour() as i16;
    *out2 = tod.minute() as i16;
    *out3 = tod.second() as i16;
    *out4 = tod.timestamp_subsec_millis() as i16;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__UINT(
    in1: i64,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
) -> i16 {
    let tod = chrono::Utc.timestamp_nanos(in1);
    *out1 = tod.hour() as u16;
    *out2 = tod.minute() as u16;
    *out3 = tod.second() as u16;
    *out4 = tod.timestamp_subsec_millis() as u16;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__DINT(
    in1: i64,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
) -> i16 {
    let tod = chrono::Utc.timestamp_nanos(in1);
    *out1 = tod.hour() as i32;
    *out2 = tod.minute() as i32;
    *out3 = tod.second() as i32;
    *out4 = tod.timestamp_subsec_millis() as i32;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__UDINT(
    in1: i64,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
) -> i16 {
    let tod = chrono::Utc.timestamp_nanos(in1);
    *out1 = tod.hour();
    *out2 = tod.minute();
    *out3 = tod.second();
    *out4 = tod.timestamp_subsec_millis();

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__LINT(
    in1: i64,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
) -> i16 {
    let tod = chrono::Utc.timestamp_nanos(in1);
    *out1 = tod.hour() as i64;
    *out2 = tod.minute() as i64;
    *out3 = tod.second() as i64;
    *out4 = tod.timestamp_subsec_millis() as i64;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__ULINT(
    in1: i64,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
) -> i16 {
    let tod = chrono::Utc.timestamp_nanos(in1);
    *out1 = tod.hour() as u64;
    *out2 = tod.minute() as u64;
    *out3 = tod.second() as u64;
    *out4 = tod.timestamp_subsec_millis() as u64;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__INT(
    in1: i64,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
    out5: &mut i16,
    out6: &mut i16,
    out7: &mut i16,
) -> i16 {
    let dt = chrono::Utc.timestamp_nanos(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as i16;
    *out3 = dt.day() as i16;
    *out4 = dt.hour() as i16;
    *out5 = dt.minute() as i16;
    *out6 = dt.second() as i16;
    *out7 = dt.timestamp_subsec_millis() as i16;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__UINT(
    in1: i64,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
    out5: &mut u16,
    out6: &mut u16,
    out7: &mut u16,
) -> i16 {
    let dt = chrono::Utc.timestamp_nanos(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as u16;
    *out3 = dt.day() as u16;
    *out4 = dt.hour() as u16;
    *out5 = dt.minute() as u16;
    *out6 = dt.second() as u16;
    *out7 = dt.timestamp_subsec_millis() as u16;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__DINT(
    in1: i64,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
    out5: &mut i32,
    out6: &mut i32,
    out7: &mut i32,
) -> i16 {
    let dt = chrono::Utc.timestamp_nanos(in1);
    *out1 = dt.year();
    *out2 = dt.month() as i32;
    *out3 = dt.day() as i32;
    *out4 = dt.hour() as i32;
    *out5 = dt.minute() as i32;
    *out6 = dt.second() as i32;
    *out7 = dt.timestamp_subsec_millis() as i32;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__UDINT(
    in1: i64,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
    out5: &mut u32,
    out6: &mut u32,
    out7: &mut u32,
) -> i16 {
    let dt = chrono::Utc.timestamp_nanos(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month();
    *out3 = dt.day();
    *out4 = dt.hour();
    *out5 = dt.minute();
    *out6 = dt.second();
    *out7 = dt.timestamp_subsec_millis();

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__LINT(
    in1: i64,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
    out5: &mut i64,
    out6: &mut i64,
    out7: &mut i64,
) -> i16 {
    let dt = chrono::Utc.timestamp_nanos(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().into();
    *out2 = dt.month() as i64;
    *out3 = dt.day() as i64;
    *out4 = dt.hour() as i64;
    *out5 = dt.minute() as i64;
    *out6 = dt.second() as i64;
    *out7 = dt.timestamp_subsec_millis() as i64;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__ULINT(
    in1: i64,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
    out5: &mut u64,
    out6: &mut u64,
    out7: &mut u64,
) -> i16 {
    let dt = chrono::Utc.timestamp_nanos(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as u64;
    *out3 = dt.day() as u64;
    *out4 = dt.hour() as u64;
    *out5 = dt.minute() as u64;
    *out6 = dt.second() as u64;
    *out7 = dt.timestamp_subsec_millis() as u64;

    0
}

/// .
/// Returns day of week for given DATE of type SINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK(in1: i64) -> i8 {
    let date = chrono::Utc.timestamp_nanos(in1);
    date.weekday().num_days_from_sunday() as i8
}
