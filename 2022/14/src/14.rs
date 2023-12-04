aoc::parts!(1, 2);

use grid::prelude::*;
use rustc_hash::FxHashSet as HashSet;

fn part_1(input: &[&str]) -> impl ToString {
    simulate(input, false)
}

fn part_2(input: &[&str]) -> impl ToString {
    simulate(input, true)
}

fn simulate(input: &[&str], floor: bool) -> u32 {
    let mut map = HashSet::default();
    for line in input {
        let mut points = line.split(" -> ").map(|point| {
            let mut parts = point.split(',');
            Vector::new(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        });
        let mut point = points.next().unwrap();
        map.insert(point);
        for next in points {
            let dir = (next - point).signum();
            while point != next {
                point += dir;
                map.insert(point);
            }
        }
    }
    let max = map.iter().map(|point| point.y).max().unwrap() + 2;
    let mut count = 0;
    rest(v(500, 0), max, &mut map, &mut count, floor);
    count
}

fn rest(sand: Vector, max: i64, map: &mut HashSet<Vector>, count: &mut u32, floor: bool) -> bool {
    if map.contains(&sand) {
        true
    } else if sand.y == max {
        floor
    } else if rest(sand + SOUTH, max, map, count, floor)
        && rest(sand + SW, max, map, count, floor)
        && rest(sand + SE, max, map, count, floor)
    {
        map.insert(sand);
        *count += 1;
        true
    } else {
        false
    }
}
