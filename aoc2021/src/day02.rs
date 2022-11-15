use itertools::Itertools;

use utility::AocDay;

pub struct Day02;

impl<'a> AocDay<'a> for Day02 {
    type Input = Vec<(&'a str, i32)>;
    type Result1 = i32;
    type Result2 = i32;

    fn day() -> usize {
        2
    }
    fn year() -> usize {
        2021
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .map(|line| {
                let (command, amount) = line.split(' ').collect_tuple().unwrap();
                (command, amount.parse().unwrap())
            })
            .collect())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut depth = 0;
        let mut position = 0;
        for (command, amount) in input {
            match *command {
                "forward" => position += *amount,
                "down" => depth += *amount,
                "up" => depth -= *amount,
                _ => panic!("Unknown command"),
            }
        }
        Ok(depth * position)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut depth = 0;
        let mut position = 0;
        let mut aim = 0;
        for (command, amount) in input {
            match *command {
                "forward" => {
                    position += *amount;
                    depth += aim * *amount;
                }
                "down" => aim += *amount,
                "up" => aim -= *amount,
                _ => panic!("Unknown command"),
            }
        }
        Ok(depth * position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day02::load(
            r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#,
        )?;
        assert_eq!(Day02::part_1(&input)?, 150);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day02::load(
            r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#,
        )?;
        assert_eq!(Day02::part_2(&input)?, 900);
        Ok(())
    }
}
