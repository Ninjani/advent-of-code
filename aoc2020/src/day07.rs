use std::collections::{HashMap, HashSet, VecDeque};

use utility::AocDay;

pub struct Day07;

impl<'a> AocDay<'a> for Day07 {
    type Input = HashMap<&'a str, Vec<(&'a str, usize)>>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        7
    }
    fn year() -> usize {
        2020
    }

    fn load(input: &'a str) -> color_eyre::Result<Self::Input> {
        let mut graph = HashMap::new();
        for line in input
            .split('\n')
            .filter(|line| !line.ends_with("no other bags."))
        {
            let (start_node, end_nodes) = line.split_once(" contain ").unwrap();
            let (start_node, _) = start_node.split_once(" bag").unwrap();
            for end_node in end_nodes.split(", ") {
                let (weight, end_node) = end_node.split_once(' ').unwrap();
                graph.entry(start_node).or_insert_with(Vec::new).push((
                    end_node.split(" bag").next().unwrap(),
                    weight.parse::<usize>()?,
                ));
            }
        }
        Ok(graph)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut queue = VecDeque::new();
        queue.push_back("shiny gold");
        let mut seen = HashSet::new();
        while let Some(start) = queue.pop_front() {
            if seen.insert(start) {
                if let Some(nodes) = input.get(&start) {
                    queue.extend(nodes.iter().map(|(node, _)| *node));
                }
            }
        }
        Ok(seen.len() - 1)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut queue = VecDeque::new();
        queue.push_back(("shiny gold", 1));
        let mut total = 0;
        while let Some((start, number)) = queue.pop_front() {
            total += number;
            if let Some(edges) = input.get(&start) {
                queue.extend(edges.iter().map(|(end, weight)| (*end, number * weight)));
            }
        }
        Ok(total - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Day07::load(
            r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#,
        )?;
        assert_eq!(Day07::part_1(&input)?, 4);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Day07::load(
            r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#,
        )?;
        assert_eq!(Day07::part_2(&input)?, 32);

        let input = Day07::load(
            r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#,
        )?;
        assert_eq!(Day07::part_2(&input)?, 126);
        Ok(())
    }
}
