use utility::AocDay;

pub struct Day02;

impl<'a> AocDay<'a> for Day02 {
    type Input = Vec<usize>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        2
    }
    fn year() -> usize {
        2019
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split(',')
            .map(|i| i.parse::<usize>())
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut intcode = input.to_vec();
        intcode[1] = 12;
        intcode[2] = 2;
        process_intcode(&mut intcode);
        Ok(intcode[0])
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        match find_noun_verb(input, 19_690_720) {
            Some((noun, verb)) => Ok(100 * noun + verb),
            None => panic!("(noun verb) pair not found"),
        }
    }
}

fn process_intcode(intcode: &mut [usize]) {
    for i in (0..intcode.len()).step_by(4) {
        match intcode[i] {
            99 => break,
            1 => intcode[intcode[i + 3]] = intcode[intcode[i + 1]] + intcode[intcode[i + 2]],
            2 => intcode[intcode[i + 3]] = intcode[intcode[i + 1]] * intcode[intcode[i + 2]],
            n => panic!("Unknown opcode {}", n),
        }
    }
}

fn find_noun_verb(intcode: &[usize], output_equals: usize) -> Option<(usize, usize)> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode_noun_verb = intcode.to_vec();
            intcode_noun_verb[1] = noun;
            intcode_noun_verb[2] = verb;
            process_intcode(&mut intcode_noun_verb);
            if intcode_noun_verb[0] == output_equals {
                return Some((noun, verb));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_1() {
        let mut intcode = vec![1, 0, 0, 0, 99];
        process_intcode(&mut intcode);
        assert_eq!(vec![2, 0, 0, 0, 99], intcode);
        let mut intcode = vec![2, 3, 0, 3, 99];
        process_intcode(&mut intcode);
        assert_eq!(vec![2, 3, 0, 6, 99], intcode);
        let mut intcode = vec![2, 4, 4, 5, 99, 0];
        process_intcode(&mut intcode);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], intcode);
        let mut intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        process_intcode(&mut intcode);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], intcode);
    }

    #[test]
    fn test_2() -> color_eyre::Result<()> {
        let intcode: Vec<_> = fs::read_to_string("../data/2019/day02.txt")?
            .split(',')
            .map(|i| i.parse::<usize>())
            .collect::<Result<_, _>>()?;
        let (noun, verb) = find_noun_verb(&intcode, 4945026).unwrap();
        assert_eq!(1202, 100 * noun + verb);
        Ok(())
    }
}
