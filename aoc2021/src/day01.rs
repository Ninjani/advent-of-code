use itertools::Itertools;

use utility::AocDay;

pub struct Day01;

impl<'a> AocDay<'a> for Day01 {
    type Input = Vec<i32>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        1
    }
    fn year() -> usize {
        2021
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut previous_measurement = 0;
        let mut increased = 0;
        for measurement in input {
            if *measurement > previous_measurement {
                increased += 1;
            }
            previous_measurement = *measurement;
        }
        Ok(increased - 1)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut previous_sum = 0;
        let mut increased = 0;
        for (m1, m2, m3) in input.iter().tuple_windows() {
            let sum: i32 = m1 + m2 + m3;
            if sum > previous_sum {
                increased += 1;
            }
            previous_sum = sum;
        }
        Ok(increased - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day01::load(
            r#"199
200
208
210
200
207
240
269
260
263"#,
        )?;
        assert_eq!(Day01::part_1(&input)?, 7);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day01::load(
            r#"199
200
208
210
200
207
240
269
260
263"#,
        )?;
        assert_eq!(Day01::part_2(&input)?, 5);
        Ok(())
    }
}
