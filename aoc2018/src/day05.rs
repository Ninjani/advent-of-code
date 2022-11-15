use utility::AocDay;

pub struct Day05;

impl<'a> AocDay<'a> for Day05 {
    type Input = String;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        5
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input.trim().to_string())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(react_polymer(&input, None))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok((b'a'..=b'z')
            .map(|a| react_polymer(&input, Some(a as char)))
            .min()
            .unwrap())
    }
}

fn react_polymer(input: &str, minus_letter: Option<char>) -> usize {
    let mut output = Vec::new();
    for character in input.chars() {
        if Some(character.to_ascii_lowercase()) == minus_letter {
            // ignore letter
        } else if !output.is_empty() && is_opposite_polarity(character, *output.last().unwrap()) {
            output.pop();
        } else {
            output.push(character);
        }
    }
    output.len()
}

fn is_opposite_polarity(c1: char, c2: char) -> bool {
    (c1 != c2) && (c1.to_ascii_lowercase() == c2.to_ascii_lowercase())
}
