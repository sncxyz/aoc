aoc::parts!(1);

use grid::{Grid, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    let mut map = parse(input);
    let mut count = 0;
    let mut updated = true;
    while updated {
        (map, updated) = step(map);
        count += 1;
    }
    count
}

fn step(map: Grid<Cucumber>) -> (Grid<Cucumber>, bool) {
    let mut new = Grid::new(map.width(), map.height(), Cucumber::Empty);
    let mut updated = false;
    for (pos, cucumber) in map.iter_positions() {
        if cucumber == &Cucumber::East {
            let moved = Vector::new((pos.x + 1) % map.width(), pos.y);
            if map[moved] == Cucumber::Empty {
                new[moved] = Cucumber::East;
                updated = true;
            } else {
                new[pos] = Cucumber::East;
            }
        }
    }
    for (pos, cucumber) in map.iter_positions() {
        if cucumber == &Cucumber::South {
            let moved = Vector::new(pos.x, (pos.y + 1) % map.height());
            if map[moved] == Cucumber::South || new[moved] == Cucumber::East {
                new[pos] = Cucumber::South;
            } else {
                new[moved] = Cucumber::South;
                updated = true;
            }
        }
    }
    (new, updated)
}

fn parse(input: &[&str]) -> Grid<Cucumber> {
    let parse = input.iter().flat_map(|line| line.chars()).map(|c| match c {
        '.' => Cucumber::Empty,
        '>' => Cucumber::East,
        'v' => Cucumber::South,
        _ => panic!(),
    });
    Grid::from_iter(input[0].len() as i64, input.len() as i64, parse)
}

#[derive(PartialEq, Eq, Clone)]
enum Cucumber {
    Empty,
    East,
    South,
}
