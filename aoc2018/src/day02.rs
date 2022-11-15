use utility::AocDay;

pub struct Day02;

impl<'a> AocDay<'a> for Day02 {
    type Input = Vec<Vec<char>>;
    type Result1 = usize;
    type Result2 = String;

    fn day() -> usize {
        2
    }
    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input.split('\n').map(|s| s.chars().collect()).collect())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let alphabet: Vec<_> = (b'a'..=b'z').map(|x| x as char).collect();
        let letter_counts: Vec<_> = input
            .iter()
            .map(|w| {
                (
                    is_letter_count(w, &alphabet, 2),
                    is_letter_count(w, &alphabet, 3),
                )
            })
            .collect();
        Ok(letter_counts.iter().filter(|(x, _)| *x).count()
            * letter_counts.into_iter().filter(|(_, x)| *x).count())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        for i in 0..(input.len() - 1) {
            for j in (i + 1)..input.len() {
                if get_hamming(&input[i], &input[j]) == 1 {
                    return Ok(input[i]
                        .iter()
                        .zip(input[j].iter())
                        .filter(|(c1, c2)| c1 == c2)
                        .map(|(c, _)| *c)
                        .collect());
                }
            }
        }
        Ok(String::new())
    }
}

fn get_hamming(word_1: &[char], word_2: &[char]) -> usize {
    word_1
        .iter()
        .zip(word_2.iter())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn is_letter_count(word_chars: &[char], alphabet: &[char], number: usize) -> bool {
    for letter in alphabet {
        if word_chars.iter().filter(|c| *c == letter).count() == number {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day02::load("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab")?;
        assert_eq!(Day02::part_1(&input)?, 12);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day02::load(
            r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#,
        )?;
        assert_eq!(Day02::part_2(&input)?, "fgij");
        Ok(())
    }
}
