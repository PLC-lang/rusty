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
    let date_time = chrono::Utc.timestamp_opt(input_seconds, 0).single().expect("Out of range DT value");

    let midnight_seconds = date_time
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("Cannot create date time from date")
        .and_utc()
        .timestamp();

    midnight_seconds as u32
}

/// .
/// Converts LDT to LDATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDATE_AND_TIME_TO_LDATE(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_nanos(input);

    let new_date_time =
        date_time.date_naive().and_hms_opt(0, 0, 0).expect("Cannot create date time from date");

    new_date_time.and_utc().timestamp_nanos_opt().expect("Out of range, cannot create DATE")
}

/// .
/// Converts DT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_TIME_OF_DAY(input: u32) -> u32 {
    let input_seconds = input as i64;
    let date_time = chrono::Utc.timestamp_opt(input_seconds, 0).single().expect("Out of range DT value");

    let midnight = chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
        .and_then(|date| date.and_hms_opt(date_time.hour(), date_time.minute(), date_time.second()))
        .expect("Cannot create date time from given parameters")
        .and_utc();

    midnight.timestamp_millis() as u32
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

    let new_date_time = chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
        .and_then(|date| date.and_hms_nano_opt(hour, min, sec, nano))
        .expect("Cannot create date time from given parameters");

    new_date_time.and_utc().timestamp_nanos_opt().expect("Out of range, cannot create TOD")
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
