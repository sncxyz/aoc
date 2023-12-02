aoc::parts!(1, 2);

use std::collections::BTreeMap;
use Node::*;

fn part_1(input: &[&str]) -> impl ToString {
    Map::new(input).count(false)
}

fn part_2(input: &[&str]) -> impl ToString {
    Map::new(input).count(true)
}

pub struct Map {
    nodes: BTreeMap<Node, Vec<Node>>,
    small_caves: usize,
}

impl Map {
    fn new(input: &[&str]) -> Map {
        let (mut s, mut l) = (0, 0);
        let mut nodes: BTreeMap<_, Vec<_>> = BTreeMap::new();
        let mut names = BTreeMap::new();
        for line in input {
            let linked: Vec<Node> = line
                .split('-')
                .map(|part| Node::new(part, &mut names, &mut s, &mut l))
                .collect();
            nodes.entry(linked[0]).or_default().push(linked[1]);
            nodes.entry(linked[1]).or_default().push(linked[0]);
        }
        Map {
            nodes,
            small_caves: s,
        }
    }

    fn count(&self, twice: bool) -> u32 {
        self.traverse(&Start, vec![false; self.small_caves], twice)
    }

    fn traverse(&self, node: &Node, mut visited: Vec<bool>, mut twice: bool) -> u32 {
        if let &Small(x) = node {
            if visited[x] {
                twice = false;
            } else {
                visited[x] = true;
            }
        }
        self.nodes[node]
            .iter()
            .map(|neighbour| match neighbour {
                Start => 0,
                End => 1,
                Large(_) => self.traverse(neighbour, visited.clone(), twice),
                &Small(x) => {
                    if !visited[x] || twice {
                        self.traverse(neighbour, visited.clone(), twice)
                    } else {
                        0
                    }
                }
            })
            .sum()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Node {
    Start,
    End,
    Large(usize),
    Small(usize),
}

impl Node {
    fn new(
        name: &str,
        names: &mut BTreeMap<String, usize>,
        small_count: &mut usize,
        large_count: &mut usize,
    ) -> Node {
        match name {
            "start" => Start,
            "end" => End,
            _ if name.chars().any(char::is_uppercase) => {
                Large(*names.entry(name.to_string()).or_insert_with(|| {
                    *large_count += 1;
                    *large_count - 1
                }))
            }
            _ => Small(*names.entry(name.to_string()).or_insert_with(|| {
                *small_count += 1;
                *small_count - 1
            })),
        }
    }
}
