use itertools::Itertools;
use ndarray::{s, Array2, ArrayView};

use utility::AocDay;

pub struct Day03;

impl<'a> AocDay<'a> for Day03 {
    type Input = Vec<Rectangle>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        3
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|claim| parse_claim(claim))
            .collect())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(get_num_claims_per_square(&input)
            .iter()
            .filter(|num_ids| **num_ids >= 2)
            .count())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let claims = get_num_claims_per_square(&input);
        for rectangle in input {
            if claims.slice(s![
                rectangle.left..rectangle.left + rectangle.width,
                rectangle.top..rectangle.top + rectangle.height
            ]) == Array2::ones((rectangle.width, rectangle.height))
            {
                return Ok(rectangle.id);
            }
        }
        Ok(0)
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Rectangle {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse_claim(line: &str) -> Rectangle {
    let (id, loc_dims) = line.split('@').collect_tuple().unwrap();
    let id = id.split('#').collect::<Vec<_>>()[1]
        .trim()
        .parse::<usize>()
        .unwrap();
    let (loc, dims) = loc_dims.split(": ").collect_tuple().unwrap();
    let (left, top) = loc
        .split(',')
        .map(|l| l.trim().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let (width, height) = dims
        .split('x')
        .map(|d| d.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Rectangle {
        id,
        left,
        top,
        width,
        height,
    }
}

fn get_num_claims_per_square(rectangles: &[Rectangle]) -> Array2<usize> {
    let (mut max_x, mut max_y) = (0, 0);
    let (mut rect_x, mut rect_y);
    for rectangle in rectangles {
        rect_x = rectangle.left + rectangle.width;
        if rect_x > max_x {
            max_x = rect_x;
        }
        rect_y = rectangle.top + rectangle.height;
        if rect_y > max_y {
            max_y = rect_y;
        }
    }
    let mut claims = Array2::zeros((max_x, max_y));
    for rectangle in rectangles {
        let mut slice = claims.slice_mut(s![
            rectangle.left..rectangle.left + rectangle.width,
            rectangle.top..rectangle.top + rectangle.height
        ]);
        slice += &ArrayView::from(&[1]);
    }
    claims
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day03::load(
            r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#,
        )?;
        assert_eq!(Day03::part_1(&input)?, 4);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day03::load(
            r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#,
        )?;
        assert_eq!(Day03::part_2(&input)?, 3);
        Ok(())
    }
}
