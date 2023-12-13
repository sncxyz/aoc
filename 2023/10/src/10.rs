use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    parse(input).1 / 2
}

fn part_2(input: aoc::Input) -> impl ToString {
    let tiles = parse(input).0;
    let mut total = 0;
    for y in 0..tiles.height() {
        let mut count = 0;
        let mut entered = None;
        for x in 0..tiles.width() {
            if let Tile::Loop(kind) = tiles[v(x, y)] {
                match kind {
                    LoopKind::Vertical => count += 1,
                    LoopKind::Turn(a) => {
                        if let Some(b) = entered {
                            if a != b {
                                count += 1;
                            }
                            entered = None;
                        } else {
                            entered = Some(a);
                        }
                    }
                    LoopKind::Horizontal => (),
                }
            } else {
                total += count % 2;
            }
        }
    }
    total
}

fn parse(input: aoc::Input) -> (Grid<Tile>, u32) {
    let mut tiles = Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Tile::parse)));
    let start = tiles
        .iter_positions()
        .find_map(|(p, t)| (*t == Tile::Start).then_some(p))
        .unwrap();

    let mut start_dir = ZERO;
    for dir in ORTHOGONAL {
        if let Some(&Tile::Pipe(a, b)) = tiles.get(start + dir) {
            if dir == -a || dir == -b {
                start_dir = dir;
                break;
            }
        }
    }

    let mut dir = start_dir;
    let mut loop_len = 1;
    let mut pos = start + dir;
    while let Tile::Pipe(a, b) = tiles[pos] {
        let old_dir = dir;
        if dir == -a {
            dir = b;
        } else if dir == -b {
            dir = a;
        }
        tiles[pos] = Tile::Loop(LoopKind::new(old_dir, dir));
        loop_len += 1;
        pos += dir;
    }
    tiles[start] = Tile::Loop(LoopKind::new(dir, start_dir));

    (tiles, loop_len)
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ground,
    Start,
    Pipe(Vector, Vector),
    Loop(LoopKind),
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

#[derive(PartialEq, Eq, Clone, Copy)]
enum LoopKind {
    Vertical,
    Horizontal,
    Turn(bool),
}

impl LoopKind {
    fn new(a: Vector, b: Vector) -> Self {
        match (a, b) {
            (SOUTH, SOUTH) | (NORTH, NORTH) => Self::Vertical,
            (EAST, EAST) | (WEST, WEST) => Self::Horizontal,
            (SOUTH, _) | (_, NORTH) => Self::Turn(true),
            (NORTH, _) | (_, SOUTH) => Self::Turn(false),
            _ => unreachable!(),
        }
    }
}
