aoc::parts!(1, 2);

use grid::prelude::*;

fn part_1(input: &[&str]) -> impl ToString {
    eval(input, v(0, 0), v(1, 0), false)
}

fn part_2(input: &[&str]) -> impl ToString {
    eval(input, v(0, 0), v(10, 1), true)
}

fn eval(input: &[&str], mut pos: Vector, mut dir: Vector, waypoint: bool) -> i64 {
    for line in input {
        let Vector { x, y } = if waypoint { &mut dir } else { &mut pos };
        match (line.chars().next().unwrap(), line[1..].parse().unwrap()) {
            ('N', arg) => *y += arg,
            ('S', arg) => *y -= arg,
            ('E', arg) => *x += arg,
            ('W', arg) => *x -= arg,
            ('L', 90) | ('R', 270) => dir = dir.perp(),
            ('L', 180) | ('R', 180) => dir = -dir,
            ('L', 270) | ('R', 90) => dir = -dir.perp(),
            (_, arg) => pos += arg * dir,
        }
    }
    pos.manhattan(v(0, 0))
}
