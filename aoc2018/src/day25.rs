use petgraph::algo::condensation;
use petgraph::Graph;

use utility::AocDay;

pub struct Day25;

impl<'a> AocDay<'a> for Day25 {
    type Input = Vec<Point>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        25
    }
    fn year() -> usize {
        2018
    }

    fn load(input: &str) -> color_eyre::Result<Self::Input> {
        Ok(input
            .split('\n')
            .map(|line| {
                let mut point = [0; 4];
                line.split(",")
                    .enumerate()
                    .for_each(|(i, x)| point[i] = x.parse().unwrap());
                point
            })
            .collect())
    }

    fn part_1(input: &Self::Input) -> color_eyre::Result<Self::Result1> {
        let mut constellations = Vec::new();
        for point in input {
            if constellations.is_empty() {
                constellations.push(vec![point]);
            } else {
                let mut found = false;
                for i in 0..constellations.len() {
                    if in_constellation(&point, &constellations[i]) {
                        constellations[i].push(point);
                        found = true;
                        break;
                    }
                }
                if !found {
                    constellations.push(vec![point]);
                }
            }
        }
        let mut graph = Graph::new_undirected();
        for i in 0..constellations.len() {
            graph.add_node(i);
        }
        for node_1 in graph.node_indices() {
            for node_2 in graph.node_indices() {
                if same_constellation(
                    &constellations[graph[node_1]],
                    &constellations[graph[node_2]],
                ) {
                    graph.add_edge(node_1, node_2, ());
                }
            }
        }
        Ok(condensation(graph, true).node_count())
    }

    fn part_2(_input: &Self::Input) -> color_eyre::Result<Self::Result2> {
        unimplemented!()
    }
}

type Point = [isize; 4];

pub fn distance(point_1: &Point, point_2: &Point) -> usize {
    (0..4)
        .map(|i| (point_1[i] - point_2[i]).abs() as usize)
        .sum()
}

pub fn in_constellation(point: &Point, constellation: &[&Point]) -> bool {
    for p in constellation {
        if distance(point, p) <= 3 {
            return true;
        }
    }
    false
}

pub fn same_constellation(constellation_1: &Vec<&Point>, constellation_2: &[&Point]) -> bool {
    for point in constellation_1 {
        if in_constellation(point, constellation_2) {
            return true;
        }
    }
    false
}
