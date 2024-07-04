use crate::back;
use chrono::Datelike;
use color_print::cformat;
use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum CalError {
    #[error("Can't make Weekday with {0}")]
    WeekdayRange(u32),
}

impl fmt::Display for back::Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        <i32 as fmt::Display>::fmt(&self.0, f)
    }
}

impl back::Event {
    fn format(&self, ctx: &back::Date, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            back::Event::Single {
                year: _,
                month: _,
                day: d,
                description,
            } => writeln!(f, "{d} | {}", description),
            back::Event::Yearly {
                months: _,
                day: d,
                description,
            } => writeln!(f, "{d} | {}", description),
            back::Event::YearlyByWeekday {
                months: _,
                days,
                description,
            } => {
                let ws = days
                    .iter()
                    .map(|w| ctx.weekday_string(w))
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(f, "{ws} | {}", description)
            }
        }
    }
}

impl fmt::Display for back::Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        {
            // header
            // "       25, June 2024       "
            // "sun mon tue wed thu fri sat"
            let month = self.month_string();
            let padding = " ".repeat((18 - month.len()) / 2);
            let year = self.year.to_string();
            let day = self.day.to_string();
            writeln!(f, "{padding}{day: >2}, {month} {year}")?;
            writeln!(
                f,
                "{}",
                self.weekday_strings()
                    .iter()
                    .fold(String::new(), |i, t| i + t + " ")
            )?;
        }

        // calendar
        // 25  26  27  28  29  30   1
        //  2   3   4   5   6   7   8
        //  9  10  11  12  13  14  15
        // 16  17  18  19  20  21  22
        // 23  24 [25] 26  27  28  29
        // 30   1   2   3   4   5   6
        let mut calendar = String::with_capacity(4 * 7 * 5);

        // days before current month
        let prev_month_count = self.month.prev().day_count(&self.year);
        let prev_appear_count = self.first_of_month.weekday().num_days_from_sunday();
        let mut dtot = prev_appear_count;
        for i in 0..prev_appear_count {
            let day = prev_month_count - (prev_appear_count - i);
            calendar.push_str(&cformat!(" <black!>{day: >2}</> "));
        }

        // this month
        let this_month_count = self.month.day_count(&self.year);
        let mut events: Vec<&back::Event> = Vec::new();
        for day in 1..=this_month_count {
            dtot += 1;
            if self.day == day {
                let mut today_events = self.get_events(day);
                let day_str = format!("[{day}]");
                if !today_events.is_empty() {
                    calendar.push_str(&cformat!("<g!>{day_str: >4}</>"));
                } else {
                    calendar.push_str(&cformat!("<w!>{day_str: >4}</>"));
                }
                events.append(&mut today_events);
            } else {
                let today_events = self.get_events(day);
                if !today_events.is_empty() {
                    calendar.push_str(&cformat!(" <g>{day: >2}</> "));
                } else {
                    calendar.push_str(&cformat!(" <w>{day: >2}</> "));
                }
            }
            if dtot % 7 == 0 {
                calendar.push('\n');
            }
        }

        // next month
        for day in 1..=7 - (dtot % 7) {
            calendar.push_str(&cformat!("<black!>{day: >3}</> "));
        }
        writeln!(f, "{}\n", calendar)?;
        events
            .into_iter()
            .try_for_each(|event| event.format(self, f))
    }
}
