use itertools::Itertools;

use utility::AocDay;

pub struct Day11;

impl<'a> AocDay<'a> for Day11 {
    type Input = Grid;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        11
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        let input = input.as_bytes();
        let num_cols = memchr::memchr(b'\n', input).unwrap();
        let items: Vec<_> = input
            .iter()
            .filter_map(|&c| match c {
                b'L' => Some(Some(false)),
                b'.' => Some(None),
                b'\n' => None,
                _ => panic!("unknown character"),
            })
            .collect();
        let num_rows = items.len() / num_cols;
        Ok(Grid {
            items,
            num_rows,
            num_cols,
        })
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut input = input.clone();
        let mut changed;
        let mut neighbors = Vec::with_capacity(input.items.len());
        loop {
            changed = false;
            for i in 0..input.num_rows {
                for j in 0..input.num_cols {
                    neighbors.push(
                        (if i == 0 { i } else { i - 1 }..(i + 2).min(input.num_rows))
                            .cartesian_product(
                                if j == 0 { j } else { j - 1 }..(j + 2).min(input.num_cols),
                            )
                            .map(|(x, y)| input.get(x, y).unwrap_or(false) as usize)
                            .sum(),
                    );
                }
            }
            input
                .items
                .iter_mut()
                .zip(neighbors.drain(..))
                .for_each(|(i, n)| {
                    *i = match (&i, n) {
                        (Some(false), 0) => {
                            changed = true;
                            Some(true)
                        }
                        (Some(true), n) if n >= 5 => {
                            changed = true;
                            Some(false)
                        }
                        _ => *i,
                    }
                });
            if !changed {
                break;
            }
        }
        Ok(input.items.iter().filter(|&&x| x == Some(true)).count())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut input = input.clone();
        let mut changed;
        let mut neighbors = Vec::with_capacity(input.items.len());
        loop {
            changed = false;
            for i in 0..input.num_rows {
                for j in 0..input.num_cols {
                    neighbors.push(
                        vec![
                            ((i + 1)..input.num_rows)
                                .find(|&x| input.get(x, j).is_some())
                                .zip(Some(j)),
                            (0..i)
                                .rev()
                                .find(|&x| input.get(x, j).is_some())
                                .zip(Some(j)),
                            Some(i).zip(
                                ((j + 1)..input.num_cols).find(|&x| input.get(i, x).is_some()),
                            ),
                            Some(i).zip((0..j).rev().find(|&x| input.get(i, x).is_some())),
                            (0..i)
                                .rev()
                                .zip((0..j).rev())
                                .take(i.min(j))
                                .find(|(x, y)| input.get(*x, *y).is_some()),
                            ((i + 1)..input.num_rows)
                                .zip((0..j).rev())
                                .take((input.num_rows - i).min(j))
                                .find(|(x, y)| input.get(*x, *y).is_some()),
                            (0..i)
                                .rev()
                                .zip((j + 1)..input.num_cols)
                                .take(i.max(input.num_cols - j))
                                .find(|(x, y)| input.get(*x, *y).is_some()),
                            ((i + 1)..input.num_rows)
                                .zip((j + 1)..input.num_cols)
                                .take((input.num_rows - i).max(input.num_cols - j))
                                .find(|(x, y)| input.get(*x, *y).is_some()),
                        ]
                        .into_iter()
                        .map(|x| input.get_option(x).unwrap_or(false) as usize)
                        .sum(),
                    );
                }
            }
            input
                .items
                .iter_mut()
                .zip(neighbors.drain(..))
                .for_each(|(i, n)| {
                    *i = match (&i, n) {
                        (Some(false), 0) => {
                            changed = true;
                            Some(true)
                        }
                        (Some(true), n) if n >= 5 => {
                            changed = true;
                            Some(false)
                        }
                        _ => *i,
                    }
                });
            if !changed {
                break;
            }
        }
        Ok(input.items.iter().filter(|&&x| x == Some(true)).count())
    }
}

#[derive(Clone)]
pub struct Grid {
    items: Vec<Option<bool>>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.items[x * self.num_cols + y]
    }
    #[inline(always)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Option<bool> {
        &mut self.items[x * self.num_cols + y]
    }
    #[inline(always)]
    pub fn get_option(&self, x_y: Option<(usize, usize)>) -> Option<bool> {
        if let Some((x, y)) = x_y {
            self.get(x, y)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day11::load(
            r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#,
        )?;
        assert_eq!(Day11::part_1(&input)?, 37);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day11::load(
            r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#,
        )?;
        assert_eq!(Day11::part_2(&input)?, 26);
        Ok(())
    }
}
