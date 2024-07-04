use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(thiserror::Error, Debug)]
pub enum CalError {
    #[error("Can't make Month with {0}")]
    MonthRange(u32),
    #[error("Can't get first day of month")]
    FirstDayError,
    #[error("Can't find CAL_RS_USERFILE nor HOME env vars to find user file")]
    UserFileNotFound,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    TOMLSeError(#[from] toml::ser::Error),
    #[error(transparent)]
    TOMLDeError(#[from] toml::de::Error),
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub enum Weekday {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl From<&Weekday> for u32 {
    fn from(u: &Weekday) -> u32 {
        match u {
            Weekday::Sun => 0,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Year(pub i32);

#[derive(Debug, Deserialize, Serialize)]
pub enum Event {
    Single {
        description: String,
        year: Year,
        month: Month,
        day: u32,
    },
    Yearly {
        description: String,
        months: Vec<Month>,
        day: u32,
    },
    YearlyByWeekday {
        description: String,
        months: Vec<Month>,
        days: Vec<Weekday>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub events: Vec<Event>,
    pub months: [String; 12],
    pub weekdays: [String; 7],
}

impl Year {
    pub fn is_leap(&self) -> bool {
        (self.0 % 4) == 0 && (self.0 % 100 != 00 || self.0 % 400 == 0)
    }
}

impl TryFrom<u32> for Month {
    type Error = CalError;
    fn try_from(u: u32) -> Result<Month, CalError> {
        Ok(match u {
            1 => Month::Jan,
            2 => Month::Feb,
            3 => Month::Mar,
            4 => Month::Apr,
            5 => Month::May,
            6 => Month::Jun,
            7 => Month::Jul,
            8 => Month::Aug,
            9 => Month::Sep,
            10 => Month::Oct,
            11 => Month::Nov,
            12 => Month::Dec,
            x => Err(CalError::MonthRange(x))?,
        })
    }
}

impl Month {
    pub fn next(&self) -> Month {
        match self {
            Month::Dec => Month::Jan,
            Month::Jan => Month::Feb,
            Month::Feb => Month::Mar,
            Month::Mar => Month::Apr,
            Month::Apr => Month::May,
            Month::May => Month::Jun,
            Month::Jun => Month::Jul,
            Month::Jul => Month::Aug,
            Month::Aug => Month::Sep,
            Month::Sep => Month::Oct,
            Month::Oct => Month::Nov,
            Month::Nov => Month::Dec,
        }
    }
    pub fn prev(&self) -> Month {
        match self {
            Month::Feb => Month::Jan,
            Month::Mar => Month::Feb,
            Month::Apr => Month::Mar,
            Month::May => Month::Apr,
            Month::Jun => Month::May,
            Month::Jul => Month::Jun,
            Month::Aug => Month::Jul,
            Month::Sep => Month::Aug,
            Month::Oct => Month::Sep,
            Month::Nov => Month::Oct,
            Month::Dec => Month::Nov,
            Month::Jan => Month::Dec,
        }
    }
    pub fn day_count(&self, year: &Year) -> u32 {
        match self {
            Month::Jan => 31,
            Month::Feb => 28 + if year.is_leap() { 1 } else { 0 },
            Month::Mar => 31,
            Month::Apr => 30,
            Month::May => 31,
            Month::Jun => 30,
            Month::Jul => 31,
            Month::Aug => 31,
            Month::Sep => 30,
            Month::Oct => 31,
            Month::Nov => 30,
            Month::Dec => 31,
        }
    }
}

#[derive(Debug)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: u32,
    pub date: NaiveDate,
    pub first_of_month: NaiveDate,
    pub user_data: UserData,
}

fn get_user_file() -> Result<std::ffi::OsString, CalError> {
    std::env::var_os("CAL_RS_USERFILE")
        .or(std::env::var_os("HOME").map(|mut h| {
            h.push("/.config/cal-rs.toml");
            h
        }))
        .ok_or(CalError::UserFileNotFound)
}

impl Date {
    pub fn get_events(&self, check_day: u32) -> Vec<&Event> {
        self.user_data
            .events
            .iter()
            .filter(|&e| match e {
                Event::Single {
                    description: _,
                    year,
                    month,
                    day,
                } => year == &self.year && month == &self.month && day == &check_day,
                Event::Yearly {
                    description: _,
                    months,
                    day,
                } => months.iter().any(|mnt| mnt == &self.month) && day == &check_day,
                Event::YearlyByWeekday {
                    description: _,
                    months,
                    days,
                } => {
                    months.iter().any(|mnt| mnt == &self.month)
                        && days
                            .iter()
                            .map(<u32 as From<&Weekday>>::from)
                            .any(|wk: u32| wk == check_day % 7)
                }
            })
            .collect()
    }
    pub fn add_event(&mut self, e: Event) {
        self.user_data.events.push(e);
    }
    pub fn remove_event(&mut self, idx: usize) -> Event {
        self.user_data.events.swap_remove(idx)
    }
    pub fn save(&self) -> Result<(), CalError> {
        let user_filename = get_user_file()?;
        let mut output = std::fs::File::create(user_filename)?;
        write!(output, "{}", toml::to_string(&self.user_data)?)?;
        Ok(())
    }
    pub fn now() -> Result<Date, CalError> {
        let date = Local::now().date_naive();
        let year = Year(date.year());
        let month = date.month().try_into()?;
        let day = date.day();
        let first =
            NaiveDate::from_ymd_opt(date.year(), date.month(), 1).ok_or(CalError::FirstDayError)?;

        let user_filename = get_user_file()?;
        let content = std::fs::read_to_string(user_filename)?;
        let user_data: UserData = toml::from_str(&content)?;
        Ok(Date {
            year,
            month,
            day,
            date,
            first_of_month: first,
            user_data,
        })
    }
}

impl Date {
    pub fn month_string(&self) -> &str {
        match self.month {
            Month::Jan => &self.user_data.months[0],
            Month::Feb => &self.user_data.months[1],
            Month::Mar => &self.user_data.months[2],
            Month::Apr => &self.user_data.months[3],
            Month::May => &self.user_data.months[4],
            Month::Jun => &self.user_data.months[5],
            Month::Jul => &self.user_data.months[6],
            Month::Aug => &self.user_data.months[7],
            Month::Sep => &self.user_data.months[8],
            Month::Oct => &self.user_data.months[9],
            Month::Nov => &self.user_data.months[10],
            Month::Dec => &self.user_data.months[11],
        }
    }
    pub fn weekday_string(&self, w: &Weekday) -> &str {
        match w {
            Weekday::Sun => &self.user_data.weekdays[0],
            Weekday::Mon => &self.user_data.weekdays[1],
            Weekday::Tue => &self.user_data.weekdays[2],
            Weekday::Wed => &self.user_data.weekdays[3],
            Weekday::Thu => &self.user_data.weekdays[4],
            Weekday::Fri => &self.user_data.weekdays[5],
            Weekday::Sat => &self.user_data.weekdays[6],
        }
    }
    pub fn weekday_strings(&self) -> &[String; 7] {
        &self.user_data.weekdays
    }
}
