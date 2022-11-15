use utility::AocDay;

use crate::parse_usizes;

pub struct Day10;

impl<'a> AocDay<'a> for Day10 {
    type Input = Vec<usize>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        10
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(parse_usizes(input.as_bytes()))
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut adaptors = input.clone();
        adaptors.sort_unstable();
        let (_, d1, d3) =
            adaptors
                .into_iter()
                .fold((0, 0, 0), |(prev, mut d1, mut d3), current| {
                    match current - prev {
                        1 => d1 += 1,
                        3 => d3 += 1,
                        _ => (),
                    }
                    (current, d1, d3)
                });
        Ok(d1 * (d3 + 1))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut adaptors = vec![0];
        adaptors.extend_from_slice(input);
        adaptors.sort_unstable();
        adaptors.push(adaptors[adaptors.len() - 1] + 3);
        let length = adaptors.len();
        let mut paths = [0; 100];
        paths[length - 1] = 1;
        for (i, adaptor) in adaptors.iter().enumerate().rev().skip(1) {
            paths[i] = (i + 1..i + 4)
                .map(|j| {
                    if j < length && adaptors[j] - adaptor <= 3 {
                        paths[j]
                    } else {
                        0
                    }
                })
                .sum();
        }
        Ok(paths[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day10::load(
            r#"16
10
15
5
1
11
7
19
6
12
4"#,
        )?;
        assert_eq!(Day10::part_1(&input)?, 35);
        let input = Day10::load(
            r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#,
        )?;
        assert_eq!(Day10::part_1(&input)?, 220);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day10::load(
            r#"16
10
15
5
1
11
7
19
6
12
4"#,
        )?;
        assert_eq!(Day10::part_2(&input)?, 8);
        let input = Day10::load(
            r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#,
        )?;
        assert_eq!(Day10::part_2(&input)?, 19208);
        Ok(())
    }
}
