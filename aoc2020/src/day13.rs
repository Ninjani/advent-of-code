use utility::AocDay;

use crate::parse_usize;

pub struct Day13;

impl<'a> AocDay<'a> for Day13 {
    type Input = &'a [u8];
    type Result1 = usize;
    type Result2 = i64;

    fn day() -> usize {
        13
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        Ok(input.as_bytes())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let index = memchr::memchr(b'\n', input).unwrap();
        let earliest = parse_usize(&input[..index]);
        let (closest_bus, time) = input[index + 1..]
            .split(|&b| b == b',')
            .filter(|b| b != b"x")
            .map(|b| {
                let b = parse_usize(b);
                let nearest = b * (earliest / b);
                if nearest >= earliest {
                    (b, nearest)
                } else {
                    (b, nearest + b)
                }
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        Ok(closest_bus * (time - earliest))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut modulii_residuals = Vec::with_capacity(10);
        let mut prod = 1;
        input[memchr::memchr(b'\n', input).unwrap() + 1..]
            .split(|&b| b == b',')
            .enumerate()
            .filter(|(_, b)| b != b"x")
            .for_each(|(i, b)| {
                let modulus = parse_usize(b) as i64;
                modulii_residuals.push((modulus, modulus - i as i64 + modulus));
                prod *= modulus;
            });
        #[inline(always)]
        fn egcd(a: i64, b: i64) -> (i64, i64) {
            if a == 0 {
                (0, 1)
            } else {
                let (x, y) = egcd(b % a, a);
                (y - (b / a) * x, x)
            }
        }
        Ok(modulii_residuals
            .into_iter()
            .fold(0, |sum, (modulus, residual)| {
                let p = prod / modulus;
                sum + residual * ((egcd(p, modulus).0 % modulus + modulus) % modulus) * p
            })
            % prod)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day13::load(
            r#"939
7,13,x,x,59,x,31,19"#,
        )?;
        assert_eq!(Day13::part_1(&input)?, 295);
        Ok(())
    }

    #[test]
    fn test_part2_1() -> color_eyre::Result<()> {
        let input = Day13::load(
            r#"939
7,13,x,x,59,x,31,19"#,
        )?;
        assert_eq!(Day13::part_2(&input)?, 1068781);
        Ok(())
    }

    #[test]
    fn test_part2_2() -> color_eyre::Result<()> {
        let input = Day13::load(
            r#"939
        17,x,13,19"#,
        )?;
        assert_eq!(Day13::part_2(&input)?, 3417);
        Ok(())
    }

    #[test]
    fn test_part2_3() -> color_eyre::Result<()> {
        let input = Day13::load(
            r#"939
67,7,59,61"#,
        )?;
        assert_eq!(Day13::part_2(&input)?, 754018);
        Ok(())
    }

    #[test]
    fn test_part2_4() -> color_eyre::Result<()> {
        let input = Day13::load(
            r#"939
67,x,7,59,61"#,
        )?;
        assert_eq!(Day13::part_2(&input)?, 779210);
        Ok(())
    }

    #[test]
    fn test_part2_5() -> color_eyre::Result<()> {
        let input = Day13::load(
            r#"939
67,7,x,59,61"#,
        )?;
        assert_eq!(Day13::part_2(&input)?, 1261476);
        Ok(())
    }

    #[test]
    fn test_part2_6() -> color_eyre::Result<()> {
        let input = Day13::load(
            r#"939
1789,37,47,1889"#,
        )?;
        assert_eq!(Day13::part_2(&input)?, 1202161486);
        Ok(())
    }
}
