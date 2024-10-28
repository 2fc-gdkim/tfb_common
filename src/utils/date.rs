use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use regex::Regex;
use std::time;

pub fn get_date(delimiter: bool) -> String {
    if delimiter {
        Local::now().format("%Y-%m-%d").to_string()
    }
    else {
        Local::now().format("%Y%m%d").to_string()
    }
}

pub fn get_time(delimiter: bool) -> String {
    if delimiter {
        Local::now().format("%H:%M:%S").to_string()
    }
    else {
        Local::now().format("%H%M%S").to_string()
    }
}

pub fn get_datetime(delimiter: bool) -> String {
    if delimiter {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
    else {
        Local::now().format("%Y%m%d%H%M%S").to_string()
    }
}


pub fn diff_days(date_string: &str) -> i64 {
    let today = Utc::now();
    let naive_datetime = NaiveDateTime::new(NaiveDate::parse_from_str(date_string, "%Y-%m-%d").unwrap(),  NaiveTime::from_hms_opt(0, 0, 0).unwrap());

    let datetime =
        DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);

    let diff = today.signed_duration_since(datetime);
    diff.num_days()
}

/**
 * 시간 포맷이 맞는지 확인
 */
pub fn is_valid_time(time_str: &str) -> bool {
    // 정규 표현식 패턴
    let time_pattern = Regex::new(r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap();

    // 패턴과 매치되는지 확인
    match time_pattern.captures(time_str) {
        Some(_) => true,
        None => false,
    }
}

/**
 * 시간:분 형태의 시간 포맷이 맞는지 확인
 */
pub fn is_valid_time_hm(time_str: &str) -> bool {
    // 정규 표현식 패턴
    let time_pattern = Regex::new(r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$").unwrap();

    // 패턴과 매치되는지 확인
    match time_pattern.captures(time_str) {
        Some(_) => true,
        None => false,
    }
}

/**
 * 시스템 시간을 문자열로 변환
 */
pub fn system_time_to_string(dt: time::SystemTime) -> String {    
    let now_time: DateTime<Local> = dt.into();
    now_time.format("%Y-%m-%d %H:%M:%S").to_string()
}