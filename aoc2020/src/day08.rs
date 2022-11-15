use color_eyre::eyre::eyre;

use utility::AocDay;

pub struct Day08;

impl<'a> AocDay<'a> for Day08 {
    type Input = Vec<Instruction>;
    type Result1 = i32;
    type Result2 = i32;

    fn day() -> usize {
        8
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        let mut instructions = Vec::new();
        for instruction in input.split('\n') {
            let (op, num) = instruction.split_once(' ').ok_or(eyre!("Wrong format"))?;
            let num: i32 = num.parse()?;
            let instruction = match op {
                "acc" => Instruction::Acc(num),
                "nop" => Instruction::Nop(num),
                "jmp" => Instruction::Jmp(num),
                _ => panic!("unknown instruction"),
            };
            instructions.push(instruction);
        }
        Ok(instructions)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        Ok(run_bootloader(input).1)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut input = input.clone();
        for op_index in 0..input.len() {
            match input[op_index] {
                Instruction::Acc(_) => continue,
                Instruction::Jmp(num) => input[op_index] = Instruction::Nop(num),
                Instruction::Nop(num) => input[op_index] = Instruction::Jmp(num),
            }
            let (fixed, acc) = run_bootloader(&input);
            if fixed {
                return Ok(acc);
            }
            match input[op_index] {
                Instruction::Jmp(num) => input[op_index] = Instruction::Nop(num),
                Instruction::Nop(num) => input[op_index] = Instruction::Jmp(num),
                _ => (),
            }
        }
        unreachable!()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

fn run_bootloader(input: &[Instruction]) -> (bool, i32) {
    let num_instructions = input.len();
    let mut accumulator = 0;
    let mut seen = vec![false; num_instructions];
    let mut index = 0;
    while index != num_instructions {
        if seen[index] {
            return (false, accumulator);
        }
        seen[index] = true;
        match input[index] {
            Instruction::Acc(num) => {
                accumulator += num;
                index += 1;
            }
            Instruction::Nop(_) => index += 1,
            Instruction::Jmp(num) => index = (index as i32 + num) as usize,
        }
    }
    (true, accumulator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        assert_eq!(Day08::part_1(&Day08::load(input)?)?, 5);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        assert_eq!(Day08::part_2(&Day08::load(input)?)?, 8);
        Ok(())
    }
}
