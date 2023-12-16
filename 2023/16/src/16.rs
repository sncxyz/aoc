use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    Contraption::parse(input).total_from(ZERO, EAST)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut con = Contraption::parse(input);
    let dim = con.tiles.dim();
    let mut max = 0;

    for y in 0..con.tiles.height() {
        max = max.max(con.total_from(v(0, y), EAST));
        max = max.max(con.total_from(v(dim.x - 1, y), WEST));
    }

    for x in 0..con.tiles.width() {
        max = max.max(con.total_from(v(x, 0), SOUTH));
        max = max.max(con.total_from(v(x, dim.y - 1), NORTH));
    }

    max
}

struct Contraption {
    tiles: Grid<Tile>,
    energised: Grid<bool>,
    count: u32,
}

impl Contraption {
    fn parse(input: aoc::Input) -> Self {
        let tiles = Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Tile::parse)));
        let energised = tiles.map(|_| false);
        Self {
            tiles,
            energised,
            count: 0,
        }
    }

    fn total_from(&mut self, pos: Vector, dir: Vector) -> u32 {
        for e in &mut self.energised {
            *e = false;
        }
        self.count = 0;
        self.scan_beam(pos, dir);
        self.count
    }

    fn scan_beam(&mut self, pos: Vector, dir: Vector) {
        if !self.tiles.in_bounds(pos) {
            return;
        }

        let energised = self.energised[pos];
        if !energised {
            self.count += 1;
            self.energised[pos] = true;
        }

        match self.tiles[pos] {
            Tile::Empty => self.scan_beam(pos + dir, dir),
            Tile::Mirror(f) => {
                let dir = v(dir.y, dir.x) * if f { -1 } else { 1 };
                self.scan_beam(pos + dir, dir)
            }
            Tile::Splitter(v) if !energised => {
                if (dir.x == 0) ^ v {
                    self.scan_beam(pos + dir.perp(), dir.perp());
                    self.scan_beam(pos - dir.perp(), -dir.perp());
                } else {
                    self.scan_beam(pos + dir, dir);
                }
            }
            _ => (),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Mirror(bool),
    Splitter(bool),
}

impl Tile {
    fn parse(byte: u8) -> Self {
        match byte {
            b'.' => Self::Empty,
            b'/' => Self::Mirror(true),
            b'\\' => Self::Mirror(false),
            b'|' => Self::Splitter(true),
            b'-' => Self::Splitter(false),
            _ => unreachable!(),
        }
    }
}
