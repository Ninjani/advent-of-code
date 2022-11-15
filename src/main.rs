use std::env;

use color_eyre::eyre::eyre;
use color_eyre::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let year = args
        .get(1)
        .ok_or(eyre!("No year specified"))?
        .parse::<usize>()?;
    let day = args
        .get(2)
        .ok_or(eyre!("No day specified"))?
        .parse::<usize>()?;
    run(year, day)
}

fn run(year: usize, day: usize) -> Result<()> {
    match year {
        2018 => aoc2018::run(day),
        2019 => aoc2019::run(day),
        2020 => aoc2020::run(day),
        2021 => aoc2021::run(day),
        2022 => aoc2022::run(day),
        _ => Err(eyre!("Not implemented")),
    }
}
