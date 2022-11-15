use std::collections::HashMap;

use itertools::Itertools;

use utility::AocDay;

pub struct Day09;

impl<'a> AocDay<'a> for Day09 {
    type Input = (usize, usize);

    type Result1 = usize;

    type Result2 = usize;

    fn day() -> usize {
        9
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        let parts: Vec<_> = input.split_whitespace().collect();
        Ok((parts[0].parse()?, parts[6].parse()?))
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let (num_players, num_marbles) = (input.0, input.1);
        Ok(get_max_score(num_players, num_marbles))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        Ok(get_max_score(input.0, input.1 * 100))
    }
}

pub struct GameCircle {
    circle: Vec<usize>,
    num_marbles: usize,
    num_players: usize,
    num_marbles_used: usize,
    current_marble_index: usize,
    current_player: usize,
    scores: HashMap<usize, usize>,
}

impl GameCircle {
    fn new(num_players: usize, num_marbles: usize) -> Self {
        GameCircle {
            circle: vec![0],
            num_marbles,
            num_players,
            num_marbles_used: 1,
            current_marble_index: 0,
            current_player: 1,
            scores: HashMap::new(),
        }
    }

    fn move_player(&mut self, offset: usize) {
        self.current_player += offset;
        if self.current_player > self.num_players {
            self.current_player -= self.num_players;
        }
    }

    fn add_score(&mut self, score: usize) {
        *self.scores.entry(self.current_player).or_insert(0) += score;
    }

    fn get_circular_index(&self, offset: isize) -> usize {
        let new_index = offset;
        if new_index >= 0 && new_index < self.circle.len() as isize {
            new_index as usize
        } else if new_index < 0 {
            self.circle.len() - (new_index.abs() as usize)
        } else {
            (new_index as usize) - self.circle.len()
        }
    }

    fn interleave(&mut self, interleave_index: usize, num_to_add: usize) {
        self.circle = [
            &self.circle[..interleave_index],
            &(interleave_index..self.circle.len())
                .map(|i| self.circle[i])
                .interleave((0..num_to_add).map(|i| i + self.num_marbles_used))
                .collect::<Vec<_>>(),
        ]
        .concat();
        self.num_marbles_used += num_to_add;
        self.current_marble_index = interleave_index + num_to_add * 2 - 1;
        self.move_player(num_to_add + 1);
    }

    fn next_circle(&mut self) {
        let interleave_index = self.get_circular_index(self.current_marble_index as isize + 1);
        let num_to_add =
            (self.circle.len() - interleave_index).min(self.num_marbles - self.num_marbles_used);
        if let Some((index, marble)) = (0..num_to_add)
            .map(|i| (i, self.num_marbles_used + i))
            .find(|(_, m)| *m % 23 == 0)
        {
            self.interleave(interleave_index, index);

            // player scores
            let remove_marble_index =
                self.get_circular_index(self.current_marble_index as isize - 7);
            let remove_marble = self.circle[remove_marble_index];
            self.circle = [
                &self.circle[..remove_marble_index],
                &self.circle[remove_marble_index + 1..],
            ]
            .concat();
            self.add_score(marble + remove_marble);
            self.num_marbles_used += 1;
            self.current_marble_index = self.get_circular_index(remove_marble_index as isize);
            self.move_player(1);
            // continue
        } else {
            self.interleave(interleave_index, num_to_add);
        }
    }
}

fn get_max_score(num_players: usize, num_marbles: usize) -> usize {
    let mut game = GameCircle::new(num_players, num_marbles);
    loop {
        if game.num_marbles_used >= num_marbles {
            break;
        }
        game.next_circle()
    }
    *game.scores.values().max().unwrap()
}
