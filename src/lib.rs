pub mod event;
pub mod filtering;

use chrono::{Datelike, Local};
use filtering::{Date, Filtering};

pub fn new(user: &str) -> Filtering {
    Filtering {
        user: user.to_string(),
        from: Date {
            year: Local::now().date().year() as u16,
            month: Local::now().date().month() as u8,
            day: Local::now().date().day() as u8,
        },
        to: Date {
            year: Local::now().date().year() as u16,
            month: Local::now().date().month() as u8,
            day: Local::now().date().day() as u8,
        },
    }
}
