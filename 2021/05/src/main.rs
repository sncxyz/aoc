aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;
use grid::Vector;

fn part_1(input: &[&str]) -> impl ToString {
    solve(input, false)
}

fn part_2(input: &[&str]) -> impl ToString {
    solve(input, true)
}

fn parse(line: &str) -> (Vector, Vector) {
    let mut ends = line.split(" -> ");
    let one = parse_coords(ends.next().unwrap().split(','));
    let two = parse_coords(ends.next().unwrap().split(','));
    (one, two)
}

fn parse_coords(mut coords: impl Iterator<Item = impl AsRef<str>>) -> Vector {
    Vector::new(
        coords.next().unwrap().as_ref().parse().unwrap(),
        coords.next().unwrap().as_ref().parse().unwrap(),
    )
}

fn solve(input: &[&str], part: bool) -> usize {
    let mut vents = HashMap::default();
    for vent in input {
        let (a, b) = parse(vent);
        let dir = (b - a).signum();
        if part || dir.x == 0 || dir.y == 0 {
            let mut pos = a - dir;
            while pos != b {
                pos += dir;
                *vents.entry(pos).or_insert(0) += 1;
            }
        }
    }
    vents.into_values().filter(|overlaps| *overlaps > 1).count()
}
