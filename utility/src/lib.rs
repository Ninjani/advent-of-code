use std::fmt::Display;
use std::fs;
use std::future::Future;
use std::time::{Duration, Instant};

use color_eyre::Result;

/// Adapted from https://github.com/ggovan/advent-of-code
pub fn time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let res = f();
    (res, Instant::now() - start)
}

pub async fn time_async<T, F: Future<Output = T>>(f: impl FnOnce() -> F) -> (T, Duration) {
    let start = Instant::now();
    let res = f().await;
    (res, Instant::now() - start)
}

pub trait AocDay<'a> {
    type Input: 'a;
    type Result1: Display;
    type Result2: Display;

    fn day() -> usize;
    fn year() -> usize;
    fn load(input: &'a str) -> Result<Self::Input>;
    fn part_1(input: &Self::Input) -> Result<Self::Result1>;
    fn part_2(input: &Self::Input) -> Result<Self::Result2>;

    fn read_input() -> Result<String> {
        Ok(fs::read_to_string(format!(
            "data/{}/day{:02}.txt",
            Self::year(),
            Self::day()
        ))?)
    }

    fn run(input: &'a str) -> Result<Vec<String>> {
        let mut output = vec![];
        let (input, t_l) = time(|| Self::load(&input));
        let input = input?;
        let (res_1, t_1) = time(|| Self::part_1(&input));
        let (res_2, t_2) = time(|| Self::part_2(&input));
        output.push(format!("Day {}", Self::day()));
        output.push(format!("  input loaded in {:?}", t_l));
        output.push(format!("  part 1: {} in {:?}", res_1?, t_1));
        output.push(format!("  part 2: {} in {:?}", res_2?, t_2));
        output.push("".to_string());
        for line in output.iter() {
            println!("{}", line);
        }
        Ok(output)
    }
}
