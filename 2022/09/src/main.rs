aoc::parts!(1, 2);

use rustc_hash::FxHashSet as HashSet;
use grid::constants::*;

fn part_1(input: &[&str]) -> impl ToString {
    simulate(input, 2)
}

fn part_2(input: &[&str]) -> impl ToString {
    simulate(input, 10)
}

pub fn simulate(input: &[&str], length: usize) -> usize {
    let mut rope = vec![ZERO; length];
    let mut visited = HashSet::from_iter([ZERO]);
    for line in input {
        for _ in 0..line[2..].parse().unwrap() {
            rope[0] += match &line[..1] {
                "U" => NORTH,
                "D" => SOUTH,
                "R" => EAST,
                "L" => WEST,
                _ => panic!(),
            };
            for i in 1..length {
                let diff = rope[i - 1] - rope[i];
                if diff.x.abs() == 2 || diff.y.abs() == 2 {
                    rope[i] += diff.signum();
                }
            }
            visited.insert(rope[length - 1]);
        }
    }
    visited.len()
}
