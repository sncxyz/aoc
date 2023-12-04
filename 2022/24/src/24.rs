aoc::parts!(1, 2);

use grid::prelude::*;

fn part_1(input: &[&str]) -> impl ToString {
    let (start, end, mut blizzards) = parse(input);
    search(&mut blizzards, start, end);
    blizzards.steps
}

fn part_2(input: &[&str]) -> impl ToString {
    let (start, end, mut blizzards) = parse(input);
    search(&mut blizzards, start, end);
    search(&mut blizzards, end, start);
    search(&mut blizzards, start, end);
    blizzards.steps
}

fn search(blizzards: &mut Blizzards, start: Vector, end: Vector) {
    let mut positions = vec![start];
    loop {
        let mut new = Vec::new();
        let mut grid = blizzards.next();
        for pos in positions {
            for offset in ORTHOGONAL_ZERO {
                let next = pos + offset;
                if grid.get(next) == Some(&true) {
                    if next == end {
                        return;
                    }
                    grid[next] = false;
                    new.push(next);
                }
            }
        }
        positions = new;
    }
}

fn parse(input: &[&str]) -> (Vector, Vector, Blizzards) {
    let blizzards = Blizzards::new(input);
    (v(1, 0), blizzards.empty.dim() - v(2, 1), blizzards)
}

struct Blizzards {
    empty: Grid<bool>,
    steps: usize,
    up: Vec<Vector>,
    down: Vec<Vector>,
    left: Vec<Vector>,
    right: Vec<Vector>,
}

impl Blizzards {
    fn new(input: &[&str]) -> Self {
        let mut empty = Grid::new(input[0].len() as i64, input.len() as i64, true);
        let mut up = Vec::new();
        let mut down = Vec::new();
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (y, line) in input.iter().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                let pos = v(x as i64, y as i64);
                match b {
                    b'#' => empty[pos] = false,
                    b'^' => up.push(pos),
                    b'v' => down.push(pos),
                    b'<' => left.push(pos),
                    b'>' => right.push(pos),
                    _ => (),
                }
            }
        }
        Self {
            empty,
            steps: 0,
            up,
            down,
            left,
            right,
        }
    }

    fn next(&mut self) -> Grid<bool> {
        self.steps += 1;
        let mut grid = self.empty.clone();
        for blizzard in &mut self.up {
            *blizzard += NORTH;
            if blizzard.y == 0 {
                blizzard.y = self.empty.height() - 2;
            }
            grid[*blizzard] = false;
        }
        for blizzard in &mut self.down {
            *blizzard += SOUTH;
            if blizzard.y == self.empty.height() - 1 {
                blizzard.y = 1;
            }
            grid[*blizzard] = false;
        }
        for blizzard in &mut self.left {
            *blizzard += WEST;
            if blizzard.x == 0 {
                blizzard.x = self.empty.width() - 2;
            }
            grid[*blizzard] = false;
        }
        for blizzard in &mut self.right {
            *blizzard += EAST;
            if blizzard.x == self.empty.width() - 1 {
                blizzard.x = 1;
            }
            grid[*blizzard] = false;
        }
        grid
    }
}
