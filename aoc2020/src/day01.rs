use std::cmp::Ordering;

use color_eyre::eyre::eyre;

use utility::AocDay;

use crate::parse_usizes;

pub struct Day01;

impl<'a> AocDay<'a> for Day01 {
    type Input = Vec<usize>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        1
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(parse_usizes(input.as_bytes()))
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut input = input.clone();
        input.sort_unstable();
        let (i, j) = two_sum(&input, 2020).ok_or(eyre!("Not found"))?;
        Ok(i * j)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut input = input.clone();
        input.sort_unstable();
        let (i, j, k) = three_sum(&input, 2020).ok_or(eyre!("Not found"))?;
        Ok(i * j * k)
    }
}

pub fn two_sum(sorted_array: &[usize], target: usize) -> Option<(usize, usize)> {
    let length = sorted_array.len();
    let (mut start, mut end) = (0, length - 1);
    while start < end {
        let sum = sorted_array[start] + sorted_array[end];
        match sum.cmp(&target) {
            Ordering::Equal => return Some((sorted_array[start], sorted_array[end])),
            Ordering::Greater => end -= 1,
            Ordering::Less => start += 1,
        }
    }
    None
}

fn three_sum(sorted_array: &[usize], target: usize) -> Option<(usize, usize, usize)> {
    let length = sorted_array.len();
    for i in 0..=(length - 2) {
        let a = sorted_array[i];
        let (mut start, mut end) = (i + 1, length - 1);
        while start < end {
            let sum = a + sorted_array[start] + sorted_array[end];
            match sum.cmp(&target) {
                Ordering::Equal => {
                    return Some((sorted_array[i], sorted_array[start], sorted_array[end]));
                }
                Ordering::Greater => end -= 1,
                Ordering::Less => start += 1,
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day01::load("1721\n979\n366\n299\n675\n1456")?;
        assert_eq!(Day01::part_1(&input)?, 514579);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day01::load("1721\n979\n366\n299\n675\n1456")?;
        assert_eq!(Day01::part_2(&input)?, 241861950);
        Ok(())
    }

    #[test]
    fn test_half() -> color_eyre::Result<()> {
        let input = Day01::load("1010\n2020\n0\n299\n670\n14")?;
        assert_eq!(Day01::part_1(&input)?, 0);
        Ok(())
    }
}
