aoc::parts!(1, 2);

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use grid::{constants::*, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    set(&wire(&input[0]))
        .intersection(&set(&wire(&input[1])))
        .map(|pos| pos.manhattan(ZERO))
        .min()
        .unwrap()
}

fn part_2(input: &[&str]) -> impl ToString {
    let (wire1, wire2) = (wire(&input[0]), wire(&input[1]));
    set(&wire1)
        .intersection(&set(&wire2))
        .map(|pos| wire1[pos] + wire2[pos])
        .min()
        .unwrap()
}

fn wire(path: &str) -> HashMap<Vector, u64> {
    let mut pos = ZERO;
    let mut distance = 0;
    let mut map = HashMap::default();
    for instruction in path.split(',') {
        let offset = match &instruction[0..1] {
            "U" => NORTH,
            "R" => EAST,
            "D" => SOUTH,
            "L" => WEST,
            _ => panic!(),
        };
        for _ in 0..instruction[1..].parse().unwrap() {
            pos += offset;
            distance += 1;
            map.insert(pos, distance);
        }
    }
    map
}

fn set(map: &HashMap<Vector, u64>) -> HashSet<Vector> {
    map.keys().copied().collect()
}
