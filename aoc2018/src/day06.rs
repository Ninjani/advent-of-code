use color_eyre::Result;
use itertools::{iproduct, Itertools};

use utility::AocDay;

pub struct Day06;

impl<'a> AocDay<'a> for Day06 {
    type Input = Grid;
    type Result1 = i32;
    type Result2 = usize;

    fn day() -> usize {
        6
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> Result<Self::Input> {
        let (mut m_x, mut m_y) = (0, 0);
        let coords = input
            .trim()
            .split('\n')
            .map(|line| {
                let (c_x, c_y) = line
                    .trim()
                    .split(", ")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                if c_x > m_x {
                    m_x = c_x;
                }
                if c_y > m_y {
                    m_y = c_y;
                }
                (c_x, c_y)
            })
            .collect();
        Ok((coords, m_x, m_y))
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let (coords, m_x, m_y) = input;
        let (m_x, m_y) = (*m_x, *m_y);
        let mut areas = vec![0; coords.len()];
        for (i, j) in iproduct!(0..m_x, 0..m_y) {
            let distances: Vec<_> = coords
                .iter()
                .enumerate()
                .map(|(index, coord)| (manhattan_distance(*coord, (i, j)), index))
                .sorted()
                .collect();
            if distances[0].0 != distances[1].0 {
                areas[distances[0].1] += 1;
                if i == 0 || i == m_x - 1 || j == 0 || j == m_y - 1 {
                    areas[distances[0].1] = i32::MIN;
                }
            }
        }
        Ok(areas.into_iter().max().unwrap())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let (coords, m_x, m_y) = input;
        let (m_x, m_y) = (*m_x, *m_y);
        Ok(iproduct!(0..=m_x, 0..=m_y)
            .map(|c| {
                if coords
                    .iter()
                    .map(|c1| manhattan_distance(*c1, c))
                    .sum::<usize>()
                    < 10000
                {
                    1
                } else {
                    0
                }
            })
            .sum())
    }
}

type Grid = (Vec<(usize, usize)>, usize, usize);

fn manhattan_distance(coord_1: (usize, usize), coord_2: (usize, usize)) -> usize {
    ((coord_1.0 as isize - coord_2.0 as isize).abs()
        + (coord_1.1 as isize - coord_2.1 as isize).abs()) as usize
}
