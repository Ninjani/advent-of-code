use color_eyre::eyre::eyre;
use ndarray::{s, Array, Array2, Array3};

use utility::AocDay;

pub struct Day08;

impl<'a> AocDay<'a> for Day08 {
    type Input = Array3<u32>;
    type Result1 = u32;
    type Result2 = String;

    fn day() -> usize {
        8
    }
    fn year() -> usize {
        2019
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(get_image(input)?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let min_zero_layer = (0..input.shape()[0])
            .map(|i| {
                (
                    input
                        .slice(s![i, .., ..])
                        .iter()
                        .filter(|x| **x == 0)
                        .count(),
                    i,
                )
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .1;
        let (mut ones, mut twos) = (0, 0);
        for x in input.slice(s![min_zero_layer, .., ..]).iter() {
            if *x == 1 {
                ones += 1;
            } else if *x == 2 {
                twos += 1;
            }
        }
        Ok(ones * twos)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut visible = Array2::zeros((HEIGHT, WIDTH));
        visible.fill(2);
        for layer in 0..input.shape()[0] {
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    if visible[(i, j)] == 2 && input[(layer, i, j)] != 2 {
                        visible[(i, j)] = input[(layer, i, j)]
                    }
                }
            }
        }
        Ok(plot(&visible))
    }
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn get_image(input: &str) -> color_eyre::Result<Array3<u32>> {
    let num_layers = input.len() / (WIDTH * HEIGHT);
    Ok(Array::from(
        input
            .chars()
            .map(|c| c.to_digit(10).ok_or_else(|| eyre!("char is not digit")))
            .collect::<Result<Vec<_>, _>>()?,
    )
    .into_shape((num_layers, HEIGHT, WIDTH))?)
}

pub fn plot(array: &Array2<u32>) -> String {
    (0..array.shape()[0])
        .flat_map(|i| {
            (0..array.shape()[1])
                .map(move |j| if array[(i, j)] == 0 { ' ' } else { '█' })
                .chain(vec!['\n'].into_iter())
        })
        .collect()
}
