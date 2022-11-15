use std::mem::swap;

use utility::AocDay;

pub struct Day12;

impl<'a> AocDay<'a> for Day12 {
    type Input = Vec<(char, isize)>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        12
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .map(|line| {
                let mut chars = line.chars();
                let first = chars.next().unwrap();
                match chars.as_str().parse() {
                    Ok(num) => Ok((first, num)),
                    Err(e) => Err(e),
                }
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut current_direction = 0;
        let (mut movement_north, mut movement_east) = (0, 0);
        for (direction, amount) in input {
            let amount = *amount;
            match *direction {
                'N' => movement_north += amount,
                'S' => movement_north -= amount,
                'E' => movement_east += amount,
                'W' => movement_east -= amount,
                'F' => match current_direction {
                    0 => movement_east += amount,
                    90 => movement_north += amount,
                    180 => movement_east -= amount,
                    270 => movement_north -= amount,
                    _ => panic!("unknown direction"),
                },
                'L' => {
                    current_direction += amount;
                    current_direction %= 360;
                }
                'R' => {
                    current_direction += 360 - amount;
                    current_direction %= 360;
                }
                _ => panic!("unknown direction"),
            }
        }
        Ok((movement_north.abs() as usize) + (movement_east.abs() as usize))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let (mut waypoint_north, mut waypoint_east) = (1, 10);
        let (mut ship_north, mut ship_east) = (0, 0);
        for (direction, amount) in input {
            let amount = *amount;
            match *direction {
                'N' => waypoint_north += amount,
                'S' => waypoint_north -= amount,
                'E' => waypoint_east += amount,
                'W' => waypoint_east -= amount,
                'F' => {
                    ship_north += waypoint_north * amount;
                    ship_east += waypoint_east * amount;
                }
                'R' => {
                    for _ in 0..amount / 90 {
                        swap(&mut waypoint_north, &mut waypoint_east);
                        waypoint_north *= -1;
                    }
                }
                'L' => {
                    for _ in 0..amount / 90 {
                        swap(&mut waypoint_north, &mut waypoint_east);
                        waypoint_east *= -1;
                    }
                }
                _ => panic!("unknown direction"),
            }
        }
        Ok((ship_north.abs() as usize) + (ship_east.abs() as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day12::load(
            r#"F10
N3
F7
R90
F11"#,
        )?;
        assert_eq!(Day12::part_1(&input)?, 25);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day12::load(
            r#"F10
N3
F7
R90
F11"#,
        )?;
        assert_eq!(Day12::part_2(&input)?, 286);
        Ok(())
    }
}
