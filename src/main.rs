extern crate chrono;
extern crate colored;
#[macro_use]
extern crate structopt;

use chrono::naive::{IsoWeek, NaiveDate};
use chrono::{Datelike, Local, Weekday};
use colored::Colorize;
use std::env;
use structopt::StructOpt;

trait WeekExt: Sized {
    fn first_date(self) -> Option<NaiveDate>;
    fn last_date(self) -> Option<NaiveDate>;
    fn next_week(self) -> Option<Self>;
}

impl WeekExt for IsoWeek {
    fn first_date(self) -> Option<NaiveDate> {
        NaiveDate::from_isoywd_opt(self.year(), self.week(), Weekday::Mon)
    }

    fn last_date(self) -> Option<NaiveDate> {
        NaiveDate::from_isoywd_opt(self.year(), self.week(), Weekday::Sun)
    }

    fn next_week(self) -> Option<IsoWeek> {
        let date = self.first_date()?;
        Some(
            date.checked_add_signed(chrono::Duration::weeks(1))?
                .iso_week(),
        )
    }
}

trait IsInMonthExt {
    fn is_in_month(self, month: Month) -> bool;
}

impl IsInMonthExt for NaiveDate {
    fn is_in_month(self, month: Month) -> bool {
        self.year() == month.year && self.month() == month.month
    }
}

impl IsInMonthExt for IsoWeek {
    fn is_in_month(self, month: Month) -> bool {
        self.first_date()
            .map(|day| day.is_in_month(month))
            .unwrap_or(false)
            || self
                .last_date()
                .map(|day| day.is_in_month(month))
                .unwrap_or(false)
    }
}

#[derive(Copy, Clone)]
struct Month {
    year: i32,
    month: u32,
}

impl Month {
    fn first_date(self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, self.month, 1)
    }

    fn first_week(self) -> Option<IsoWeek> {
        Some(self.first_date()?.iso_week())
    }

    fn pred(self) -> Month {
        if self.month == 1 {
            Month {
                year: self.year.checked_sub(1).unwrap(),
                month: 12,
            }
        } else {
            Month {
                year: self.year,
                month: self.month - 1,
            }
        }
    }

    fn succ(self) -> Month {
        if self.month == 12 {
            Month {
                year: self.year.checked_add(1).unwrap(),
                month: 1,
            }
        } else {
            Month {
                year: self.year,
                month: self.month + 1,
            }
        }
    }
}

fn weekdays(week: IsoWeek) -> impl Iterator<Item = NaiveDate> {
    let mut date = week.first_date();
    (0..7).filter_map(move |_| {
        let result = date;
        date = date.and_then(|date| date.succ_opt());
        result
    })
}

fn render_week(month: Month, week: IsoWeek, today: NaiveDate) -> String {
    let mut output = String::with_capacity(7 * 3 + 2);
    let week_str = format!("{:>2} ", week.week()).dimmed().to_string();
    output.push_str(&week_str);

    for date in weekdays(week) {
        let mut curline = format!("{:>3}", date.day());

        if !date.is_in_month(month) {
            curline = curline.dimmed().italic().to_string();
        } else if date == today {
            curline = curline.red().bold().to_string();
        }
        output.push_str(&curline);
    }
    output
}

fn render_month(month: Month, today: NaiveDate) -> Option<[String; 8]> {
    let mut cur_week = month.first_week()?;
    let mut output = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];

    output[0] = format!(
        "   {:^21}",
        format!(" {} ", month.first_date()?.format("%B"))
    );

    output[1] = "    Mo Tu We Th Fr Sa Su".to_string();
    let mut pos = 2;
    while cur_week.is_in_month(month) {
        output[pos] = render_week(month, cur_week, today);
        cur_week = cur_week.next_week()?;
        pos += 1;
    }

    for o in &mut output[pos..] {
        *o = "                        ".to_string();
    }

    Some(output)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "date-stuff", about = "A better cal replacement")]
struct Opt {
    #[structopt(short = "b")]
    before: Option<u32>,
    #[structopt(short = "a")]
    after: Option<u32>,
    #[structopt(short = "c")]
    context: Option<u32>,
    year: Option<i32>,
    month: Option<u32>,
}

#[derive(Debug)]
enum Command {
    RenderMonths {
        year: i32,
        month: u32,
        before: u32,
        after: u32,
    },
    RenderYear {
        year: i32,
    },
}

fn main() {
    let mut opt = Opt::from_args();
    let today = Local::today().naive_local();

    let get_env = |var, def| -> u32 {
        match env::var(var).map(|s| s.parse()).unwrap_or(Ok(def)) {
            Ok(v) => v,
            Err(e) => panic!("Could not parse {}: {:?}", var, e),
        }
    };

    let before_noargs = get_env("DATE_BEFORE_NOARGS", 1);
    let before_args = get_env("DATE_BEFORE_ARGS", 4);
    let after_noargs = get_env("DATE_AFTER_NOARGS", 4);
    let after_args = get_env("DATE_AFTER_ARGS", 4);

    opt.before = opt.before.or(opt.context);
    opt.after = opt.after.or(opt.context);

    let cmd = match (opt.year, opt.month) {
        (Some(year), Some(month)) => {
            let before = opt.before.unwrap_or(before_args);
            let after = opt.after.unwrap_or(after_args);
            Command::RenderMonths {
                year,
                month,
                before,
                after,
            }
        }
        (Some(year), None) => Command::RenderYear { year },
        (None, None) => {
            let year = today.year();
            let month = today.month();
            let before = opt.before.unwrap_or(before_noargs);
            let after = opt.after.unwrap_or(after_noargs);
            Command::RenderMonths {
                year,
                month,
                before,
                after,
            }
        }
        (None, Some(_month)) => unreachable!(),
    };

    let mut months = Vec::new();

    match cmd {
        Command::RenderYear { year } => {
            months.extend((1..=12).map(|month| Month { year, month }));
        }
        Command::RenderMonths {
            year,
            month,
            before,
            after,
        } => {
            let mut cur_month = Month { year, month };
            for _ in 0..before {
                cur_month = cur_month.pred();
            }
            for _ in 0..(before + 1 + after) {
                months.push(cur_month);
                cur_month = cur_month.succ();
            }
        }
    }

    let months = months
        .into_iter()
        .map(|m| render_month(m, today).unwrap())
        .collect::<Vec<_>>();

    for c in months.chunks(3) {
        match c {
            &[ref m0] => {
                for l0 in m0.iter() {
                    if !l0.trim().is_empty() {
                        println!("{}", l0);
                    }
                }
            }
            &[ref m0, ref m1] => {
                for (l0, l1) in m0.iter().zip(m1.iter()) {
                    if !(l0.trim().is_empty() && l1.trim().is_empty()) {
                        println!("{}    {}", l0, l1);
                    }
                }
            }
            &[ref m0, ref m1, ref m2] => {
                for ((l0, l1), l2) in m0.iter().zip(m1.iter()).zip(m2.iter()) {
                    if !(l0.trim().is_empty() && l1.trim().is_empty() && l2.trim().is_empty()) {
                        println!("{}    {}    {}", l0, l1, l2);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
