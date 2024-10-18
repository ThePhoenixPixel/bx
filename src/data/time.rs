use serde::{Deserialize, Serialize};
use std::fmt;
use chrono::Timelike;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

impl Time {
    // create new Time obj.
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    // getter and setter
    pub fn hours(&self) -> u32 {
        self.hours
    }

    pub fn set_hours(&mut self, hours: u32) {
        self.hours = hours;
    }

    pub fn minutes(&self) -> u32 {
        self.minutes
    }

    pub fn set_minutes(&mut self, minutes: u32) {
        self.minutes = minutes;
    }

    pub fn seconds(&self) -> u32 {
        self.seconds
    }

    pub fn set_seconds(&mut self, seconds: u32) {
        self.seconds = seconds;
    }

    // other functions

    // check if time valid
    pub fn is_valid(&self) -> bool {
        self.hours < 24 && self.minutes < 60 && self.seconds < 60
    }

    // convert time in seconds
    pub fn to_seconds(&self) -> u32 {
        self.hours * 3600 + self.minutes * 60 + self.seconds
    }

    // format time in format (HH MM SS)
    pub fn format(&self, split: &char) -> String {
        format!(
            "{:02}{split}{:02}{split}{:02}",
            self.hours, self.minutes, self.seconds
        )
    }

    // add seconds
    pub fn add_seconds(&mut self, seconds: u32) {
        let total = self.to_seconds() + seconds;
        self.hours = (total / 3600) % 24;
        self.minutes = (total % 3600) / 60;
        self.seconds = total % 60;
    }

    // remove seconds
    pub fn subtract_seconds(&mut self, seconds: u32) {
        let total = self.to_seconds().saturating_sub(seconds);
        self.hours = (total / 3600) % 24;
        self.minutes = (total % 3600) / 60;
        self.seconds = total % 60;
    }

    // add minutes
    pub fn add_minutes(&mut self, minutes: u32) {
        let total = self.to_seconds() + minutes;
        self.hours = (total / 3600) % 24;
        self.minutes = (total % 3600) / 60;
        self.seconds = total % 60;
    }

    // remove seconds
    pub fn subtract_minutes(&mut self, minutes: u32) {
        let total = self.to_seconds().saturating_sub(minutes);
        self.hours = (total / 3600) % 24;
        self.minutes = (total % 3600) / 60;
        self.seconds = total % 60;
    }

    // add hours
    pub fn add_hours(&mut self, hours: u32) {
        let total = self.to_seconds() + hours;
        self.hours = (total / 3600) % 24;
        self.minutes = (total % 3600) / 60;
        self.seconds = total % 60;
    }

    // remove hours
    pub fn subtract_hours(&mut self, hours: u32) {
        let total = self.to_seconds().saturating_sub(hours);
        self.hours = (total / 3600) % 24;
        self.minutes = (total % 3600) / 60;
        self.seconds = total % 60;
    }

    pub fn is_earlier_than(&self, other: &Time) -> bool {
        self.to_seconds() < other.to_seconds()
    }

    pub fn is_later_than(&self, other: &Time) -> bool {
        self.to_seconds() > other.to_seconds()
    }

    // get a now time obj.
    pub fn now() -> Self {
        let now = chrono::Local::now();
        Self {
            seconds: now.second(),
            minutes: now.minute(),
            hours: now.hour(),
        }
    }
}

// impl for format for time as string
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds
        )
    }
}
