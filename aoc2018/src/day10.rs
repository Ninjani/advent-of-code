use std::collections::HashSet;

use itertools::Itertools;
use ndarray::Array2;

use utility::AocDay;

pub struct Day10;

impl<'a> AocDay<'a> for Day10 {
    type Input = Vec<PosVel>;

    type Result1 = usize;

    type Result2 = usize;

    fn day() -> usize {
        10
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: Vec<_> = line.split('=').collect();
                let ((pos_x, pos_y), (vel_x, vel_y)) = [1, 2]
                    .iter()
                    .map(|i| {
                        let part = parts[*i].split('>').next().unwrap();
                        part[1..]
                            .split(", ")
                            .map(|x| x.trim().parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap();
                ((pos_x, pos_y), (vel_x, vel_y))
            })
            .collect())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut xs: Vec<_> = input.iter().map(|(pos, vel)| (pos.0, vel.0)).collect();
        let mut ys: Vec<_> = input.iter().map(|(pos, vel)| (pos.1, vel.1)).collect();
        let (mut best_second, mut min_score, mut second) = (0, ::std::usize::MAX, 0);
        loop {
            let score = score_text(&xs, &ys);
            if score < min_score {
                min_score = score;
                best_second = second;
            }
            xs = xs.into_iter().map(|(x, v)| (x + v, v)).collect();
            ys = ys.into_iter().map(|(y, v)| (y + v, v)).collect();
            second += 1;
            if second > 15000 {
                break;
            }
        }
        let xs: Vec<_> = input
            .iter()
            .map(|(pos, vel)| (pos.0 + (best_second as isize) * vel.0, vel.0))
            .collect();
        let ys: Vec<_> = input
            .iter()
            .map(|(pos, vel)| (pos.1 + (best_second as isize) * vel.1, vel.1))
            .collect();
        display_text(&xs, &ys);
        Ok(best_second)
    }

    fn part_2(_input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        unimplemented!()
    }
}

type PosVel = ((isize, isize), (isize, isize));

fn display_text(xs: &[(isize, isize)], ys: &[(isize, isize)]) {
    let (min_x, max_x) = (xs.iter().min().unwrap().0, xs.iter().max().unwrap().0);
    let (min_y, max_y) = (ys.iter().min().unwrap().0, ys.iter().max().unwrap().0);
    let mut array = Array2::zeros(((max_x - min_x) as usize + 1, (max_y - min_y) as usize + 1));
    for i in 0..xs.len() {
        array[((xs[i].0 - min_x) as usize, (ys[i].0 - min_y) as usize)] = 1;
    }
    for j in 0..array.shape()[1] {
        for i in 0..array.shape()[0] {
            print!("{}", if array[(i, j)] == 1 { "#" } else { "." });
        }
        println!();
    }
}

fn score_text(xs: &[(isize, isize)], ys: &[(isize, isize)]) -> usize {
    xs.iter().map(|(x, _)| x).collect::<HashSet<_>>().len()
        + ys.iter().map(|(y, _)| y).collect::<HashSet<_>>().len()
}
