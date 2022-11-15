// pub mod day01;

use utility::AocDay;

pub mod day01;
pub mod day02;
pub mod day03;

pub fn run(day: usize) -> color_eyre::Result<()> {
    match day {
        1 => day01::Day01::run(&day01::Day01::read_input()?),
        2 => day02::Day02::run(&day02::Day02::read_input()?),
        3 => day03::Day03::run(&day03::Day03::read_input()?),
        _ => unimplemented!("Day {} is not implemented yet", day),
    }?;
    Ok(())
}
