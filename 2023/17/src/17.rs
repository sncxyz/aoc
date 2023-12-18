use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    part_n(input, State::adj_1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_n(input, State::adj_2)
}

fn part_n(input: aoc::Input, adj: fn(State, &Grid<u8>, &mut Dirs) -> Vec<State>) -> impl ToString {
    let grid = parse(input);
    let mins = get_mins(&grid);
    let mut dirs = Dirs::new(&grid);
    let heat = search::a_star(
        State::start(),
        |&state| adj(state, &grid, &mut dirs),
        |state| (state.pos, state.dir, state.line),
        |state| state.heat,
        |state| mins[state.pos],
    )
    .find(|state| state.pos == grid.dim() - v(1, 1))
    .unwrap()
    .heat;
    heat
}

#[derive(Clone, Copy)]
struct State {
    pos: Vector,
    dir: Vector,
    line: u8,
    heat: u32,
}

impl State {
    #[inline(always)]
    fn start() -> Self {
        Self {
            pos: ZERO,
            dir: EAST,
            line: 0,
            heat: 0,
        }
    }

    #[inline(always)]
    fn next(self, dir: Vector, line: u8, heat: u8) -> Self {
        Self {
            pos: self.pos + dir,
            dir,
            line,
            heat: self.heat + heat as u32,
        }
    }

    fn jump_4(mut self, grid: &Grid<u8>, dir: Vector) -> Option<Self> {
        for _ in 0..4 {
            if let Some(&heat) = grid.get(self.pos + dir) {
                self = self.next(dir, 4, heat);
            } else {
                return None;
            }
        }
        Some(self)
    }

    fn adj_1(self, grid: &Grid<u8>, dirs: &mut Dirs) -> Vec<Self> {
        let mut adj = Vec::new();
        if dirs.cull(self.pos, self.dir, self.line) {
            return adj;
        }
        if self.line < 3 {
            if let Some(&heat) = grid.get(self.pos + self.dir) {
                adj.push(self.next(self.dir, self.line + 1, heat));
            }
        }
        for dir in [self.dir.perp(), -self.dir.perp()] {
            if let Some(&heat) = grid.get(self.pos + dir) {
                adj.push(self.next(dir, 1, heat));
            }
        }
        adj
    }

    fn adj_2(self, grid: &Grid<u8>, dirs: &mut Dirs) -> Vec<Self> {
        if self.line == 0 {
            return [EAST, SOUTH]
                .into_iter()
                .map(|dir| self.jump_4(grid, dir).unwrap())
                .collect();
        }
        let mut adj = Vec::new();
        if dirs.cull(self.pos, self.dir, self.line) {
            return adj;
        }
        if self.line < 10 {
            if let Some(&heat) = grid.get(self.pos + self.dir) {
                adj.push(self.next(self.dir, self.line + 1, heat));
            }
        }
        for dir in [self.dir.perp(), -self.dir.perp()] {
            if let Some(next) = self.jump_4(grid, dir) {
                adj.push(next);
            }
        }
        adj
    }
}

fn parse(input: aoc::Input) -> Grid<u8> {
    Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(|b| b - b'0')))
}

fn get_mins(grid: &Grid<u8>) -> Grid<u32> {
    let mut mins = grid.map(|_| 0);
    let search = search::dijkstra(
        (grid.dim() - v(1, 1), 0),
        |&(pos, heat)| {
            let h = grid[pos] as u32;
            ORTHOGONAL
                .into_iter()
                .map(move |o| pos + o)
                .filter_map(move |p| grid.in_bounds(p).then_some((p, heat + h)))
        },
        |&(pos, _)| pos,
        |&(_, heat)| heat,
    );
    for (pos, heat) in search {
        mins[pos] = heat;
    }
    mins
}

struct Dirs {
    dirs: Grid<[u8; 4]>,
}

impl Dirs {
    fn new(grid: &Grid<u8>) -> Self {
        Self {
            dirs: grid.map(|_| [u8::MAX; 4]),
        }
    }

    fn cull(&mut self, pos: Vector, dir: Vector, line: u8) -> bool {
        let i = match dir {
            NORTH => 0,
            EAST => 1,
            SOUTH => 2,
            WEST => 3,
            _ => unreachable!(),
        };
        if self.dirs[pos][i] <= line {
            return true;
        }
        self.dirs[pos][i] = line;
        false
    }
}
