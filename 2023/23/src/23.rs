use grid::prelude::*;

aoc::parts!(1, 2);

// TODO: optimise and generalise part 1 using junctions and Graph as in part 2
fn part_1(input: aoc::Input) -> impl ToString {
    let grid = parse_1(input);
    let mut visited = grid.map(|_| false);
    visited[v(1, 0)] = true;
    visited[v(1, 1)] = true;
    search_1(&grid, &mut visited, v(1, 1)) + 1
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut grid = parse_2(input);

    let start = v(1, 0);
    let end = grid.dim() - v(2, 1);
    let mut junctions = vec![
        Junction::new(start, vec![SOUTH]),
        Junction::new(end, Vec::new()),
    ];
    let mut indices = grid.map(|_| None);
    indices[start] = Some(0);
    indices[end] = Some(1);

    let mut dirs = Vec::new();

    while let Some(i) = to_explore(&junctions) {
        let mut pos = junctions[i].pos;
        let mut dir = junctions[i].to_explore.pop().unwrap();
        if grid[pos + dir] {
            continue;
        }
        let mut steps = 0;
        loop {
            pos += dir;
            steps += 1;
            if let Some(j) = indices[pos] {
                junctions[i].adj.push((j, steps));
                junctions[j].adj.push((i, steps));
                grid[pos - dir] = true;
                break;
            }
            for offset in ORTHOGONAL {
                if offset != -dir && !grid[pos + offset] {
                    dirs.push(offset);
                }
            }
            if dirs.is_empty() {
                break;
            }
            if dirs.len() > 1 {
                let j = junctions.len();
                junctions[i].adj.push((j, steps));
                junctions.push(Junction::adj(pos, dirs, i, steps));
                indices[pos] = Some(j);
                dirs = Vec::new();
                break;
            }
            dir = dirs.pop().unwrap();
        }
    }

    let nodes = junctions.into_iter().map(|j| j.adj).collect();
    Graph::new(nodes).longest_path(0, 1)
}

// TODO: attempt to optimise with memoisation
// the same (node, set of visited nodes) always produces the same answer
// pack visited into a u64 for use as a key in a hashmap
// does the order of traversal affect how effective this memoisation will be?
struct Graph {
    nodes: Vec<Vec<(usize, u32)>>,
}

impl Graph {
    fn new(nodes: Vec<Vec<(usize, u32)>>) -> Self {
        Self { nodes }
    }

    fn longest_path(&self, start: usize, end: usize) -> u32 {
        let mut visited = vec![false; self.nodes.len()];
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
        for &(j, s) in &self.nodes[i] {
            if !visited[j] {
                visited[j] = true;
                self.search(j, end, steps + s, visited, max);
                visited[j] = false;
            }
        }
    }
}

fn parse_1(input: aoc::Input) -> Grid<Tile> {
    Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Tile::parse)))
}

fn parse_2(input: aoc::Input) -> Grid<bool> {
    Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(|b| b == b'#')))
}

fn search_1(grid: &Grid<Tile>, visited: &mut Grid<bool>, pos: Vector) -> i32 {
    if pos == grid.dim() - v(2, 1) {
        return 0;
    }
    let mut max = i32::MIN;
    for offset in ORTHOGONAL {
        let pos = pos + offset;
        if !visited[pos] {
            let tile = grid[pos];
            if tile == Tile::Path {
                visited[pos] = true;
                max = max.max(search_1(grid, visited, pos) + 1);
                visited[pos] = false;
            } else if let Tile::Slope(d) = tile {
                if d == offset {
                    visited[pos] = true;
                    let pos2 = pos + offset;
                    visited[pos2] = true;
                    max = max.max(search_1(grid, visited, pos2) + 2);
                    visited[pos] = false;
                    visited[pos2] = false;
                }
            }
        }
    }
    max
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

struct Junction {
    pos: Vector,
    to_explore: Vec<Vector>,
    adj: Vec<(usize, u32)>,
}

impl Junction {
    fn new(pos: Vector, to_explore: Vec<Vector>) -> Self {
        Self {
            pos,
            to_explore,
            adj: Vec::new(),
        }
    }

    fn adj(pos: Vector, to_explore: Vec<Vector>, i: usize, steps: u32) -> Self {
        Self {
            pos,
            to_explore,
            adj: vec![(i, steps)],
        }
    }
}

fn to_explore(junctions: &[Junction]) -> Option<usize> {
    junctions
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, j)| (!j.to_explore.is_empty()).then_some(i))
}
