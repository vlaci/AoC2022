use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use pathfinding::prelude::dijkstra;

use color_eyre::{eyre::ContextCompat, Report, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let graph: Graph = input.parse()?;

    let distance = graph.distance(&Node::Start)?;
    println!("The shortest path from start is {distance}");

    let distance = graph.shortest()?;
    println!("The shortest path from any point is {distance}");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Regular((usize, usize), u8),
}

impl Node {
    fn value(&self) -> u8 {
        match self {
            Node::Start => b'a',
            Node::End => b'z',
            Node::Regular(_, e) => *e,
        }
    }
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    fn new() -> Self {
        Default::default()
    }

    fn add_vertex(&mut self, vertex: [Node; 2]) {
        let [start, end] = vertex;
        self.nodes
            .entry(start)
            .or_insert_with(HashSet::new)
            .insert(end);
    }

    fn distance(&self, start: &Node) -> Result<usize> {
        dijkstra(
            start,
            |v| {
                self.nodes[v]
                    .iter()
                    .filter_map(|&n| {
                        if (n.value() as i16 - v.value() as i16) <= 1 {
                            Some((n, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |v| *v == Node::End,
        )
        .wrap_err("Not possible to find path")
        .map(|r| r.1)
    }

    fn shortest(&self) -> Result<usize> {
        let start_points = self.nodes.keys().filter(|v| match v {
            Node::Start => true,
            Node::End => false,
            Node::Regular(_, e) => *e == b'a',
        });

        start_points
            .filter_map(|s| self.distance(s).ok())
            .min()
            .wrap_err("No path found")
    }
}

impl FromStr for Graph {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: HashMap<(usize, usize), u8> = HashMap::new();
        let _m = s.lines().next().unwrap().len();
        let _n = s.lines().count();
        for (y, line) in s.lines().enumerate() {
            for (x, &c) in line.as_bytes().iter().enumerate() {
                nodes.insert((x, y), c);
            }
        }

        let neighbors = |pos: (usize, usize)| {
            [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)]
                .into_iter()
                .map(move |(dx, dy)| ((pos.0 as i32 + dx), (pos.1 as i32 + dy)))
                .map(move |(x, y)| (x as usize, y as usize))
                .filter(|pos| nodes.contains_key(pos))
        };

        let mut graph = Graph::new();

        for (&pos, &start_height) in &nodes {
            for neigh in neighbors(pos) {
                let &end_height = nodes
                    .get(&neigh)
                    .wrap_err_with(|| format!("Coordinate not present {neigh:?}"))?;
                let start_node = match start_height {
                    b'S' => Node::Start,
                    b'E' => Node::End,
                    _ => Node::Regular(pos, start_height),
                };
                let end_node = match end_height {
                    b'S' => Node::Start,
                    b'E' => Node::End,
                    _ => Node::Regular(neigh, end_height),
                };

                graph.add_vertex([start_node, end_node]);
            }
        }
        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use rstest::*;

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        indoc! {"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "}
    }

    #[rstest]
    fn test_parsing(input: &str) {
        let graph: Graph = input.parse().unwrap();
        assert_eq!(graph.distance(&Node::Start).unwrap(), 31);
        assert_eq!(graph.shortest().unwrap(), 29);
    }
}
