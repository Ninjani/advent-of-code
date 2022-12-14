use memchr::Memchr2;

use utility::AocDay;

pub struct Day04;

impl<'a> AocDay<'a> for Day04 {
    type Input = Vec<[Option<&'a str>; 7]>;

    type Result1 = usize;

    type Result2 = usize;

    fn day() -> usize {
        4
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        let mut passports = Vec::new();
        let mut passport = [None, None, None, None, None, None, None];
        let mut start = 0;
        for end in Memchr2::new(b'\n', b' ', input.as_bytes()) {
            if start == end {
                passports.push(passport);
                passport = [None, None, None, None, None, None, None];
            } else {
                let index = match &input[start..start + 3] {
                    "byr" => 0,
                    "iyr" => 1,
                    "eyr" => 2,
                    "hgt" => 3,
                    "hcl" => 4,
                    "ecl" => 5,
                    "pid" => 6,
                    _ => 7,
                };
                if index < 7 {
                    passport[index] = Some(&input[start + 4..end]);
                };
            }
            start = end + 1;
        }
        passports.push(passport);
        Ok(passports)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(input
            .iter()
            .filter(|passport| passport.iter().all(|val| val.is_some()))
            .count())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok(input
            .into_iter()
            .filter(|passport| validate(passport))
            .count())
    }
}

fn validate(passport: &&[Option<&str>; 7]) -> bool {
    match passport {
        [Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)] => {
            byr.len() == 4
                && iyr.len() == 4
                && eyr.len() == 4
                && hgt.len() > 2
                && hcl.len() == 7
                && pid.len() == 9
                && pid.chars().all(char::is_numeric)
                && hcl.as_bytes()[0] == b'#'
                && hcl.chars().skip(1).all(char::is_alphanumeric)
                && byr
                    .parse::<u32>()
                    .map_or(false, |byr| (1920..=2002).contains(&byr))
                && iyr
                    .parse::<u32>()
                    .map_or(false, |iyr| (2010..=2020).contains(&iyr))
                && eyr
                    .parse::<u32>()
                    .map_or(false, |eyr| (2020..=2030).contains(&eyr))
                && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl)
                && hgt[..hgt.len() - 2].parse::<u8>().map_or(false, |height| {
                    match &hgt[hgt.len() - 2..] {
                        "cm" => (150..=193).contains(&height),
                        "in" => (59..=76).contains(&height),
                        _ => false,
                    }
                })
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
        let input = Day04::load(input)?;
        assert_eq!(Day04::part_1(&input)?, 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;
        let input = Day04::load(input)?;
        assert_eq!(Day04::part_2(&input)?, 0);
        let input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
        let input = Day04::load(input)?;
        assert_eq!(Day04::part_2(&input)?, 4);
        Ok(())
    }
}
