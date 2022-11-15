use utility::AocDay;

use crate::intcode_compiler::IntCode;

pub struct Day05;

impl<'a> AocDay<'a> for Day05 {
    type Input = Vec<i64>;

    type Result1 = i64;

    type Result2 = i64;

    fn day() -> usize {
        5
    }
    fn year() -> usize {
        2019
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split(',')
            .map(|i| i.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut intcode = IntCode::new(input.to_vec(), 1);
        Ok(intcode.process()?.unwrap())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut intcode = IntCode::new(input.to_vec(), 5);
        Ok(intcode.process()?.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> color_eyre::Result<()> {
        let program = Day05::load("1002,4,3,4,33")?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(intcode.program[4], 99);

        let program = Day05::load("1101,100,-1,4,0")?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(intcode.program[4], 99);
        Ok(())
    }
}
