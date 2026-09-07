use common::{compile_and_run, get_includes};
use iec61131std::date_time_conversion as dtc;

// Import common functionality into the integration tests
mod common;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

#[test]
fn ltime_to_time_conversion() {
    let src = "
    FUNCTION main : TIME
        main := LTIME_TO_TIME(LTIME#10s);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(res, 10000);
}

#[test]
fn time_to_ltime_conversion() {
    let src = "
    FUNCTION main : LTIME
        main := TIME_TO_LTIME(TIME#10s);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(res, 10000000000);
}

#[test]
fn ldt_to_dt_conversion() {
    let src = "
    FUNCTION main : DT
        main := LDT_TO_DT(LDT#2021-04-20-22:33:14);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2021, 4, 20)
            .unwrap()
            .and_hms_opt(22, 33, 14)
            .unwrap()
            .and_utc()
            .timestamp() as u32
    );
}

#[test]
fn ldt_to_date_conversion() {
    let src = "
    FUNCTION main : DATE
        main := LDT_TO_DATE(LDT#2000-01-01-20:15:11);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp() as u32
    );
}

#[test]
fn ldt_to_ltod_conversion() {
    let src = "
    FUNCTION main : LTOD
        main := LDT_TO_LTOD(LDT#2000-01-01-15:36:30.123456);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_nano_opt(15, 36, 30, 123456000)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn ldt_to_tod_conversion() {
    let src = "
    FUNCTION main : TOD
        main := LDT_TO_TOD(LDT#2120-02-12-20:15:11.543);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_milli_opt(20, 15, 11, 543)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u32
    );
}

#[test]
fn dt_to_ldt_conversion() {
    let src = "
    FUNCTION main : LDT
        main := DT_TO_LDT(DT#2021-04-20-22:33:14);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2021, 4, 20)
            .unwrap()
            .and_hms_opt(22, 33, 14)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn dt_to_date_conversion() {
    let src = "
    FUNCTION main : DATE
        main := DT_TO_DATE(DT#2000-01-01-20:15:11);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp() as u32
    );
}

#[test]
fn dt_to_ldate_conversion() {
    let src = "
    FUNCTION main : LDATE
        main := DT_TO_LDATE(DT#2000-01-01-20:15:11);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn dt_to_ltod_conversion() {
    let src = "
    FUNCTION main : LTOD
        main := DT_TO_LTOD(DT#2000-01-01-15:36:30.123);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(15, 36, 30)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn dt_to_tod_conversion() {
    let src = "
    FUNCTION main : TOD
        main := DT_TO_TOD(DT#2120-02-12-20:15:11.543);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2120, 2, 12)
            .unwrap()
            .and_hms_opt(20, 15, 11)
            .unwrap()
            .and_utc()
            .timestamp() as u32
            % 86_400
            * 1000
    );
}

#[test]
fn ltod_to_tod_conversion() {
    let src = "
    FUNCTION main : TOD
        main := LTOD_TO_TOD(LTOD#10:20:30);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: u32 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(10, 20, 30)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u32
    );
}

#[test]
fn tod_to_ltod_conversion() {
    let src = "
    FUNCTION main : LTOD
        main := TOD_TO_LTOD(TOD#10:20:30);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(10, 20, 30)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

// The conversions are total integer arithmetic; these tests pin the values the
// previous chrono-based implementation produced and the saturation at the LDT
// range minimum (midnight of the first representable day lies below i64::MIN).
#[test]
fn ldate_and_time_to_ldate_saturates_on_the_first_representable_day() {
    assert_eq!(dtc::LDATE_AND_TIME_TO_LDATE(i64::MIN), i64::MIN);

    let noon_first_day = i64::MIN + 12 * 3_600 * 1_000_000_000;
    assert_eq!(dtc::LDATE_AND_TIME_TO_LDATE(noon_first_day), i64::MIN);
}

#[test]
fn ldate_and_time_to_ldate_returns_midnight() {
    let datetime = chrono::NaiveDate::from_ymd_opt(2024, 5, 5)
        .and_then(|date| date.and_hms_opt(13, 30, 15))
        .expect("valid date");
    let midnight = chrono::NaiveDate::from_ymd_opt(2024, 5, 5)
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .expect("valid date");

    assert_eq!(
        dtc::LDATE_AND_TIME_TO_LDATE(datetime.and_utc().timestamp_nanos_opt().expect("in range")),
        midnight.and_utc().timestamp_nanos_opt().expect("in range")
    );

    // pre-1970 values floor to the day boundary below, not toward zero
    let before_epoch = chrono::NaiveDate::from_ymd_opt(1969, 12, 31)
        .and_then(|date| date.and_hms_opt(23, 0, 0))
        .expect("valid date");
    let before_epoch_midnight = chrono::NaiveDate::from_ymd_opt(1969, 12, 31)
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .expect("valid date");

    assert_eq!(
        dtc::LDATE_AND_TIME_TO_LDATE(before_epoch.and_utc().timestamp_nanos_opt().expect("in range")),
        before_epoch_midnight.and_utc().timestamp_nanos_opt().expect("in range")
    );
}

#[test]
fn date_and_time_conversions_match_previous_values_at_the_range_bounds() {
    assert_eq!(dtc::DATE_AND_TIME_TO_DATE(0), 0);
    assert_eq!(dtc::DATE_AND_TIME_TO_DATE(u32::MAX), 4_294_944_000);
    assert_eq!(dtc::DATE_AND_TIME_TO_TIME_OF_DAY(0), 0);
    assert_eq!(dtc::DATE_AND_TIME_TO_TIME_OF_DAY(u32::MAX), 23_295_000);

    let noon_nanos = 12 * 3_600 * 1_000_000_000_i64;
    assert_eq!(dtc::LDATE_AND_TIME_TO_LTIME_OF_DAY(noon_nanos), noon_nanos);
    // one hour before the epoch is 23:00 on the previous day
    assert_eq!(dtc::LDATE_AND_TIME_TO_LTIME_OF_DAY(-3_600 * 1_000_000_000), 23 * 3_600 * 1_000_000_000);
}
