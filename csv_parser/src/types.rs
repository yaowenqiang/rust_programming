use crate::util;
use chrono::NaiveDate;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

pub struct Visit {
    start_date: NaiveDate,
    end_date: NaiveDate,
    description: String,
}

impl Visit {
    pub fn from_string(s: String) -> Visit {
        let tokens: Vec<&str> = s.split(',').collect();
        Visit {
            start_date: util::date_from_str(tokens[0]),
            end_date: util::date_from_str(tokens[1]),
            description: tokens[2].to_string(),
        }
    }

    pub fn get_days_in_period(
        &self,
        window_start_date: NaiveDate,
        window_end_date: NaiveDate,
    ) -> i64 {
        if self.start_date > window_end_date || self.end_date < window_start_date {
            0
        } else {
            let start_effective = std::cmp::max(self.start_date, self.end_date);
            let end_effective = std::cmp::min(self.start_date, self.end_date);
            end_effective
                .signed_duration_since(start_effective)
                .num_days()
        }
    }
}

impl Display for Visit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{},{},{}\n",
            self.start_date, self.end_date, self.description
        )
    }
}

#[derive(Eq, PartialEq)]
pub struct Record {
    date: NaiveDate,
    days: i64,
}

impl Record {
    pub fn new(date: NaiveDate, days: i64) -> Record {
        Record { date, days }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Days: {},date {}", self.days, self.date)
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        other.days.cmp(&self.days)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
