aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;
use grid::{constants::*, v, Grid, Vector};
use std::ops::Mul;

const N: i64 = 50;

fn part_1(input: &[&str]) -> i64 {
    let (board, path) = parse(input);
    let mut pos = ZERO;
    while board[pos] != MaybeTile::Open {
        pos += EAST;
    }
    let mut facing = EAST;
    for movement in path {
        match movement {
            Movement::Left => facing = -facing.perp(),
            Movement::Right => facing = facing.perp(),
            Movement::Foward(steps) => {
                for _ in 0..steps {
                    let mut next = pos + facing;
                    if !board.in_bounds(next) || board[next] == MaybeTile::Empty {
                        next = pos;
                        while board.in_bounds(next) && board[next] != MaybeTile::Empty {
                            next -= facing;
                        }
                        next += facing;
                    }
                    if board[next] == MaybeTile::Open {
                        pos = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    (pos.y + 1) * 1000 + (pos.x + 1) * 4 + 3 - num_turns(facing)
}

fn part_2(input: &[&str]) -> i64 {
    let (board, path) = parse(input);
    let mut cube = Cube::new(board);
    for movement in path {
        cube.update(movement);
    }
    let pos = cube.pos + cube.faces[&cube.face].offset * N;
    (pos.y + 1) * 1000 + (pos.x + 1) * 4 + 3 - num_turns(cube.facing)
}

fn parse<'a>(input: &'a [&str]) -> (Grid<MaybeTile>, Path<'a>) {
    let end = input.iter().copied().position(str::is_empty).unwrap();
    let width = input.iter().copied().take(end).map(str::len).max().unwrap() as i64;
    let mut board = Grid::new(width, end as i64, MaybeTile::Empty);
    for (y, line) in input.iter().take(end).enumerate() {
        for (x, b) in line.bytes().enumerate() {
            board[v!(x as i64, y as i64)] = match b {
                b' ' => MaybeTile::Empty,
                b'.' => MaybeTile::Open,
                b'#' => MaybeTile::Wall,
                _ => panic!(),
            };
        }
    }
    (board, Path::new(input[end + 1]))
}

struct Cube {
    faces: HashMap<Direction, Face>,
    face: Direction,
    pos: Vector,
    facing: Vector,
}

impl Cube {
    fn update(&mut self, movement: Movement) {
        match movement {
            Movement::Left => self.facing = -self.facing.perp(),
            Movement::Right => self.facing = self.facing.perp(),
            Movement::Foward(steps) => {
                for _ in 0..steps {
                    let mut next = self.pos + self.facing;
                    let mut next_face = self.face;
                    let mut next_facing = self.facing;
                    if !self.faces[&self.face].grid.in_bounds(next) {
                        next = self.pos;
                        let mut dir = self.faces[&self.face].direction;
                        (next_face, dir) = rotate(self.facing, self.face, dir);
                        while dir != self.faces[&next_face].direction {
                            dir = dir * next_face;
                            next_facing = next_facing.perp();
                            next = v!(N - 1 - next.y, next.x);
                        }
                        if next_facing.x == 0 {
                            next.y = N - 1 - next.y;
                        } else {
                            next.x = N - 1 - next.x;
                        }
                    }
                    if self.faces[&next_face].grid[next] == Tile::Wall {
                        break;
                    } else {
                        self.pos = next;
                        self.face = next_face;
                        self.facing = next_facing;
                    }
                }
            }
        }
    }

    fn new(board: Grid<MaybeTile>) -> Self {
        let mut grids = HashMap::default();
        let mut start = None;
        for y in 0..board.height() / N {
            for x in 0..board.width() / N {
                let pos = v!(x, y);
                if board[pos * N] != MaybeTile::Empty {
                    let mut grid = Grid::new(N, N, Tile::Open);
                    for y in 0..N {
                        for x in 0..N {
                            if board[pos * N + v!(x, y)] == MaybeTile::Wall {
                                grid[v!(x, y)] = Tile::Wall;
                            }
                        }
                    }
                    if start.is_none() {
                        let mut p = ZERO;
                        while grid[p] == Tile::Wall {
                            p += EAST;
                        }
                        start = Some((pos, p));
                    }
                    grids.insert(pos, grid);
                }
            }
        }

        let start = start.unwrap();
        let mut faces = HashMap::default();
        let start_face = Direction::new(Axis::Z, Sign::Positive);
        let mut added = vec![start_face];
        let grid = grids.remove(&start.0).unwrap();
        faces.insert(
            start_face,
            Face::new(grid, Direction::new(Axis::Y, Sign::Positive), start.0),
        );
        while faces.len() < 6 {
            let face = added.pop().unwrap();
            let pos = faces[&face].offset;
            for offset in ORTHOGONAL {
                let pos = pos + offset;
                if let Some(grid) = grids.remove(&pos) {
                    let (new_face, new_dir) = rotate(offset, face, faces[&face].direction);
                    faces.insert(new_face, Face::new(grid, new_dir, pos));
                    added.push(new_face);
                }
            }
        }
        Self {
            faces,
            pos: start.1,
            facing: EAST,
            face: start_face,
        }
    }
}

fn rotate(facing: Vector, face: Direction, dir: Direction) -> (Direction, Direction) {
    let mut rotation = dir;
    for _ in 0..num_turns(facing) {
        rotation = rotation * face;
    }
    (rotation, dir * (rotation * face))
}

struct Face {
    grid: Grid<Tile>,
    direction: Direction,
    offset: Vector,
}

impl Face {
    fn new(grid: Grid<Tile>, direction: Direction, offset: Vector) -> Self {
        Self {
            grid,
            direction,
            offset,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    axis: Axis,
    sign: Sign,
}

impl Direction {
    fn new(axis: Axis, sign: Sign) -> Self {
        Self { axis, sign }
    }
}

impl Mul for Direction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.axis == rhs.axis {
            return self;
        }
        let sign = self.sign
            * rhs.sign
            * match (self.axis, rhs.axis) {
                (Axis::X, Axis::Y) | (Axis::Y, Axis::Z) | (Axis::Z, Axis::X) => Sign::Negative,
                _ => Sign::Positive,
            };
        let axis = match (self.axis, rhs.axis) {
            (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Axis::Z,
            (Axis::X, Axis::Z) | (Axis::Z, Axis::X) => Axis::Y,
            _ => Axis::X,
        };
        Self { axis, sign }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Sign {
    Positive,
    Negative,
}

impl Mul for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Positive, Self::Positive) => Self::Positive,
            (Self::Positive, Self::Negative) => Self::Negative,
            (Self::Negative, Self::Positive) => Self::Negative,
            (Self::Negative, Self::Negative) => Self::Positive,
        }
    }
}

struct Path<'a> {
    line: &'a str,
    i: usize,
}

impl<'a> Path<'a> {
    fn new(line: &'a str) -> Self {
        Self { line, i: 0 }
    }
}

impl<'a> Iterator for Path<'a> {
    type Item = Movement;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.line.len() {
            return None;
        }
        self.i += 1;
        Some(match self.line.as_bytes()[self.i - 1] {
            b'L' => Movement::Left,
            b'R' => Movement::Right,
            _ => {
                let (j, mut k) = (self.i - 1, self.i);
                while k < self.line.len() && (b'0'..=b'9').contains(&self.line.as_bytes()[k]) {
                    k += 1;
                }
                self.i = k;
                Movement::Foward(self.line[j..k].parse().unwrap())
            }
        })
    }
}

#[derive(Debug)]
enum Movement {
    Foward(i64),
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MaybeTile {
    Empty,
    Open,
    Wall,
}

fn num_turns(facing: Vector) -> i64 {
    match facing {
        NORTH => 0,
        WEST => 1,
        SOUTH => 2,
        EAST => 3,
        _ => unreachable!(),
    }
}
