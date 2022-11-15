use itertools::Itertools;

use utility::AocDay;

use crate::parse_usizes;

pub struct Day09;

impl<'a> AocDay<'a> for Day09 {
    type Input = Vec<usize>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        9
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(parse_usizes(input.as_bytes()))
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(first_wrong_solve(input, 25))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok(sliding_window_solve(input, first_wrong_solve(input, 25)))
    }
}

fn first_wrong_solve(numbers: &[usize], preamble_size: usize) -> usize {
    for (window, next_value) in numbers
        .windows(preamble_size)
        .zip(numbers.iter().skip(preamble_size))
    {
        if !window
            .iter()
            .enumerate()
            .any(|(i, v)| next_value > v && window[i + 1..].contains(&(next_value - v)))
        {
            return *next_value;
        }
    }
    unreachable!()
}

fn sliding_window_solve(input: &[usize], sum: usize) -> usize {
    let mut current_sum = input[0];
    let length = input.len();
    let (mut start, mut end) = (0, 1);
    loop {
        while current_sum > sum && start < end - 1 {
            current_sum -= input[start];
            start += 1;
        }
        if current_sum == sum {
            let (min, max) = input[start..end].iter().minmax().into_option().unwrap();
            return min + max;
        }
        while current_sum < sum && end < length - 1 {
            current_sum += input[end];
            end += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day09::load(
            r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#,
        )?;
        assert_eq!(first_wrong_solve(&input, 5), 127);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day09::load(
            r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#,
        )?;
        let wrong_number = first_wrong_solve(&input, 5);
        assert_eq!(sliding_window_solve(&input, wrong_number), 62);
        Ok(())
    }
}
