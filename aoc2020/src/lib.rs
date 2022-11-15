extern crate core;

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
        _ => unimplemented!("Day {} is not implemented yet", day),
    }?;
    Ok(())
}

#[inline(always)]
pub fn parse_usizes(input: &[u8]) -> Vec<usize> {
    input
        .split(|&b| b == b'\n')
        .map(|line| parse_usize(line))
        .collect()
}

#[inline(always)]
pub fn parse_usize(input: &[u8]) -> usize {
    input.iter().fold(0, |a, &c| a * 10 + (c & 0x0f) as usize)
}
