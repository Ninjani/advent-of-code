use color_eyre::eyre::eyre;

use utility::AocDay;

pub struct Day04;

impl<'a> AocDay<'a> for Day04 {
    type Input = (u32, u32);
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        4
    }
    fn year() -> usize {
        2019
    }
    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        let mut ranges = input.split('-');
        let min_password = ranges.next().unwrap().parse::<u32>()?;
        let max_password = ranges.next().unwrap().parse::<u32>()?;
        Ok((min_password, max_password))
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(generate_candidates_1(input.0, input.1)?.len())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok(generate_candidates_2(input.0, input.1)?.len())
    }
}

/// TODO This takes over a second. Unacceptable!

fn number_to_digits(number: u32) -> color_eyre::Result<Vec<u32>> {
    Ok(number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).ok_or_else(|| eyre!("not a number")))
        .collect::<Result<Vec<_>, _>>()?)
}

fn generate_candidates_1(min_password: u32, max_password: u32) -> color_eyre::Result<Vec<u32>> {
    let mut candidates = Vec::new();
    for candidate in min_password..max_password {
        if check_password_1(&number_to_digits(candidate)?) {
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

fn check_password_1(candidate: &[u32]) -> bool {
    if candidate.len() != 6 {
        return false;
    }
    let mut previous = candidate[0];
    let mut adjacency_check = false;
    let mut increasing_check = true;
    for c in candidate.iter().skip(1) {
        if *c < previous {
            increasing_check = false;
            break;
        }
        if !adjacency_check && *c == previous {
            adjacency_check = true;
        }
        previous = *c;
    }
    if !adjacency_check || !increasing_check {
        return false;
    }
    true
}

fn generate_candidates_2(min_password: u32, max_password: u32) -> color_eyre::Result<Vec<u32>> {
    let mut candidates = Vec::new();
    for candidate in min_password..max_password {
        if check_password_2(&number_to_digits(candidate)?) {
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

fn check_password_2(candidate: &[u32]) -> bool {
    if candidate.len() != 6 {
        return false;
    }
    let mut previous = candidate[0];
    let mut adjacency_check = false;
    let mut increasing_check = true;
    for i in 1..candidate.len() {
        if candidate[i] < previous {
            increasing_check = false;
            break;
        }
        if !adjacency_check && candidate[i] == previous {
            adjacency_check = !(i > 1 && candidate[i - 2] == previous
                || i < candidate.len() - 1 && candidate[i + 1] == previous);
        }
        previous = candidate[i];
    }
    if !adjacency_check || !increasing_check {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> color_eyre::Result<()> {
        assert!(check_password_1(&number_to_digits(111111)?));
        assert!(!check_password_1(&number_to_digits(223450)?));
        assert!(!check_password_1(&number_to_digits(123789)?));
        Ok(())
    }

    #[test]
    fn test_2() -> color_eyre::Result<()> {
        assert!(check_password_2(&number_to_digits(112233)?));
        assert!(!check_password_2(&number_to_digits(123444)?));
        assert!(check_password_2(&number_to_digits(111122)?));
        Ok(())
    }
}
