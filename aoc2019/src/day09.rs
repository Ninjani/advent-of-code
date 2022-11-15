use utility::AocDay;

use crate::intcode_compiler::IntCode;

pub struct Day09;

impl<'a> AocDay<'a> for Day09 {
    type Input = Vec<i64>;

    type Result1 = i64;

    type Result2 = i64;

    fn day() -> usize {
        9
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
        let mut intcode = IntCode::new(input.to_vec(), 2);
        Ok(intcode.process()?.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> color_eyre::Result<()> {
        let program = Day09::load("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")?;
        let mut intcode = IntCode::new(program.clone(), 1);
        intcode.process()?;
        assert_eq!(intcode.outputs, program);

        let program = Day09::load("1102,34915192,34915192,7,4,7,99,0")?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(
            intcode
                .get_last_output()
                .unwrap()
                .to_string()
                .chars()
                .count(),
            16
        );

        let program = Day09::load("104,1125899906842624,99")?;
        let mut intcode = IntCode::new(program, 1);
        intcode.process()?;
        assert_eq!(Some(1125899906842624), intcode.get_last_output());
        Ok(())
    }
}
