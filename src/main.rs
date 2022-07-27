use std::env;

use clap::Parser;
use color_eyre::eyre::{eyre, ContextCompat, Result, WrapErr};
use colored::Colorize;
use time::{Date, Month, Weekday};

#[derive(Copy, Clone)]
pub struct IsoWeek {
    year: i32,
    week: u8,
}

impl IsoWeek {
    pub fn new(date: Date) -> Self {
        let (year, week, _day) = date.to_iso_week_date();
        Self { year, week }
    }
    pub fn first_date(self) -> Result<Date> {
        Date::from_iso_week_date(self.year, self.week, Weekday::Monday)
            .wrap_err("Overflow when trying to find first date in week")
    }

    pub fn last_date(self) -> Result<Date> {
        Date::from_iso_week_date(self.year, self.week, Weekday::Sunday)
            .wrap_err("Overflow when trying to find last date in week")
    }

    pub fn next_week(self) -> Result<IsoWeek> {
        let date = self.first_date()?;
        Ok(IsoWeek::new(
            date.checked_add(time::Duration::WEEK)
                .wrap_err("Overflow when finding next week")?,
        ))
    }

    pub fn weekdays(self) -> Result<[Date; 7]> {
        let date = self.first_date()?;
        let mut dates = [date; 7];
        for i in 1..7 {
            dates[i] = dates[i - 1]
                .next_day()
                .ok_or_else(|| eyre!("Overflow when finding the days of the week"))?;
        }
        Ok(dates)
    }
}

#[derive(Copy, Clone)]
struct CalendarMonth {
    year: i32,
    month: Month,
}

impl CalendarMonth {
    fn first_date(self) -> Result<Date> {
        Date::from_calendar_date(self.year, self.month, 1)
            .wrap_err("Overflow when finding first date in month")
    }

    fn first_week(self) -> Result<IsoWeek> {
        Ok(IsoWeek::new(self.first_date()?))
    }

    fn contains_date(self, date: Date) -> bool {
        self.year == date.year() && self.month == date.month()
    }

    fn contains_week(self, week: IsoWeek) -> Result<bool> {
        Ok(self.contains_date(week.first_date()?) || self.contains_date(week.last_date()?))
    }

    fn previous(self) -> Result<CalendarMonth> {
        if self.month == Month::January {
            Ok(CalendarMonth {
                year: self
                    .year
                    .checked_sub(1)
                    .ok_or_else(|| eyre!("Overflow when finding the previous month"))?,
                month: Month::December,
            })
        } else {
            Ok(CalendarMonth {
                year: self.year,
                month: self.month.previous(),
            })
        }
    }

    fn next(self) -> Result<CalendarMonth> {
        if self.month == Month::December {
            Ok(CalendarMonth {
                year: self
                    .year
                    .checked_add(1)
                    .ok_or_else(|| eyre!("Overflow when finding the next month"))?,
                month: Month::January,
            })
        } else {
            Ok(CalendarMonth {
                year: self.year,
                month: self.month.next(),
            })
        }
    }

    fn render_week(self, iso_week: IsoWeek, today: Date) -> Result<String> {
        let mut output = String::with_capacity(7 * 3 + 2);
        let week_str = format!("{:>2} ", iso_week.week).dimmed().to_string();
        output.push_str(&week_str);

        for date in iso_week.weekdays()? {
            let mut curline = format!("{:>3}", date.day());

            if !self.contains_date(date) {
                curline = curline.dimmed().italic().to_string();
            } else if date == today {
                curline = curline.red().bold().to_string();
            }
            output.push_str(&curline);
        }
        Ok(output)
    }

    fn render(self, today: Date) -> Result<[String; 8]> {
        let mut cur_week = self.first_week()?;
        let mut output = [(); 8].map(|()| String::new());

        output[0] = format!("   {:^21}", format!(" {} ", self.first_date()?.month()));
        output[1] = "    Mo Tu We Th Fr Sa Su".to_string();

        for o in output[2..].iter_mut() {
            if self.contains_week(cur_week)? {
                *o = self.render_week(cur_week, today)?;
                cur_week = cur_week.next_week()?;
            } else {
                *o = "                        ".to_string();
            }
        }

        Ok(output)
    }
}

#[derive(Debug, clap::Parser)]
#[structopt(name = "date-stuff", about = "A better cal replacement")]
struct Opt {
    #[clap(short = 'b')]
    before: Option<u32>,
    #[clap(short = 'a')]
    after: Option<u32>,
    #[clap(short = 'c')]
    context: Option<u32>,
    year: Option<i32>,
    month: Option<u8>,
}

#[derive(Debug)]
enum Command {
    RenderMonths {
        year: i32,
        month: Month,
        before: u32,
        after: u32,
    },
    RenderYear {
        year: i32,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut opt = Opt::parse();
    let today = time::OffsetDateTime::now_local()?.date();

    let get_env = |var, def| -> Result<u32> {
        match env::var(var) {
            Ok(s) => s
                .parse()
                .wrap_err_with(|| eyre!("Cannot parse environment variable {var} as a u32")),
            Err(env::VarError::NotPresent) => Ok(def),
            Err(e) => {
                Err(e).wrap_err_with(|| eyre!("Cannot parse environment variable {var} as a u32"))
            }
        }
    };

    let before_noargs = get_env("DATE_BEFORE_NOARGS", 1)?;
    let before_args = get_env("DATE_BEFORE_ARGS", 4)?;
    let after_noargs = get_env("DATE_AFTER_NOARGS", 4)?;
    let after_args = get_env("DATE_AFTER_ARGS", 4)?;

    opt.before = opt.before.or(opt.context);
    opt.after = opt.after.or(opt.context);

    let cmd = match (opt.year, opt.month) {
        (Some(year), Some(month)) => {
            let before = opt.before.unwrap_or(before_args);
            let after = opt.after.unwrap_or(after_args);
            Command::RenderMonths {
                year,
                month: month.try_into().wrap_err_with(|| {
                    eyre!("The number {month} does not correspond to a valid month")
                })?,
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
            let mut month = Month::January;
            for _ in 0..12 {
                months.push(CalendarMonth { year, month });
                month = month.next();
            }
        }
        Command::RenderMonths {
            year,
            month,
            before,
            after,
        } => {
            let mut cur_month = CalendarMonth { year, month };
            for _ in 0..before {
                cur_month = cur_month.previous()?;
            }
            for _ in 0..(before + 1 + after) {
                months.push(cur_month);
                cur_month = cur_month.next()?;
            }
        }
    }

    let months = months
        .into_iter()
        .map(|m| m.render(today))
        .collect::<Result<Vec<_>>>()?;

    for c in months.chunks(3) {
        match c {
            &[ref m0] => {
                for l0 in m0 {
                    if !l0.trim().is_empty() {
                        println!("{}", l0);
                    }
                }
            }
            &[ref m0, ref m1] => {
                for (l0, l1) in m0.iter().zip(m1) {
                    if !(l0.trim().is_empty() && l1.trim().is_empty()) {
                        println!("{}    {}", l0, l1);
                    }
                }
            }
            &[ref m0, ref m1, ref m2] => {
                for ((l0, l1), l2) in m0.iter().zip(m1).zip(m2) {
                    if !(l0.trim().is_empty() && l1.trim().is_empty() && l2.trim().is_empty()) {
                        println!("{}    {}    {}", l0, l1, l2);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
