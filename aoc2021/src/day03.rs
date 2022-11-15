use utility::AocDay;

pub struct Day03;

impl<'a> AocDay<'a> for Day03 {
    type Input = (usize, Vec<&'a str>);
    type Result1 = i32;
    type Result2 = i32;

    fn day() -> usize {
        3
    }
    fn year() -> usize {
        2021
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        let input = input.trim();
        let num_bits = input.split('\n').next().unwrap().len();
        Ok((num_bits, input.split('\n').collect()))
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let (num_bits, bit_strings) = input;
        let mut num_ones = vec![0; *num_bits];
        let mut total = 0i32;
        for line in bit_strings {
            for (i, bit) in line.chars().map(|c| c.to_digit(2)).enumerate() {
                num_ones[i] += bit.unwrap() as i32;
            }
            total += 1;
        }
        let mut gamma = String::new();
        let mut epsilon = String::new();
        for n in num_ones {
            if n <= total - n {
                gamma.push('0');
                epsilon.push('1');
            } else {
                gamma.push('1');
                epsilon.push('0');
            }
        }
        Ok(i32::from_str_radix(&gamma, 2)? * i32::from_str_radix(&epsilon, 2)?)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let (num_bits, bit_strings) = input;
        let bit_strings: Vec<_> = bit_strings
            .iter()
            .map(|x| x.to_string().into_bytes())
            .collect();
        let oxygen_index = get_bit_rating(&bit_strings, *num_bits, true);
        let co2_index = get_bit_rating(&bit_strings, *num_bits, false);
        Ok(
            i32::from_str_radix(std::str::from_utf8(&bit_strings[oxygen_index])?, 2)?
                * i32::from_str_radix(std::str::from_utf8(&bit_strings[co2_index])?, 2)?,
        )
    }
}

fn get_bit_rating(bit_strings: &[Vec<u8>], num_bits: usize, most: bool) -> usize {
    let mut indices: Vec<_> = (0usize..bit_strings.len()).collect();
    for i in 0..num_bits {
        let num_ones: i32 = indices
            .iter()
            .map(|j| if bit_strings[*j][i] == b'1' { 1 } else { 0 })
            .sum();
        let total = indices.len() as i32;
        indices = indices
            .into_iter()
            .filter(|&j| {
                if num_ones < (total - num_ones) {
                    if most {
                        bit_strings[j][i] == b'0'
                    } else {
                        bit_strings[j][i] == b'1'
                    }
                } else if most {
                    bit_strings[j][i] == b'1'
                } else {
                    bit_strings[j][i] == b'0'
                }
            })
            .collect();
        if indices.len() == 1 {
            break;
        }
    }
    indices[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day03::load(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        )?;
        assert_eq!(Day03::part_1(&input)?, 198);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day03::load(
            r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
        )?;
        assert_eq!(Day03::part_2(&input)?, 230);
        Ok(())
    }
}
