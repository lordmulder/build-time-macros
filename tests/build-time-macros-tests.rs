// SPDX-License-Identifier: 0BSD
// build-time-macros
// Copyright (C) 2026 by LoRd_MuldeR <mulder2@gmx.de>

use build_time_macros::*;
use regex::Regex;
use time::{format_description::well_known::Rfc3339, macros::format_description, OffsetDateTime};

#[test]
fn test_time_utc_regex() {
    let re = Regex::new(r"^([0-9]{2}):([0-9]{2}):([0-9]{2})$").unwrap();
    assert!(re.is_match(build_time!()));
}

#[test]
fn test_date_utc_regex() {
    let re = Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    assert!(re.is_match(build_date!()));
}

#[test]
fn test_full_utc_regex() {
    let re = Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})Z$").unwrap();
    assert!(re.is_match(build_full!()));
}

#[cfg(feature = "local")]
#[test]
fn test_time_local_regex() {
    let re = Regex::new(r"^([0-9]{2}):([0-9]{2}):([0-9]{2})$").unwrap();
    assert!(re.is_match(local_time!()));
}

#[cfg(feature = "local")]
#[test]
fn test_date_local_regex() {
    let re = Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    assert!(re.is_match(local_date!()));
}

#[cfg(feature = "local")]
#[test]
fn test_full_local_regex() {
    let re = Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(Z|(\+|-)[0-9]{2}:[0-9]{2})$").unwrap();
    assert!(re.is_match(local_full!()));
}

#[test]
fn test_unix_to_time() {
    let timestamp = OffsetDateTime::from_unix_timestamp(build_unix!()).unwrap();
    assert_eq!(timestamp.format(format_description!("[hour]:[minute]:[second]")).unwrap(), build_time!());
}

#[test]
fn test_unix_to_date() {
    let timestamp = OffsetDateTime::from_unix_timestamp(build_unix!()).unwrap();
    assert_eq!(timestamp.format(format_description!("[year]-[month]-[day]")).unwrap(), build_date!());
}

#[test]
fn test_unix_to_full() {
    let timestamp = OffsetDateTime::from_unix_timestamp(build_unix!()).unwrap();
    assert_eq!(timestamp.format(&Rfc3339).unwrap(), build_full!());
}
