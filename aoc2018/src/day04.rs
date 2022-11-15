use std::collections::HashMap;

use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use itertools::Itertools;
use ndarray::{s, Array2, Axis};

use utility::AocDay;

pub struct Day04;

impl<'a> AocDay<'a> for Day04 {
    type Input = Vec<Entry>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        4
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| parse_line(line))
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut input = input.to_vec();
        input.sort_by(|a, b| (a.date).cmp(&(b.date)));
        let (guard_to_date, times) = get_matrix(&input);
        let (_, max_guard) = guard_to_date
            .iter()
            .map(|(g_id, dates)| (times.select(Axis(0), &dates).sum(), g_id))
            .max()
            .unwrap();
        let (_max_count, max_minute) =
            get_max_value_index(&times.select(Axis(0), &guard_to_date[&max_guard]));
        Ok(max_minute * max_guard)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut input = input.to_vec();
        input.sort_by(|a, b| (a.date).cmp(&(b.date)));
        let (guard_to_date, times) = get_matrix(&input);
        let ((_max_count, max_minute), max_guard) = guard_to_date
            .iter()
            .map(|(g_id, indices)| (get_max_value_index(&times.select(Axis(0), indices)), g_id))
            .max()
            .unwrap();
        Ok(max_minute * max_guard)
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    guard_id: Option<usize>,
    date: DateTime<Utc>,
    minute: usize,
    is_asleep: bool,
}

fn parse_line(line: &str) -> color_eyre::Result<Entry> {
    let (date_time, entry) = line.split("] ").collect_tuple().unwrap();
    let mut date = Utc.datetime_from_str(&date_time[1..], "%Y-%m-%d %H:%M")?;
    if date.hour() > 0 {
        date = (date + Duration::days(1)).date().and_hms(0, 0, 0);
    }
    let guard_id = if entry.contains('#') {
        Some(
            entry
                .split(' ')
                .filter(|x| x.contains('#'))
                .map(|x| x[1..].parse::<usize>())
                .next()
                .unwrap()?,
        )
    } else {
        None
    };
    Ok(Entry {
        guard_id,
        minute: date.minute() as usize,
        date,
        is_asleep: entry.contains("asleep"),
    })
}

fn get_matrix(entries: &[Entry]) -> (HashMap<usize, Vec<usize>>, Array2<usize>) {
    let mut guard_to_date = HashMap::new();
    let (min_date, max_date) = (entries[0].date, entries[entries.len() - 1].date);
    let mut times = Array2::zeros(((max_date - min_date).num_days() as usize + 1, 60));
    for i in 0..entries.len() {
        let days = (entries[i].date - min_date).num_days() as usize;
        if let Some(guard_id) = entries[i].guard_id {
            guard_to_date
                .entry(guard_id)
                .or_insert_with(Vec::new)
                .push(days);
        } else if i < entries.len() - 1 && entries[i].is_asleep && !entries[i + 1].is_asleep {
            times
                .slice_mut(s![days, entries[i].minute..entries[i + 1].minute])
                .fill(1);
        }
    }
    (guard_to_date, times)
}

fn get_max_value_index(array: &Array2<usize>) -> (usize, usize) {
    (0..array.shape()[1])
        .map(|i| (array.column(i).sum(), i))
        .max()
        .unwrap()
}
