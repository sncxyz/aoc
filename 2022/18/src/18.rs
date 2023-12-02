aoc::parts!(1, 2);

use std::hash::BuildHasherDefault;

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

const OFFSETS: [(usize, i64); 6] = [(0, -1), (0, 1), (1, -1), (1, 1), (2, -1), (2, 1)];

fn part_1(input: &[&str]) -> impl ToString {
    let (cubes, _) = parse(input);
    let mut area = 0;
    for cube in &cubes {
        for (i, d) in OFFSETS {
            let mut cube = *cube;
            cube[i] += d;
            area += !cubes.contains(&cube) as u32;
        }
    }
    area
}

fn part_2(input: &[&str]) -> u32 {
    let (cubes, bounds) = parse(input);
    let mut counts = HashMap::default();
    for cube in &cubes {
        for (i, d) in OFFSETS {
            let mut cube = *cube;
            cube[i] += d;
            if !cubes.contains(&cube) {
                *counts.entry(cube).or_insert(0) += 1;
            }
        }
    }
    let air = search::bft(
        bounds.map(|(min, _)| min),
        |&cube| {
            OFFSETS
                .into_iter()
                .map(move |(i, d)| {
                    let mut cube = cube;
                    cube[i] += d;
                    (i, cube)
                })
                .filter_map(|(i, cube)| {
                    (!cubes.contains(&cube) && cube[i] >= bounds[i].0 && cube[i] <= bounds[i].1)
                        .then_some(cube)
                })
        },
        |cube| *cube,
    );
    air.filter_map(|cube| counts.get(&cube).copied()).sum()
}

fn parse(input: &[&str]) -> (HashSet<[i64; 3]>, [(i64, i64); 3]) {
    let mut cubes = HashSet::with_capacity_and_hasher(
        input.len(),
        BuildHasherDefault::<rustc_hash::FxHasher>::default(),
    );
    let mut bounds = [(i64::MAX, i64::MIN); 3];
    for line in input {
        let mut parts = line.split(',').map(|s| s.parse().unwrap());
        let cube = [(); 3].map(|_| parts.next().unwrap());
        for i in 0..3 {
            bounds[i].0 = bounds[i].0.min(cube[i] - 1);
            bounds[i].1 = bounds[i].1.max(cube[i] + 1);
        }
        cubes.insert(cube);
    }
    (cubes, bounds)
}
