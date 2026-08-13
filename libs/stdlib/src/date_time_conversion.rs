use chrono::{TimeZone, Timelike};

const NANOS_PER_MILLISECOND: i64 = 1_000 * 1_000;
const NANOS_PER_SECOND: i64 = 1_000 * 1_000 * 1_000;
const SECONDS_PER_DAY: u32 = 60 * 60 * 24;

/// .
/// Converts DT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_DATE(input: u32) -> u32 {
    let input_seconds = input as i64;
    // Every u32 second count is a valid chrono timestamp; fall back to the
    // epoch defensively instead of panicking.
    chrono::Utc
        .timestamp_opt(input_seconds, 0)
        .single()
        .and_then(|date_time| date_time.date_naive().and_hms_opt(0, 0, 0))
        .map(|midnight| midnight.and_utc().timestamp() as u32)
        .unwrap_or(0)
}

/// .
/// Converts LDT to LDATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDATE_AND_TIME_TO_LDATE(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_nanos(input);

    // Midnight of a date derived from an i64 nanosecond timestamp is always
    // representable; fall back to the epoch defensively instead of panicking.
    date_time
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .and_then(|new_date_time| new_date_time.and_utc().timestamp_nanos_opt())
        .unwrap_or(0)
}

/// .
/// Converts DT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_TIME_OF_DAY(input: u32) -> u32 {
    let input_seconds = input as i64;
    // Every u32 second count is a valid chrono timestamp; fall back to midnight
    // defensively instead of panicking.
    chrono::Utc
        .timestamp_opt(input_seconds, 0)
        .single()
        .and_then(|date_time| {
            chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                .and_then(|date| date.and_hms_opt(date_time.hour(), date_time.minute(), date_time.second()))
        })
        .map(|time_of_day| time_of_day.and_utc().timestamp_millis() as u32)
        .unwrap_or(0)
}

/// .
/// Converts LDT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDATE_AND_TIME_TO_LTIME_OF_DAY(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_nanos(input);
    let hour = date_time.hour();
    let min = date_time.minute();
    let sec = date_time.second();
    let nano = date_time.timestamp_subsec_nanos();

    // A time of day on 1970-01-01 is always representable; fall back to
    // midnight defensively instead of panicking.
    chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
        .and_then(|date| date.and_hms_nano_opt(hour, min, sec, nano))
        .and_then(|new_date_time| new_date_time.and_utc().timestamp_nanos_opt())
        .unwrap_or(0)
}

/// .
/// Converts LTIME to TIME
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LTIME_TO_TIME(input: i64) -> u32 {
    (input / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts TIME to LTIME
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TIME_TO_LTIME(input: u32) -> i64 {
    (input as i64) * NANOS_PER_MILLISECOND
}

/// .
/// Converts LDT to DT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_DT(input: i64) -> u32 {
    (input / NANOS_PER_SECOND) as u32
}

/// .
/// Converts LDT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_DATE(input: i64) -> u32 {
    (LDATE_AND_TIME_TO_LDATE(input) / NANOS_PER_SECOND) as u32
}

/// .
/// Converts LDT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_LTOD(input: i64) -> i64 {
    LDATE_AND_TIME_TO_LTIME_OF_DAY(input)
}

/// .
/// Converts LDT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_TOD(input: i64) -> u32 {
    (LDT_TO_LTOD(input) / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts DT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_LDT(input: u32) -> i64 {
    (input as i64) * NANOS_PER_SECOND
}

/// .
/// Converts DT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_DATE(input: u32) -> u32 {
    (input / SECONDS_PER_DAY) * SECONDS_PER_DAY
}

/// .
/// Converts DT to LDATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_LDATE(input: u32) -> i64 {
    (DT_TO_DATE(input) as i64) * NANOS_PER_SECOND
}

/// .
/// Converts DT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_LTOD(input: u32) -> i64 {
    LDT_TO_LTOD(DT_TO_LDT(input))
}

/// .
/// Converts DT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_TOD(input: u32) -> u32 {
    (DT_TO_LTOD(input) / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts LTOD to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LTOD_TO_TOD(input: i64) -> u32 {
    (input / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts TOD to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOD_TO_LTOD(input: u32) -> i64 {
    (input as i64) * NANOS_PER_MILLISECOND
}
