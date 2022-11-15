#[macro_use]
extern crate strum_macros;

use utility::AocDay;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day18;
pub mod day19;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn run(day: usize) -> color_eyre::Result<()> {
    match day {
        1 => day01::Day01::run(&day01::Day01::read_input()?),
        2 => day02::Day02::run(&day02::Day02::read_input()?),
        3 => day03::Day03::run(&day03::Day03::read_input()?),
        4 => day04::Day04::run(&day04::Day04::read_input()?),
        5 => day05::Day05::run(&day05::Day05::read_input()?),
        6 => day06::Day06::run(&day06::Day06::read_input()?),
        7 => day07::Day07::run(&day07::Day07::read_input()?),
        8 => day08::Day08::run(&day08::Day08::read_input()?),
        9 => day09::Day09::run(&day09::Day09::read_input()?),
        10 => day10::Day10::run(&day10::Day10::read_input()?),
        11 => day11::Day11::run(&day11::Day11::read_input()?),
        12 => day12::Day12::run(&day12::Day12::read_input()?),
        13 => day13::Day13::run(&day13::Day13::read_input()?),
        14 => day14::Day14::run(&day14::Day14::read_input()?),
        15 => day15::Day15::run(&day15::Day15::read_input()?),
        16 => day16::Day16::run(&day16::Day16::read_input()?),
        18 => day18::Day18::run(&day18::Day18::read_input()?),
        19 => day19::Day19::run(&day19::Day19::read_input()?),
        23 => day23::Day23::run(&day23::Day23::read_input()?),
        24 => day24::Day24::run(&day24::Day24::read_input()?),
        25 => day25::Day25::run(&day25::Day25::read_input()?),
        _ => unimplemented!("Day {} is not implemented", day),
    }?;
    Ok(())
}
