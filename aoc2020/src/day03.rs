use utility::AocDay;

pub struct Day03;

impl<'a> AocDay<'a> for Day03 {
    type Input = Vec<Vec<bool>>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        3
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .as_bytes()
            .split(|&b| b == b'\n')
            .map(|line| line.iter().map(|&c| c == b'#').collect())
            .collect())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(check_slope(input, 3, 1))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .map(|(slope_i, slope_j)| check_slope(input, slope_i, slope_j))
            .product())
    }
}

fn check_slope(input: &[Vec<bool>], slope_i: usize, slope_j: usize) -> usize {
    let (max_i, max_j) = (input[0].len(), input.len());
    (slope_j..max_j - slope_j + 1)
        .step_by(slope_j)
        .enumerate()
        .filter(|(index_i, index_j)| input[*index_j][((index_i + 1) * slope_i) % max_i])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day03::load("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#")?;
        assert_eq!(Day03::part_1(&input)?, 7);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day03::load("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#")?;
        assert_eq!(Day03::part_2(&input)?, 336);
        Ok(())
    }
}
