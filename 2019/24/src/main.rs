aoc::parts!(1, 2);

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use grid::{constants::*, v, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    let mut bugs = parse(input);
    let mut rating = biodiversity(&bugs);
    let mut history = HashSet::default();
    while !history.contains(&rating) {
        history.insert(rating);
        bugs = update(bugs, adj);
        rating = biodiversity(&bugs);
    }
    rating
}

fn part_2(input: &[&str]) -> usize {
    let mut bugs = parse(input);
    for _ in 0..200 {
        bugs = update(bugs, level_adj);
    }
    bugs.len()
}

fn parse(input: &[&str]) -> HashSet<Bug> {
    let mut bugs = HashSet::default();
    let mut y = -2;
    for line in input {
        let mut x = -2;
        for c in line.chars() {
            if c == '#' {
                bugs.insert(Bug::new(x, y, 0));
            }
            x += 1;
        }
        y += 1;
    }
    bugs
}

fn update<F, I>(bugs: HashSet<Bug>, mut adjacent: F) -> HashSet<Bug>
where
    F: FnMut(&Bug) -> I,
    I: IntoIterator<Item = Bug>,
{
    let mut counts = HashMap::default();
    for bug in &bugs {
        for adj in adjacent(bug) {
            *counts.entry(adj).or_insert(0) += 1;
        }
    }
    let mut new = HashSet::default();
    for (bug, count) in counts.into_iter() {
        if count == 1 || (count == 2 && !bugs.contains(&bug)) {
            new.insert(bug);
        }
    }
    new
}

fn adj(bug: &Bug) -> Vec<Bug> {
    let mut bugs = Vec::new();
    for pos in ORTHOGONAL {
        let new = bug.pos + pos;
        if new.x.abs() != 3 && new.y.abs() != 3 {
            bugs.push(Bug::new(new.x, new.y, 0));
        }
    }
    bugs
}

fn level_adj(bug: &Bug) -> Vec<Bug> {
    let mut bugs = Vec::new();
    for pos in ORTHOGONAL {
        let new = bug.pos + pos;
        if new.x.abs() == 3 {
            bugs.push(Bug::new(new.x.signum(), 0, bug.level - 1));
        } else if new.y.abs() == 3 {
            bugs.push(Bug::new(0, new.y.signum(), bug.level - 1));
        } else if new == ZERO {
            for z in -2..=2 {
                if bug.pos.x == 0 {
                    bugs.push(Bug::new(z, bug.pos.y * 2, bug.level + 1));
                } else {
                    bugs.push(Bug::new(bug.pos.x * 2, z, bug.level + 1));
                }
            }
        } else {
            bugs.push(Bug::new(new.x, new.y, bug.level));
        }
    }
    bugs
}

fn biodiversity(bugs: &HashSet<Bug>) -> u32 {
    let mut rating = 0;
    for bug in bugs {
        rating |= bug.rating();
    }
    rating
}

#[derive(PartialEq, Eq, Hash)]
struct Bug {
    pos: Vector,
    level: i8,
}

impl Bug {
    fn new(x: i64, y: i64, level: i8) -> Bug {
        Bug {
            pos: v!(x, y),
            level,
        }
    }

    fn rating(&self) -> u32 {
        1 << (self.pos.x + 2 + 5 * (self.pos.y + 2))
    }
}
