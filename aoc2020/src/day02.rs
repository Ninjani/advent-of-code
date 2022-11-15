use itertools::Itertools;

use utility::AocDay;

use crate::parse_usize;

pub struct Day02;

impl<'a> AocDay<'a> for Day02 {
    type Input = Vec<Policy<'a>>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        2
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .as_bytes()
            .split(|b| b == &b'\n')
            .map(|line| {
                let (n1_n2, letter, _, password) = line
                    .split(|b| b == &b' ' || b == &b':')
                    .collect_tuple()
                    .unwrap();
                let (n1, n2) = n1_n2
                    .split(|b| b == &b'-')
                    .map(|n| parse_usize(n))
                    .collect_tuple()
                    .unwrap();
                Policy {
                    n1,
                    n2,
                    letter: letter[0],
                    password,
                }
            })
            .collect())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(input
            .iter()
            .filter(|policy| {
                let count: usize = bytecount::count(&policy.password, policy.letter);
                count >= policy.n1 && count <= policy.n2
            })
            .count())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok(input
            .iter()
            .filter(|policy| {
                (policy.password[policy.n1 - 1] == policy.letter)
                    != (policy.password[policy.n2 - 1] == policy.letter)
            })
            .count())
    }
}

pub struct Policy<'a> {
    n1: usize,
    n2: usize,
    letter: u8,
    password: &'a [u8],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day02::load("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")?;
        assert_eq!(Day02::part_1(&input)?, 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day02::load("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")?;
        assert_eq!(Day02::part_2(&input)?, 1);
        Ok(())
    }
}
