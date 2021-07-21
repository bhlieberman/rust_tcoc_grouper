#![allow(dead_code)]
#![allow(unused_imports)]
use chrono::{Datelike, Duration, Month, NaiveDate, ParseResult, format};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct DatePeriod {
    start_date: Option<String>,
    end_date: Option<String>,
}

impl DatePeriod {
    pub fn new(s_date: Option<String>, e_date: Option<String>) -> DatePeriod {
        DatePeriod {
            start_date: s_date,
            end_date: e_date,
        }
    }
    pub fn validate(&self) -> ParseResult<bool> {
        match self.start_date.as_ref() {
            None => {
            match self.end_date.as_ref() {
                Some(_s) => panic!("End date must be null if start date is null"),
                None => Ok(true),
                }
            }
            Some(start) => match self.end_date.as_ref() {
                Some(end) => {
                    let end = NaiveDate::parse_from_str(end.as_str(), "%Y-%m-%d")?;
                    let start = NaiveDate::parse_from_str(start.as_str(), "%Y-%m-%d")?;
                    Ok(end - start >= Duration::days(0))
                    }
                None => Ok(true),
                }
            }
        }
    }

#[cfg(test)]
mod date_time_tests {
    use core::panic;

    use chrono::NaiveDate;

    use super::*;
    #[test]
    fn date_constructor() {
        let test_date = DatePeriod::new(Some(String::from("2020-07-14")), Some(String::from("2021-07-14")));
        assert_eq!(test_date, DatePeriod { start_date: Some(String::from("2020-07-14")), end_date: Some(String::from("2021-07-14")) });
    }
    #[test]
    fn strptime_test() {
        let test_date = DatePeriod::new(Some(String::from("2020-07-14")), Some(String::from("2021-07-14")));
        assert_eq!(NaiveDate::parse_from_str(test_date.start_date.unwrap().as_str(), "%Y-%m-%d"), Ok(NaiveDate::from_ymd(2020, 07, 14)));
    }
    #[test]
    fn validation() {
        let start_null = DatePeriod {
            start_date: None,
            end_date: Some(String::from("2021-07-14")),
        };
        let both_some = DatePeriod {
            start_date: Some(String::from("2020-07-14")),
            end_date: Some(String::from("2021-07-13")),
        };
        assert!(start_null.validate().unwrap());
        assert!(both_some.validate().unwrap());
    }
    #[test]
    #[should_panic]
    fn validation_panic() {
        let end_null = DatePeriod {
            start_date: Some(String::from("2020-07-14")),
            end_date: None,
        };
        let both_null = DatePeriod {
            start_date: None,
            end_date: None,
        };
        assert!(end_null.validate().unwrap());
        assert!(both_null.validate().unwrap());
    }
}

