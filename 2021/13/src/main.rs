aoc::parts!(1, 2);

use fxhash::FxHashSet as HashSet;
use grid::{constants::*, v, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    let mut i = 0;
    let mut dots = parse(input, &mut i);
    i += 1;
    dots = fold(dots, &input[i]);
    dots.len()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut i = 0;
    let mut dots = parse(input, &mut i);
    i += 1;
    while i < input.len() {
        dots = fold(dots, &input[i]);
        i += 1;
    }

    let mut max = ZERO;
    for &pos in dots.iter() {
        max = max.max(pos);
    }

    let mut result = String::new();
    for y in 0..=max.y {
        for x in 0..=max.x {
            result.push(if dots.contains(&v!(x, y)) { '█' } else { ' ' });
        }
        if y < max.y {
            result.push('\n');
        }
    }
    result
}

fn fold(dots: HashSet<Vector>, line: &str) -> HashSet<Vector> {
    let mut parts = line.split('=');
    let axis = parts.next().unwrap().chars().last().unwrap();
    let value: i64 = parts.next().unwrap().parse().unwrap();
    dots.into_iter()
        .map(|pos| {
            if axis == 'x' {
                Vector::new(value - (pos.x - value).abs(), pos.y)
            } else {
                Vector::new(pos.x, value - (pos.y - value).abs())
            }
        })
        .collect()
}

fn parse(input: &[&str], i: &mut usize) -> HashSet<Vector> {
    let mut dots: HashSet<Vector> = HashSet::default();

    while !input[*i].is_empty() {
        let mut parts = input[*i].split(',');
        dots.insert(Vector::new(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ));
        *i += 1;
    }

    dots
}
