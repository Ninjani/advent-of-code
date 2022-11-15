use std::collections::HashSet;

use utility::AocDay;

pub struct Day01;

impl<'a> AocDay<'a> for Day01 {
    type Input = Vec<isize>;
    type Result1 = isize;
    type Result2 = isize;

    fn day() -> usize {
        1
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .map(|s| s.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(input.iter().map(|i| *i).sum())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let (mut frequencies, mut current_frequency) = (HashSet::new(), 0);
        frequencies.insert(current_frequency);
        input.iter().cycle().find(|c| {
            current_frequency += **c;
            !frequencies.insert(current_frequency) // insert returns false if key exists
        });
        Ok(current_frequency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day01::load("+1\n+1\n+1")?;
        assert_eq!(Day01::part_1(&input)?, 3);
        let input = Day01::load("+1\n-2\n+3\n+1")?;
        assert_eq!(Day01::part_1(&input)?, 3);
        let input = Day01::load("+1\n+1\n-2")?;
        assert_eq!(Day01::part_1(&input)?, 0);
        let input = Day01::load("-1\n-2\n-3")?;
        assert_eq!(Day01::part_1(&input)?, -6);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day01::load("+1\n-1")?;
        assert_eq!(Day01::part_2(&input)?, 0);
        let input = Day01::load("+3\n+3\n+4\n-2\n-4")?;
        assert_eq!(Day01::part_2(&input)?, 10);
        let input = Day01::load("-6\n+3\n+8\n+5\n-6")?;
        assert_eq!(Day01::part_2(&input)?, 5);
        let input = Day01::load("+7\n+7\n-2\n-7\n-4")?;
        assert_eq!(Day01::part_2(&input)?, 14);
        Ok(())
    }
}
