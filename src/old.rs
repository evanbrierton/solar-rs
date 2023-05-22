use std::fs::read_dir;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use solar_rs::{parse::read_from_file, solar_record::SolarRecord};
use tabled::{
    settings::{object::Rows, Alignment, Modify, Style},
    Table, Tabled,
};

#[derive(Clone)]
struct Day {
    date: Option<DateTime<Utc>>,
    old: f32,
    new: f32,
    savings: f32,
    production: f32,
    consumption: f32,
}

impl Day {
    fn new(
        date: Option<DateTime<Utc>>,
        old: f32,
        new: f32,
        savings: f32,
        production: f32,
        consumption: f32,
    ) -> Self {
        Self {
            date,
            old,
            new,
            savings,
            production,
            consumption,
        }
    }
}

impl Default for Day {
    fn default() -> Self {
        Self::new(None, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Tabled, Clone)]

struct TabledDay {
    date: String,
    old: String,
    new: String,
    savings: String,
    production: String,
    consumption: String,
}

impl TabledDay {
    fn new(
        date: String,
        old: f32,
        new: f32,
        savings: f32,
        production: f32,
        consumption: f32,
    ) -> Self {
        let f = |x| format!("€{:.2}", x);

        Self {
            date,
            old: f(old),
            new: f(new),
            savings: f(savings),
            production: format!("{:.2}kWh", production / 1000.0),
            consumption: format!("{:.2}kWh", consumption / 1000.0),
        }
    }
}

impl From<Day> for TabledDay {
    fn from(day: Day) -> Self {
        Self::new(
            day.date.unwrap().format("%Y/%m/%d").to_string(),
            day.old,
            day.new,
            day.savings,
            day.production,
            day.consumption,
        )
    }
}

fn main() {
    let days = read_dir("data")
        .unwrap()
        .sorted_by_key(|dir| dir.as_ref().unwrap().path())
        .map(|file| {
            let file = file.unwrap();
            let path = file.path();
            let info = read_from_file::<SolarRecord>(path.to_str().unwrap());

            match info {
                Ok(records) => records.iter().fold(Day::default(), |acc, record| {
                    let duration = match acc.date {
                        Some(prev_date) => record.time - prev_date,
                        None => chrono::Duration::minutes(5),
                    };

                    let date = Some(record.time);
                    let old = record.old_cost(&duration) + acc.old;
                    let new = record.cost(&duration) + acc.new;
                    let savings = record.savings(&duration) + acc.savings;
                    let production = record.production(&duration) + acc.production;
                    let consumption = record.consumption(&duration) + acc.consumption;

                    Day::new(date, old, new, savings, production, consumption)
                }),
                Err(e) => panic!("Error: {}", e),
            }
        })
        .collect::<Vec<_>>();

    let total = 11_000.0;
    let n_days = days.len();

    let total_savings = days.iter().map(|day| day.savings).sum::<f32>();
    let mean = total_savings / n_days as f32;

    let remaining = total - total_savings;
    let remaining_days = remaining / mean;

    let payoff_date = Utc::now() + chrono::Duration::days(remaining_days as i64);

    let Day {
        date: _,
        old,
        new,
        savings,
        production,
        consumption,
    } = days.iter().fold(Day::default(), |acc, day| {
        Day::new(
            None,
            acc.old + day.old,
            acc.new + day.new,
            acc.savings + day.savings,
            acc.production + day.production,
            acc.consumption + day.consumption,
        )
    });

    let totals = TabledDay::new(
        "Total".to_string(),
        old,
        new,
        savings,
        production,
        consumption,
    );

    let tabled_days = days
        .into_iter()
        .map(|day| day.into())
        .collect::<Vec<TabledDay>>();

    let table = Table::new([tabled_days, vec![totals]].concat())
        .with(Style::rounded())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
        .to_string();

    println!("{}", table);

    println!("Total savings: €{:.2}", total_savings,);
    println!("Mean savings: €{:.2}", mean,);
    println!("Remaining: €{:.2}", remaining,);
    println!("Remaining days: {:.2}", remaining_days,);
    println!("Payoff date: {}", payoff_date.format("%Y/%m/%d"),);
}
