aoc::parts!(1, 2);

use grid::{constants::*, v, Grid, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    let heights = &parse(input);
    let (w, h) = (heights.width(), heights.height());
    let mut visible = Grid::default(w, h);
    for x in 0..w {
        scan1(heights, &mut visible, v!(x, 0), SOUTH);
        scan1(heights, &mut visible, v!(x, h - 1), NORTH);
    }
    for y in 0..h {
        scan1(heights, &mut visible, v!(0, y), EAST);
        scan1(heights, &mut visible, v!(w - 1, y), WEST);
    }
    visible.into_iter().filter(|v| *v).count()
}

fn part_2(input: &[&str]) -> u64 {
    let heights = &parse(input);
    heights
        .positions()
        .map(|pos| {
            ORTHOGONAL
                .into_iter()
                .map(|dir| scan2(heights, pos, dir))
                .product()
        })
        .max()
        .unwrap()
}

fn parse(input: &[&str]) -> Grid<u8> {
    let parse = input.iter().flat_map(|line| line.bytes()).map(|b| b - 47);
    Grid::from_iter(input[0].len() as i64, input.len() as i64, parse)
}

fn scan1(heights: &Grid<u8>, visible: &mut Grid<bool>, mut pos: Vector, dir: Vector) {
    let mut max = 0;
    while max < 10 && heights.in_bounds(pos) {
        if heights[pos] > max {
            visible[pos] = true;
            max = heights[pos];
        }
        pos += dir;
    }
}

fn scan2(heights: &Grid<u8>, start: Vector, dir: Vector) -> u64 {
    let height = heights[start];
    let mut count = 0;
    let mut pos = start + dir;
    while heights.in_bounds(pos) {
        count += 1;
        if heights[pos] >= height {
            break;
        }
        pos += dir;
    }
    count
}
