use std::collections::VecDeque;

use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    Field::parse(input).get_loop().1 / 2
}

fn part_2(input: aoc::Input) -> impl ToString {
    States::new(input).solve()
}

struct States {
    field: Field,
    states: Grid<State>,
}

impl States {
    fn new(input: aoc::Input) -> Self {
        let field = Field::parse(input);
        Self {
            states: Grid::new(field.tiles.width(), field.tiles.height(), State::Unknown),
            field,
        }
    }

    fn solve(&mut self) -> u32 {
        let (loop_dir, _, clockwise) = self.field.get_loop();
        let m = if clockwise { 1 } else { -1 };

        let mut inside = Vec::new();

        let (mut pos, mut dir) = (self.field.start, loop_dir);
        while let Some((p, d)) = self.field.next(pos, dir) {
            self.states[pos] = State::Loop;
            inside.push(pos + dir.perp() * m);
            pos = p;
            inside.push(pos + dir.perp() * m);
            dir = d;
            if pos == self.field.start {
                break;
            }
        }

        let mut total = 0;
        for pos in inside {
            if self.states[pos] == State::Unknown {
                total += self.fill(pos);
            }
        }

        total
    }

    fn fill(&mut self, from: Vector) -> u32 {
        let mut count = 1;
        self.states[from] = State::Inside;
        let mut queue = VecDeque::from([from]);

        while let Some(pos) = queue.pop_front() {
            for dir in ORTHOGONAL {
                let pos = pos + dir;
                if self.states.get(pos).copied() == Some(State::Unknown) {
                    count += 1;
                    self.states[pos] = State::Inside;
                    queue.push_back(pos);
                }
            }
        }

        count
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Loop,
    Inside,
    Unknown,
}

struct Field {
    tiles: Grid<Tile>,
    start: Vector,
}

impl Field {
    fn parse(input: aoc::Input) -> Self {
        let tiles = Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Tile::parse)));
        let start = tiles
            .iter_positions()
            .find_map(|(p, t)| (*t == Tile::Start).then_some(p))
            .unwrap();
        Self { tiles, start }
    }

    fn next(&self, pos: Vector, dir: Vector) -> Option<(Vector, Vector)> {
        let pos = pos + dir;
        match self.tiles.get(pos) {
            Some(&Tile::Pipe(a, b)) => {
                if dir == -a {
                    Some((pos, b))
                } else if dir == -b {
                    Some((pos, a))
                } else {
                    None
                }
            }
            Some(Tile::Start) => Some((pos, dir)),
            _ => None,
        }
    }

    fn get_loop(&self) -> (Vector, u32, bool) {
        let mut right_turns: i32 = 0;
        for start_dir in ORTHOGONAL {
            let mut dir = start_dir;
            let mut pos = self.start;
            let mut steps = 0;
            while let Some((p, d)) = self.next(pos, dir) {
                steps += 1;
                if dir.perp() == d {
                    right_turns += 1;
                } else if dir.perp() == -d {
                    right_turns -= 1;
                }
                if p == self.start {
                    return (start_dir, steps, right_turns > 0);
                }
                (pos, dir) = (p, d);
            }
        }
        unreachable!()
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ground,
    Start,
    Pipe(Vector, Vector),
}

impl Tile {
    fn parse(byte: u8) -> Self {
        match byte {
            b'.' => Self::Ground,
            b'S' => Self::Start,
            b'|' => Self::Pipe(NORTH, SOUTH),
            b'-' => Self::Pipe(EAST, WEST),
            b'L' => Self::Pipe(NORTH, EAST),
            b'J' => Self::Pipe(NORTH, WEST),
            b'7' => Self::Pipe(SOUTH, WEST),
            b'F' => Self::Pipe(SOUTH, EAST),
            _ => unreachable!(),
        }
    }
}
