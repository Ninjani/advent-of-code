use std::collections::HashMap;
use std::collections::HashSet;

use color_eyre::Result;
use itertools::Itertools;
use petgraph::prelude::StableGraph;
use petgraph::Direction;

use utility::AocDay;

pub struct Day07;

impl<'a> AocDay<'a> for Day07 {
    type Input = StableGraph<char, ()>;
    type Result1 = String;
    type Result2 = usize;

    fn day() -> usize {
        7
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> Result<Self::Input> {
        let (mut graph, mut node_to_index) = (StableGraph::new(), HashMap::new());
        let (mut index_1, mut index_2);
        for line in input.trim().split('\n') {
            let parts: Vec<_> = line.split_whitespace().collect();
            let (c1, c2) = (
                parts[1].chars().next().unwrap(),
                parts[7].chars().next().unwrap(),
            );
            index_1 = *node_to_index
                .entry(c1)
                .or_insert_with(|| graph.add_node(c1));
            index_2 = *node_to_index
                .entry(c2)
                .or_insert_with(|| graph.add_node(c2));
            graph.add_edge(index_1, index_2, ());
        }
        Ok(graph)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut input = input.clone();
        let mut order = String::new();
        while input.node_count() > 0 {
            let node = input
                .node_indices()
                .filter(|n| input.neighbors_directed(*n, Direction::Incoming).count() == 0)
                .min_by(|a, b| input[*a].cmp(&input[*b]))
                .unwrap();
            order.push(input[node]);
            input.remove_node(node);
        }
        Ok(order)
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        fn get_time(c: char) -> usize {
            (c as u8 - b'A') as usize + 1 + 60
        }
        let mut input = input.clone();
        let (mut time, mut elf_nodes, mut elf_times) = (0, [None; 5], [0; 5]);
        let mut not_assigned: HashSet<_> = input.node_indices().collect();
        let (mut time_flag, mut zero_indices, mut min_time);
        while input.node_count() > 0 {
            time_flag = false;
            zero_indices = Vec::new();
            min_time = (*elf_times.iter().min().unwrap()).max(1);
            for i in 0..elf_times.len() {
                if elf_times[i] > 0 {
                    elf_times[i] -= min_time;
                    time_flag = true;
                }
                if elf_times[i] == 0 {
                    zero_indices.push(i);
                    if let Some(node) = elf_nodes[i] {
                        input.remove_node(node);
                        elf_nodes[i] = None
                    }
                }
            }
            if time_flag {
                time += min_time;
            }
            if !zero_indices.is_empty() {
                let nodes: Vec<_> = not_assigned
                    .iter()
                    .filter(|n| input.neighbors_directed(**n, Direction::Incoming).count() == 0)
                    .cloned()
                    .sorted_by(|a, b| input[*a].cmp(&input[*b]))
                    .collect();
                for i in 0..(zero_indices.len().min(nodes.len())) {
                    elf_times[zero_indices[i]] = get_time(input[nodes[i]]);
                    elf_nodes[zero_indices[i]] = Some(nodes[i]);
                    not_assigned.remove(&nodes[i]);
                }
            }
        }
        Ok(time)
    }
}
