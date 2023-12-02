aoc::parts!(1, 2);

use std::ops::BitAnd;

fn part_1(input: &[&str]) -> impl ToString {
    let claims = parse(input).0;
    let mut overlaps = Vec::with_capacity(claims.len());
    for (claim_a, &region_a) in claims.iter().enumerate() {
        let mut overlaps_a = Vec::new();
        for (claim_b, &region_b) in claims.iter().enumerate().skip(claim_a + 1) {
            if (region_a & region_b).is_valid() {
                overlaps_a.push(claim_b);
            }
        }
        overlaps.push(overlaps_a);
    }
    let state = State { claims, overlaps };
    state
        .claims
        .iter()
        .enumerate()
        .map(|(claim, &region)| count(&state, claim, region, 1))
        .sum::<i32>()
}

fn part_2(input: &[&str]) -> impl ToString {
    let (claims, ids) = parse(input);
    let mut overlaps = vec![false; claims.len()];
    for (claim_a, &region_a) in claims.iter().enumerate() {
        for (claim_b, &region_b) in claims.iter().enumerate().skip(claim_a + 1) {
            if (region_a & region_b).is_valid() {
                overlaps[claim_a] = true;
                overlaps[claim_b] = true;
            }
        }
        if !overlaps[claim_a] {
            return ids[claim_a];
        }
    }
    0
}

fn parse(input: &[&str]) -> (Vec<Rect>, Vec<u32>) {
    let mut claims = Vec::with_capacity(input.len());
    let mut ids = Vec::with_capacity(input.len());
    for &line in input {
        let mut parts = line[1..].split(" @ ");
        ids.push(parts.next().unwrap().parse().unwrap());
        let mut parts = parts.next().unwrap().split(": ");
        let mut pos = parts.next().unwrap().split(',');
        let mut dim = parts.next().unwrap().split('x');
        let claim = Rect::new(
            pos.next().unwrap().parse().unwrap(),
            pos.next().unwrap().parse().unwrap(),
            dim.next().unwrap().parse().unwrap(),
            dim.next().unwrap().parse().unwrap(),
        );
        claims.push(claim);
    }
    (claims, ids)
}

fn count(state: &State, claim: usize, region: Rect, depth: i32) -> i32 {
    let mut area = 0;
    for &claim in &state.overlaps[claim] {
        let region = region & state.claims[claim];
        if region.is_valid() {
            area += region.area() * depth - count(state, claim, region, depth + 1);
        }
    }
    area
}

struct State {
    claims: Vec<Rect>,
    overlaps: Vec<Vec<usize>>,
}

#[derive(Clone, Copy)]
struct Rect {
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
}

impl Rect {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x_min: x,
            y_min: y,
            x_max: x + width - 1,
            y_max: y + height - 1,
        }
    }

    fn is_valid(self) -> bool {
        self.x_max >= self.x_min && self.y_max >= self.y_min
    }

    fn area(self) -> i32 {
        (self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1)
    }
}

impl BitAnd for Rect {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x_min: self.x_min.max(rhs.x_min),
            y_min: self.y_min.max(rhs.y_min),
            x_max: self.x_max.min(rhs.x_max),
            y_max: self.y_max.min(rhs.y_max),
        }
    }
}
