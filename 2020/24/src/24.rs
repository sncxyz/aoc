aoc::parts!(1, 2);

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use grid::{constants::*, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    flip(input).len()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut tiles = flip(input);
    for _ in 0..100 {
        let mut adjacent = HashMap::default();
        for &pos in &tiles {
            for offset in [NORTH, SOUTH, EAST, WEST, NW, SE] {
                *adjacent.entry(pos + offset).or_insert(0) += 1;
            }
        }
        let mut new = HashSet::default();
        for tile in &tiles {
            if let Some(&count) = adjacent.get(tile) {
                if count == 1 || count == 2 {
                    new.insert(*tile);
                }
            }
        }
        for (&tile, &count) in adjacent.iter() {
            if count == 2 {
                new.insert(tile);
            }
        }
        tiles = new;
    }
    tiles.len()
}

fn flip(input: &[&str]) -> HashSet<Vector> {
    let mut flipped = HashSet::default();
    for line in input {
        let mut pos = ZERO;
        for offset in parse(line) {
            pos += offset;
        }
        if !flipped.insert(pos) {
            flipped.remove(&pos);
        }
    }
    flipped
}

fn parse(line: &str) -> Vec<Vector> {
    let mut i = 0;
    let mut parsed = Vec::new();
    while i < line.len() {
        parsed.push(match &line[i..=i] {
            "n" | "s" => {
                i += 1;
                match &line[i - 1..=i] {
                    "ne" => NORTH,
                    "nw" => NW,
                    "se" => SE,
                    "sw" => SOUTH,
                    _ => panic!(),
                }
            }
            "e" => EAST,
            "w" => WEST,
            _ => panic!(),
        });
        i += 1;
    }
    parsed
}
