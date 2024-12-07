use std::collections::HashMap;

use nd::{v, Matrix, Vec2};

type Vector = Vec2<i32>;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut map = Matrix::new(input.lines().map(|line| line.bytes()));
    let mut pos = map.enumerate().find(|(_, x)| **x == b'^').unwrap().0;
    let mut dir = Vector::n();
    map[pos] = b'X';
    let mut total = 1;
    while let Some(&x) = map.get(pos + dir) {
        if x == b'#' {
            dir = dir.perp();
        } else {
            pos += dir;
            if map[pos] != b'X' {
                map[pos] = b'X';
                total += 1;
            }
        }
    }
    total
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut map = Matrix::new(input.lines().map(|line| line.bytes()));
    let start = map.enumerate().find(|(_, x)| **x == b'^').unwrap().0;
    map[start] = b'.';

    let mut candidates = HashMap::new();
    let mut pos = start;
    let mut dir = Vector::n();
    while let Some(&x) = map.get(pos + dir) {
        if x == b'#' {
            dir = dir.perp();
        } else {
            pos += dir;
            candidates.entry(pos).or_insert(dir);
        }
    }
    candidates.remove(&start);

    let dim: Vector = map.dim();

    let mut up = Matrix::init(dim, None);
    for x in 0..map.width() {
        let mut dest: Option<Vector> = None;
        for y in 0..map.height() {
            if map[v(x, y)] == b'#' {
                dest = Some(v(x, y + 1));
            } else {
                up[v(x, y)] = dest;
            }
        }
    }

    let mut right = Matrix::init(dim, None);
    for y in 0..map.height() {
        let mut dest: Option<Vector> = None;
        for x in (0..map.width()).rev() {
            if map[v(x, y)] == b'#' {
                dest = Some(v(x - 1, y));
            } else {
                right[v(x, y)] = dest;
            }
        }
    }

    let mut down = Matrix::init(dim, None);
    for x in 0..map.width() {
        let mut dest: Option<Vector> = None;
        for y in (0..map.height()).rev() {
            if map[v(x, y)] == b'#' {
                dest = Some(v(x, y - 1));
            } else {
                down[v(x, y)] = dest;
            }
        }
    }

    let mut left = Matrix::init(dim, None);
    for y in 0..map.height() {
        let mut dest: Option<Vector> = None;
        for x in 0..map.width() {
            if map[v(x, y)] == b'#' {
                dest = Some(v(x + 1, y));
            } else {
                left[v(x, y)] = dest;
            }
        }
    }

    let dests = [up, right, down, left];

    let mut total = 0;

    let dirs = [Vector::w(), Vector::n(), Vector::e(), Vector::s()];

    for (obstr, dir) in candidates {
        let start = obstr - dir;
        let start_dir = dirs.iter().position(|d| *d == dir).unwrap();

        let mut tortoise = start;
        let mut hare = start;
        let mut tortoise_dir = start_dir;
        let mut hare_dir = start_dir;

        loop {
            if let Some(dest) = apply_obstr(hare, hare_dir, dests[hare_dir][hare], obstr) {
                hare = dest;
            } else {
                break;
            }
            hare_dir = (hare_dir + 1) & 0b11;
            if let Some(dest) = apply_obstr(hare, hare_dir, dests[hare_dir][hare], obstr) {
                hare = dest;
            } else {
                break;
            }
            hare_dir = (hare_dir + 1) & 0b11;

            tortoise =
                apply_obstr(tortoise, tortoise_dir, dests[tortoise_dir][tortoise], obstr).unwrap();
            tortoise_dir = (tortoise_dir + 1) & 0b11;

            if tortoise == hare && tortoise_dir == hare_dir {
                total += 1;
                break;
            }
        }
    }

    total
}

fn apply_obstr(start: Vector, dir: usize, end: Option<Vector>, obstr: Vector) -> Option<Vector> {
    match dir {
        0 => {
            if obstr.x == start.x && obstr.y < start.y && end.is_none_or(|end| obstr.y >= end.y) {
                return Some(v(obstr.x, obstr.y + 1));
            }
        }
        1 => {
            if obstr.y == start.y && obstr.x > start.x && end.is_none_or(|end| obstr.x <= end.x) {
                return Some(v(obstr.x - 1, obstr.y));
            }
        }
        2 => {
            if obstr.x == start.x && obstr.y > start.y && end.is_none_or(|end| obstr.y <= end.y) {
                return Some(v(obstr.x, obstr.y - 1));
            }
        }
        _ => {
            if obstr.y == start.y && obstr.x < start.x && end.is_none_or(|end| obstr.x >= end.x) {
                return Some(v(obstr.x + 1, obstr.y));
            }
        }
    }
    end
}
