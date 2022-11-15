use utility::AocDay;

pub struct Day06;

impl<'a> AocDay<'a> for Day06 {
    type Input = &'a [u8];

    type Result1 = u32;

    type Result2 = u32;

    fn day() -> usize {
        1
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        Ok(input.as_bytes())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let (total, group, _) =
            input
                .iter()
                .fold((0, 0u32, input[0]), |(mut total, mut group, prev), c| {
                    if c != &b'\n' {
                        group |= 1 << (c - b'a');
                    } else if prev == b'\n' {
                        total += group.count_ones();
                        group = 0;
                    }
                    (total, group, *c)
                });
        Ok(total + group.count_ones())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let union_all: u32 = (b'a'..=b'z').fold(0, |union, i| union | (1 << (i - b'a')));
        let (total, group, person, _) = input.iter().fold(
            (0, union_all, 0, input[0]),
            |(mut total, mut group, mut person, prev), c| {
                if c != &b'\n' {
                    person |= 1 << (c - b'a');
                } else if prev != b'\n' {
                    group &= person;
                    person = 0;
                } else {
                    total += group.count_ones();
                    group = union_all;
                }
                (total, group, person, *c)
            },
        );
        Ok(total + (group & person).count_ones())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let input = Day06::load(input)?;
        assert_eq!(Day06::part_1(&input)?, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let input = Day06::load(input)?;
        assert_eq!(Day06::part_2(&input)?, 6);
        Ok(())
    }
}
