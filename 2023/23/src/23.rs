use std::collections::VecDeque;

use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let map = Map::parse(input);
    let junctions = Junctions::new(map);
    let (graph, extra) = junctions.get_graph(|dir, tile| {
        if let Tile::Slope(d) = tile {
            d != dir
        } else {
            false
        }
    });
    graph.longest_path(0, 1) + extra
}

fn part_2(input: aoc::Input) -> impl ToString {
    let map = Map::parse(input);
    let junctions = Junctions::new(map);
    let (graph, extra) = junctions.get_graph(|_, _| false);
    graph.longest_path(0, 1) + extra
}

struct Map {
    tiles: Grid<Tile>,
}

impl Map {
    fn parse(input: aoc::Input) -> Self {
        let tiles = Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Tile::parse)));
        Self { tiles }
    }

    fn is_junction(&self, pos: Vector) -> bool {
        ORTHOGONAL
            .into_iter()
            .filter(|&o| self.tiles[pos + o] != Tile::Forest)
            .count()
            > 2
    }

    fn continue_path(&self, pos: Vector, dir: Vector) -> Vector {
        for offset in ORTHOGONAL {
            if offset != -dir && self.tiles[pos + offset] != Tile::Forest {
                return offset;
            }
        }
        unreachable!() // dead end
    }
}

struct Junctions {
    map: Map,
    adj: Vec<Vec<(usize, u32)>>,
    queue: VecDeque<(usize, Vector, Vector)>,
    indices: Grid<Option<usize>>,
    extra: u32,
}

impl Junctions {
    fn new(map: Map) -> Self {
        let start = v(1, 0);
        let mut end = map.tiles.dim() - v(2, 1);

        let mut steps = 1;
        let mut dir = NORTH;
        end += dir;
        while !map.is_junction(end) {
            steps += 1;
            dir = map.continue_path(end, dir);
            end += dir;
        }

        let queue = VecDeque::from([(0, start, SOUTH)]);
        let adj = vec![Vec::new(); 2];
        let mut indices = map.tiles.map(|_| None);
        indices[start] = Some(0);
        indices[end] = Some(1);

        Self {
            map,
            adj,
            queue,
            indices,
            extra: steps,
        }
    }

    fn get_graph(mut self, blocked: impl Fn(Vector, Tile) -> bool) -> (Graph, u32) {
        while let Some((i, mut pos, mut dir)) = self.queue.pop_front() {
            let mut steps = 0;
            loop {
                steps += 1;
                pos += dir;
                if blocked(dir, self.map.tiles[pos]) {
                    break;
                }
                if let Some(j) = self.indices[pos] {
                    self.adj[i].push((j, steps));
                    break;
                }
                if self.map.is_junction(pos) {
                    let j = self.adj.len();
                    self.adj.push(Vec::new());
                    self.indices[pos] = Some(j);
                    self.adj[i].push((j, steps));
                    for offset in ORTHOGONAL {
                        if self.map.tiles[pos + offset] != Tile::Forest {
                            self.queue.push_back((j, pos, offset));
                        }
                    }
                    break;
                }
                dir = self.map.continue_path(pos, dir);
            }
        }

        (Graph::new(self.adj), self.extra)
    }
}

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
        for &(j, s) in &self.adj[i] {
            if j == end {
                *max = (*max).max(steps + s);
            } else if !visited[j] {
                visited[j] = true;
                self.search(j, end, steps + s, visited, max);
                visited[j] = false;
            }
        }
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
