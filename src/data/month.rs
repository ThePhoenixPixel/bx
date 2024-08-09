use serde::{Deserialize, Serialize};
use std::fmt;
use chrono::Datelike;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Month {
    JANUARY,
    FEBRUARY,
    MARCH,
    APRIL,
    MAY,
    JUNE,
    JULY,
    AUGUST,
    SEPTEMBER,
    OCTOBER,
    NOVEMBER,
    DECEMBER,
}

impl Month {
    // convert in string
    pub fn to_string(&self) -> &str {
        match self {
            Month::JANUARY => "January",
            Month::FEBRUARY => "February",
            Month::MARCH => "March",
            Month::APRIL => "April",
            Month::MAY => "May",
            Month::JUNE => "June",
            Month::JULY => "July",
            Month::AUGUST => "August",
            Month::SEPTEMBER => "September",
            Month::OCTOBER => "October",
            Month::NOVEMBER => "November",
            Month::DECEMBER => "December",
        }
    }

    // convert from string
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "january" => Some(Month::JANUARY),
            "february" => Some(Month::FEBRUARY),
            "march" => Some(Month::MARCH),
            "april" => Some(Month::APRIL),
            "may" => Some(Month::MAY),
            "june" => Some(Month::JUNE),
            "july" => Some(Month::JULY),
            "august" => Some(Month::AUGUST),
            "september" => Some(Month::SEPTEMBER),
            "october" => Some(Month::OCTOBER),
            "november" => Some(Month::NOVEMBER),
            "december" => Some(Month::DECEMBER),
            _ => None,
        }
    }

    // post as Number (1-12)
    pub fn to_number(&self) -> u8 {
        match self {
            Month::JANUARY => 1,
            Month::FEBRUARY => 2,
            Month::MARCH => 3,
            Month::APRIL => 4,
            Month::MAY => 5,
            Month::JUNE => 6,
            Month::JULY => 7,
            Month::AUGUST => 8,
            Month::SEPTEMBER => 9,
            Month::OCTOBER => 10,
            Month::NOVEMBER => 11,
            Month::DECEMBER => 12,
        }
    }

    // convert from number (1-12) to Month
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            1 => Some(Month::JANUARY),
            2 => Some(Month::FEBRUARY),
            3 => Some(Month::MARCH),
            4 => Some(Month::APRIL),
            5 => Some(Month::MAY),
            6 => Some(Month::JUNE),
            7 => Some(Month::JULY),
            8 => Some(Month::AUGUST),
            9 => Some(Month::SEPTEMBER),
            10 => Some(Month::OCTOBER),
            11 => Some(Month::NOVEMBER),
            12 => Some(Month::DECEMBER),
            _ => None,
        }
    }

    pub fn now() -> Self {
        let now = chrono::Local::now();
        Self::from_number(now.month() as u8).expect("Error from 'BX' lib error month")
    }
}

// impl for format for month as string
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
