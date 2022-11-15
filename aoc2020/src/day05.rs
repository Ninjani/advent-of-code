use utility::AocDay;

pub struct Day05;

impl<'a> AocDay<'a> for Day05 {
    type Input = &'a [u8];
    type Result1 = u32;
    type Result2 = u32;

    fn day() -> usize {
        5
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        Ok(input.as_bytes())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(input.chunks(11).fold(0, |max, seat| max.max(get_id(&seat))))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let (sum, min, max) = input
            .chunks(11)
            .fold((0, 1024, 0), |(sum, min, max), seat| {
                let id = get_id(&seat);
                (sum + id, min.min(id), max.max(id))
            });
        Ok((min..max + 1).sum::<u32>() - sum)
    }
}

#[inline(always)]
fn get_id(seat: &[u8]) -> u32 {
    seat.iter().take(10).fold(0, |acc, &b| {
        (acc << 1) | if b == b'F' || b == b'L' { 0 } else { 1 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> color_eyre::Result<()> {
        assert_eq!(
            Day05::part_1(&Day05::load("FBFBBFFRLR\nBFFFBBFRRR\nBBFFBBFRLL")?)?,
            820
        );
        assert_eq!(Day05::part_1(&Day05::load("BFFFBBFRRR")?)?, 567);
        assert_eq!(Day05::part_1(&Day05::load("BBFFBBFRLL")?)?, 820);
        Ok(())
    }
}
