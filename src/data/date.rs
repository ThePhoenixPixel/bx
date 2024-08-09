use chrono::Datelike;
use serde::{Deserialize, Serialize};

use crate::data::month::Month;
use crate::data::time::Time;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Date {
    day: u8,
    month: Month,
    year: u32,
    time: Time,
}

impl Date {
    // create new Date obj.
    pub fn new(day: &u8, month: &Month, year: &u32, time: &Time) -> Result<Self, String> {
        // number of days
        let days_in_month = match month {
            Month::JANUARY
            | Month::MARCH
            | Month::MAY
            | Month::JULY
            | Month::AUGUST
            | Month::OCTOBER
            | Month::DECEMBER => 31,
            Month::APRIL | Month::JUNE | Month::SEPTEMBER | Month::NOVEMBER => 30,
            Month::FEBRUARY => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
        };
        if day < &1.clone() || day > &days_in_month {
            return Err("Invalid day for the given month".to_string());
        }

        // time validation
        if !time.is_valid() {
            return Err("Invalid time".to_string());
        }

        Ok(Date {
            day: day.clone(),
            month: month.clone(),
            year: year.clone(),
            time: time.clone(),
        })
    }

    // getter and setter
    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn set_day(&mut self, day: &u8) {
        self.day = *day;
    }

    // Getter für month
    pub fn month(&self) -> &Month {
        &self.month
    }

    // Setter für month
    pub fn set_month(&mut self, month: Month) {
        self.month = month;
    }

    // Getter für year
    pub fn year(&self) -> u32 {
        self.year
    }

    // Setter für year
    pub fn set_year(&mut self, year: u32) {
        self.year = year;
    }

    // Getter für time
    pub fn time(&self) -> &Time {
        &self.time
    }

    // Setter für time
    pub fn set_time(&mut self, time: Time) {
        self.time = time;
    }

    // get the date as format string
    pub fn formatted_date(&self, split: &char) -> String {
        format!(
            "{:02}{split}{:02}{split}{:04} {}",
            self.day,
            self.month.to_number(),
            self.year,
            self.time.format(&split)
        )
    }

    // check is leap year
    pub fn is_leap_year(year: &u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    
    pub fn now() -> Self {
        let now = chrono::Local::now();
        Self {
            day: now.day() as u8,
            month: Month::now(),
            year: now.year() as u32,
            time: Time::now(),
        }
    }
}
