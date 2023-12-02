aoc::parts!(1, 2);

use grid::{constants::*, Grid, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    get_low_points(input).0
}

fn part_2(input: &[&str]) -> impl ToString {
    let low_points = get_low_points(input).1;

    let mut highest = [0; 3];

    for count in low_points.into_iter().map(|h| h as u32) {
        if count > highest[0] {
            highest[2] = highest[1];
            highest[1] = highest[0];
            highest[0] = count;
        } else if count > highest[1] {
            highest[2] = highest[1];
            highest[1] = count;
        } else if count > highest[2] {
            highest[2] = count;
        }
    }

    highest[0] * highest[1] * highest[2]
}

fn get_low_points(input: &[&str]) -> (u16, Grid<u8>) {
    let parse = input.iter().flat_map(|line| line.bytes()).map(|b| b - b'0');
    let heightmap = Grid::from_iter(input[0].len() as i64, input.len() as i64, parse);

    let mut low_points = Grid::default(heightmap.width(), heightmap.height());
    let mut total = 0;

    for pos in heightmap.positions() {
        if heightmap[pos] != 9 {
            let mut current = pos;

            while let Some(adjacent) = lower_neighbour(&heightmap, current) {
                current = adjacent;
            }

            let count = &mut low_points[current];
            if *count == 0 {
                total += heightmap[current] as u16 + 1;
            }
            *count += 1;
        }
    }

    (total, low_points)
}

fn lower_neighbour(heightmap: &Grid<u8>, pos: Vector) -> Option<Vector> {
    let height = heightmap[pos];
    for offset in ORTHOGONAL {
        let adj = pos + offset;
        if let Some(h) = heightmap.get(adj) {
            if *h < height {
                return Some(adj);
            }
        }
    }
    None
}
