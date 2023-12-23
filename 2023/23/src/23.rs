use std::collections::VecDeque;

use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    Junctions::parse(input)
        .get_graph(|dir, tile| {
            if let Tile::Slope(d) = tile {
                d != dir
            } else {
                false
            }
        })
        .0
        .longest_path(0, 1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    Junctions::parse(input)
        .get_graph(|_, _| false)
        .0
        .longest_path(0, 1)
}

// TODO: attempt to optimise with memoisation
// the same (node, set of visited nodes) always produces the same answer
// pack visited into a u64 for use as a key in a hashmap
// does the order of traversal affect how effective this memoisation will be?
struct Graph {
    adj: Vec<Vec<(usize, u32)>>,
}

impl Graph {
    fn new(adj: Vec<Vec<(usize, u32)>>) -> Self {
        Self { adj }
    }

    fn longest_path(&self, start: usize, end: usize) -> u32 {
        let mut visited = vec![false; self.adj.len()];
        visited[start] = true;
        let mut max = 0;
        self.search(start, end, 0, &mut visited, &mut max);
        max
    }

    fn search(&self, i: usize, end: usize, steps: u32, visited: &mut [bool], max: &mut u32) {
        if i == end {
            *max = (*max).max(steps);
            return;
        }
        for &(j, s) in &self.adj[i] {
            if !visited[j] {
                visited[j] = true;
                self.search(j, end, steps + s, visited, max);
                visited[j] = false;
            }
        }
    }
}

struct Junctions {
    map: Grid<Tile>,
    adj: Vec<Vec<(usize, u32)>>,
    queue: VecDeque<(usize, Vector, Vector)>,
    indices: Grid<Option<usize>>,
}

impl Junctions {
    fn parse(input: aoc::Input) -> Self {
        let map = Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Tile::parse)));
        let start = v(1, 0);
        let end = map.dim() - v(2, 1);
        let queue = VecDeque::from([(0, start, SOUTH)]);
        let adj = vec![Vec::new(); 2];
        let mut indices = map.map(|_| None);
        indices[start] = Some(0);
        indices[end] = Some(1);
        Self {
            map,
            adj,
            queue,
            indices,
        }
    }

    fn get_graph(mut self, blocked: impl Fn(Vector, Tile) -> bool) -> (Graph, u32) {
        while let Some((i, mut pos, mut dir)) = self.queue.pop_front() {
            let mut steps = 0;
            loop {
                steps += 1;
                pos += dir;
                if blocked(dir, self.map[pos]) {
                    break;
                }
                if let Some(j) = self.indices[pos] {
                    self.adj[i].push((j, steps));
                    break;
                }
                if self.is_junction(pos) {
                    let j = self.adj.len();
                    self.adj.push(Vec::new());
                    self.indices[pos] = Some(j);
                    self.adj[i].push((j, steps));
                    for offset in ORTHOGONAL {
                        if self.map[pos + offset] != Tile::Forest {
                            self.queue.push_back((j, pos, offset));
                        }
                    }
                    break;
                }
                dir = self.continue_path(pos, dir);
            }
        }

        (Graph::new(self.adj), 0)
    }

    fn is_junction(&self, pos: Vector) -> bool {
        ORTHOGONAL
            .into_iter()
            .filter(|&o| self.map[pos + o] != Tile::Forest)
            .count()
            > 2
    }

    fn continue_path(&self, pos: Vector, dir: Vector) -> Vector {
        for offset in ORTHOGONAL {
            if offset != -dir && self.map[pos + offset] != Tile::Forest {
                return offset;
            }
        }
        unreachable!() // dead end
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Vector),
}

impl Tile {
    fn parse(byte: u8) -> Self {
        match byte {
            b'.' => Self::Path,
            b'#' => Self::Forest,
            b'>' => Self::Slope(EAST),
            b'v' => Self::Slope(SOUTH),
            _ => unreachable!(),
        }
    }
}
