use std::collections::HashMap;

use grid::prelude::*;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut platform = Platform::parse(input);
    platform.tilt(NORTH);
    platform.load()
}

fn part_2(input: aoc::Input) -> impl ToString {
    const CYCLES: usize = 1_000_000_000;

    let mut platform = Platform::parse(input);
    let mut loads = Vec::new();
    let mut history = HashMap::new();

    let mut cycle = 0;
    loop {
        cycle += 1;
        platform.cycle();
        let load = platform.load();
        loads.push(load);
        let platforms = history.entry(load).or_insert(Vec::new());
        for (c, p) in platforms.iter() {
            if p == &platform {
                let offset = (CYCLES - c) % (cycle - c);
                return loads[c + offset - 1];
            }
        }
        platforms.push((cycle, platform.clone()));
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Platform {
    tiles: Grid<Tile>,
}

impl Platform {
    fn parse(input: aoc::Input) -> Self {
        Self {
            tiles: Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Tile::parse))),
        }
    }

    fn tilt(&mut self, dir: Vector) {
        let mut start = v(
            (1 + dir.x + dir.y) / 2 * (self.tiles.width() - 1),
            (1 - dir.x + dir.y) / 2 * (self.tiles.height() - 1),
        );

        while self.tiles.in_bounds(start) {
            let mut pos = start;
            while self.tiles.in_bounds(pos) {
                if self.tiles[pos] == Tile::Rounded {
                    let mut new_pos = pos;
                    while self.tiles.get(new_pos + dir) == Some(&Tile::Empty) {
                        new_pos += dir;
                    }
                    self.tiles[pos] = Tile::Empty;
                    self.tiles[new_pos] = Tile::Rounded;
                }
                pos -= dir;
            }
            start += dir.perp();
        }
    }

    fn cycle(&mut self) {
        self.tilt(NORTH);
        self.tilt(WEST);
        self.tilt(SOUTH);
        self.tilt(EAST);
    }

    fn load(&self) -> i64 {
        self.tiles
            .iter_positions()
            .filter_map(|(pos, &tile)| (tile == Tile::Rounded).then_some(pos))
            .map(|pos| self.tiles.height() - pos.y)
            .sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rounded,
    Cube,
    Empty,
}

impl Tile {
    fn parse(byte: u8) -> Self {
        match byte {
            b'O' => Self::Rounded,
            b'#' => Self::Cube,
            b'.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}
