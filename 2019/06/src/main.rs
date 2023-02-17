aoc::parts!(1, 2);

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn part_1(input: &[&str]) -> impl ToString {
    let orbits = orbits(input);
    let mut total = 0;
    for object in orbits.keys() {
        total += total_orbits(&orbits, object);
    }
    total
}

fn part_2(input: &[&str]) -> impl ToString {
    let orbits = orbits(input);
    let you = full_path(&orbits, orbits["YOU"]);
    let san = full_path(&orbits, orbits["SAN"]);
    you.len() + san.len() - (&you & &san).len() * 2
}

fn total_orbits(orbits: &HashMap<&str, &str>, object: &str) -> usize {
    if object == "COM" {
        0
    } else {
        total_orbits(orbits, orbits[object]) + 1
    }
}

fn full_path<'a>(orbits: &'a HashMap<&str, &str>, mut object: &'a str) -> HashSet<&'a str> {
    let mut path = HashSet::default();
    while object != "COM" {
        path.insert(object);
        object = orbits[object];
    }
    path
}

fn orbits<'a>(input: &[&'a str]) -> HashMap<&'a str, &'a str> {
    let mut orbits = HashMap::default();
    for orbit in input {
        orbits.insert(&orbit[4..7], &orbit[..3]);
    }
    orbits
}
