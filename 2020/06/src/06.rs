aoc::parts!(1, 2);

use rustc_hash::FxHashSet as HashSet;

fn part_1(input: &[&str]) -> impl ToString {
    let mut total = 0;
    let mut group = HashSet::default();
    for line in input {
        if line.is_empty() {
            total += group.len();
            group.clear();
        } else {
            group = &group | &parse(line);
        }
    }
    total + group.len()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut total = 0;
    let mut group = HashSet::default();
    let mut new = true;
    for line in input {
        if line.is_empty() {
            total += group.len();
            group.clear();
            new = true;
        } else {
            group = if new {
                new = false;
                parse(line)
            } else {
                &group & &parse(line)
            };
        }
    }
    total + group.len()
}

fn parse(line: &str) -> HashSet<char> {
    line.chars().collect()
}
