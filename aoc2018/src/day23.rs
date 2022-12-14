use itertools::Itertools;
use ndarray::Array3;
use regex::Regex;

use utility::AocDay;

pub struct Day23;

impl<'a> AocDay<'a> for Day23 {
    type Input = Vec<NanoBot>;

    type Result1 = usize;

    type Result2 = usize;

    fn day() -> usize {
        23
    }
    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        let nanobot_re =
            Regex::new(r"pos=<([-]??[0-9]+),([-]??[0-9]+),([-]??[0-9]+)>, r=([-]??[0-9]+)")?;
        let mut nanobots = Vec::new();
        let mut nanobot;
        for line in input.split('\n') {
            nanobot = nanobot_re.captures(line).unwrap();
            nanobots.push(NanoBot {
                coordinate: Coordinate {
                    x: nanobot[1].parse()?,
                    y: nanobot[2].parse()?,
                    z: nanobot[3].parse()?,
                },
                radius: nanobot[4].parse()?,
            });
        }
        Ok(nanobots)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let max_nanobot = input.iter().max_by(|a, b| a.radius.cmp(&b.radius)).unwrap();
        Ok(input
            .iter()
            .filter(|nanobot| {
                max_nanobot.coordinate.get_distance(&nanobot.coordinate) <= max_nanobot.radius
            })
            .count())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let max_x = input
            .iter()
            .max_by(|a, b| a.coordinate.x.cmp(&b.coordinate.x))
            .unwrap()
            .coordinate
            .x;
        let max_y = input
            .iter()
            .max_by(|a, b| a.coordinate.y.cmp(&b.coordinate.y))
            .unwrap()
            .coordinate
            .y;
        let max_z = input
            .iter()
            .max_by(|a, b| a.coordinate.z.cmp(&b.coordinate.z))
            .unwrap()
            .coordinate
            .z;
        let min_x = input
            .iter()
            .min_by(|a, b| a.coordinate.x.cmp(&b.coordinate.x))
            .unwrap()
            .coordinate
            .x;
        let min_y = input
            .iter()
            .min_by(|a, b| a.coordinate.y.cmp(&b.coordinate.y))
            .unwrap()
            .coordinate
            .y;
        let min_z = input
            .iter()
            .min_by(|a, b| a.coordinate.z.cmp(&b.coordinate.z))
            .unwrap()
            .coordinate
            .z;
        let max_radius = input
            .iter()
            .max_by(|a, b| a.radius.cmp(&b.radius))
            .unwrap()
            .radius;
        println!(
            "{} {} {}\n{} {} {}",
            max_x, max_y, max_z, min_x, min_y, min_z
        );
        let mut array = Array3::<usize>::zeros((
            (max_x - min_x) as usize + 2 * max_radius,
            (max_y - min_y) as usize + 2 * max_radius,
            (max_z - min_z) as usize + 2 * max_radius,
        ));
        let (mut x, mut y, mut z);
        for nanobot in input {
            x = (nanobot.coordinate.x - min_x) as usize;
            y = (nanobot.coordinate.y - min_y) as usize;
            z = (nanobot.coordinate.z - min_z) as usize;
            for i in (x - nanobot.radius)..=(x + nanobot.radius) {
                for j in (y - nanobot.radius)..=(y + nanobot.radius) {
                    for k in (z - nanobot.radius)..=(z + nanobot.radius) {
                        if nanobot.coordinate.get_distance(&Coordinate {
                            x: i as isize + min_x,
                            y: j as isize + min_y,
                            z: k as isize + min_z,
                        }) <= nanobot.radius
                        {
                            array[(i, j, k)] += 1;
                        }
                    }
                }
            }
        }
        let indices: Vec<_> = array
            .indexed_iter()
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect();
        let max_count = indices[0].1;
        let mut top_indices = Vec::new();
        for i in indices {
            if i.1 == max_count {
                top_indices.push(i);
            } else {
                break;
            }
        }
        Ok(top_indices
            .into_iter()
            .map(|(coordinate, _)| Coordinate {
                x: coordinate.0 as isize + min_x,
                y: coordinate.1 as isize + min_y,
                z: coordinate.2 as isize + min_z,
            })
            .map(|coordinate| coordinate.get_distance(&Coordinate { x: 0, y: 0, z: 0 }))
            .min()
            .unwrap())
    }
}

pub struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    fn get_distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

pub struct NanoBot {
    coordinate: Coordinate,
    radius: usize,
}
