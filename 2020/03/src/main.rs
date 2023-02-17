aoc::parts!(1, 2);

use grid::{v, Grid, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    encounters(&parse(input), v!(3, 1))
}

fn part_2(input: &[&str]) -> u64 {
    let map = &parse(input);
    [v!(1, 1), v!(3, 1), v!(5, 1), v!(7, 1), v!(1, 2)]
        .into_iter()
        .map(|offset| encounters(map, offset))
        .product()
}

fn encounters(map: &Grid<bool>, offset: Vector) -> u64 {
    let mut pos = v!(0, 0);
    let mut total = 0;
    while pos.y < map.height() {
        total += map[pos] as u64;
        pos += offset;
        pos.x %= map.width();
    }
    total
}

fn parse(input: &[&str]) -> Grid<bool> {
    let parse = input.iter().flat_map(|line| line.chars()).map(|c| c == '#');
    Grid::from_iter(input[0].len() as i64, input.len() as i64, parse)
}
