use utility::AocDay;

pub struct Day14;

impl<'a> AocDay<'a> for Day14 {
    type Input = String;
    type Result1 = String;
    type Result2 = usize;

    fn day() -> usize {
        14
    }
    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input.to_string())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let num_rounds = input.parse::<usize>().unwrap();
        let mut leaderboard = vec![3, 7];
        let (mut first_index, mut second_index) = (0, 1);
        let mut total;
        loop {
            total = leaderboard[first_index] + leaderboard[second_index];
            if total >= 10 {
                leaderboard.push(total / 10);
                leaderboard.push(total % 10);
            } else {
                leaderboard.push(total);
            }
            if leaderboard.len() > num_rounds + 10 {
                return Ok(leaderboard[num_rounds..num_rounds + 10]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(""));
            }
            first_index += leaderboard[first_index] + 1;
            first_index %= leaderboard.len();
            second_index += leaderboard[second_index] + 1;
            second_index %= leaderboard.len();
        }
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let puzzle: Vec<_> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let mut leaderboard = vec![3, 7];
        let (mut first_index, mut second_index) = (0, 1);
        let mut total;
        loop {
            total = leaderboard[first_index] + leaderboard[second_index];
            if total >= 10 {
                leaderboard.push(total / 10);
                leaderboard.push(total % 10);
            } else {
                leaderboard.push(total);
            }
            if leaderboard.len() > puzzle.len() {
                if leaderboard[leaderboard.len() - puzzle.len()..] == puzzle[..] {
                    return Ok(leaderboard.len() - puzzle.len());
                } else if leaderboard[leaderboard.len() - puzzle.len() - 1..leaderboard.len() - 1]
                    == puzzle[..]
                {
                    return Ok(leaderboard.len() - puzzle.len() - 1);
                }
            }
            first_index += leaderboard[first_index] + 1;
            first_index %= leaderboard.len();
            second_index += leaderboard[second_index] + 1;
            second_index %= leaderboard.len();
        }
    }
}
