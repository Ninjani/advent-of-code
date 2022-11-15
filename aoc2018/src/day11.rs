use ndarray::{s, Array2};

use utility::AocDay;

pub struct Day11;

impl<'a> AocDay<'a> for Day11 {
    type Input = Array2<i32>;
    type Result1 = String;
    type Result2 = String;

    fn day() -> usize {
        11
    }
    fn year() -> usize {
        2018
    }
    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        let serial_number = input.parse::<usize>()?;
        let mut grid = Array2::zeros((300, 300));
        for i in 0..300 {
            for j in 0..300 {
                grid[(i, j)] = get_battery(i + 1, j + 1, serial_number);
            }
        }
        Ok(grid)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut grid_battery;
        let mut max_battery = 0;
        let mut best_xy = (0, 0);
        for i in 0..300 - 3 {
            for j in 0..300 - 3 {
                grid_battery = input.slice(s![i..i + 3, j..j + 3]).sum();
                if grid_battery > max_battery {
                    max_battery = grid_battery;
                    best_xy = (i, j);
                }
            }
        }
        Ok(format!("{},{}", best_xy.0 + 1, best_xy.1 + 1))
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut grid_battery;
        let mut max_battery = 0;
        let mut best_xys = (0, 0, 0);
        for s in 0..300 {
            for i in 0..300 - s {
                for j in 0..300 - s {
                    grid_battery = input.slice(s![i..i + s, j..j + s]).sum();
                    if grid_battery > max_battery {
                        max_battery = grid_battery;
                        best_xys = (i, j, s);
                    }
                }
            }
        }
        Ok(format!(
            "{},{},{}",
            best_xys.0 + 1,
            best_xys.1 + 1,
            best_xys.2
        ))
    }
}

fn get_battery(x: usize, y: usize, serial_number: usize) -> i32 {
    let rack_id = x + 10;
    (((((rack_id * y) + serial_number) * rack_id) / 100) % 10) as i32 - 5
}
