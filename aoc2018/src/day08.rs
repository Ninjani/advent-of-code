use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use utility::AocDay;

pub struct Day08;

impl<'a> AocDay<'a> for Day08 {
    type Input = Vec<usize>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        8
    }

    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split_whitespace()
            .map(|x| x.parse::<usize>())
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut tree = Graph::new();
        build_tree(&mut tree, 0, &input);
        Ok(tree
            .node_indices()
            .map(|n| tree[n].iter().sum::<usize>())
            .sum())
    }

    fn part_2(input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        let mut tree = Graph::new();
        let (root, _) = build_tree(&mut tree, 0, &input);
        Ok(get_node_value(&tree, root))
    }
}

fn build_tree(
    tree: &mut Graph<Vec<usize>, ()>,
    index: usize,
    numbers: &[usize],
) -> (NodeIndex<u32>, usize) {
    let (num_children, num_metadata) = (numbers[index], numbers[index + 1]);
    let parent = tree.add_node(Vec::with_capacity(num_metadata));
    let mut index = index + 2;
    for _ in 0..num_children {
        let (child, new_index) = build_tree(tree, index, &numbers);
        index = new_index;
        tree.add_edge(parent, child, ());
    }
    for _ in 0..num_metadata {
        tree[parent].push(numbers[index]);
        index += 1;
    }
    (parent, index)
}

fn get_node_value(tree: &Graph<Vec<usize>, ()>, index: NodeIndex<u32>) -> usize {
    if tree.neighbors(index).count() == 0 {
        tree[index].iter().sum()
    } else {
        let children: Vec<_> = tree.neighbors(index).sorted().collect();
        tree[index]
            .iter()
            .map(|m| {
                if *m == 0 || *m > children.len() {
                    0
                } else {
                    get_node_value(tree, children[*m - 1])
                }
            })
            .sum()
    }
}
