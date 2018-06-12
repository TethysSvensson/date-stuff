extern crate chrono;
extern crate colored;

use chrono::naive::{IsoWeek, NaiveDate};
use chrono::{Datelike, Local, Weekday};
use colored::Colorize;

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
            || self.last_date()
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

    output[0] = format!("   {:^21}", format!("{}", month.first_date()?.format("%B")));
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

fn main() {
    let today = Local::today().naive_local();
    let render = |m| {
        render_month(
            Month {
                year: today.year(),
                month: m,
            },
            today,
        ).unwrap()
    };

    for &m in &[1, 4, 7, 10] {
        for ((l0, l1), l2) in render(m)
            .iter()
            .zip(render(m + 1).iter())
            .zip(render(m + 2).iter())
        {
            if l0.trim().len() > 0 || l1.trim().len() > 0 || l2.trim().len() > 0 {
                println!("{}    {}    {}", l0, l1, l2);
            }
        }
        println!("");
    }
}
