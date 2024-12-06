use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

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
    let dim: Vector = map.dim();
    let mut start = None;

    let mut up = Matrix::init(dim, None);
    for x in 0..map.width() {
        let mut dest: Option<Vector> = None;
        for y in 0..map.height() {
            if map[v(x, y)] == b'^' {
                map[v(x, y)] = b'.';
                start = Some(v(x, y));
            }
            if map[v(x, y)] == b'#' {
                dest = Some(v(x, y + 1));
            } else {
                up[v(x, y)] = dest;
            }
        }
    }

    let start = start.unwrap();

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

    let mut to_check = HashSet::default();
    let mut pos = start;
    let mut dir = 0;
    loop {
        if let Some(dest) = dests[dir][pos] {
            match dir {
                0 => {
                    for y in dest.y..pos.y {
                        to_check.insert(v(pos.x, y));
                    }
                }
                1 => {
                    for x in (pos.x + 1)..=dest.x {
                        to_check.insert(v(x, pos.y));
                    }
                }
                2 => {
                    for y in (pos.y + 1)..=dest.y {
                        to_check.insert(v(pos.x, y));
                    }
                }
                3 => {
                    for x in dest.x..pos.x {
                        to_check.insert(v(x, pos.y));
                    }
                }
                _ => unreachable!(),
            }
            pos = dest;
        } else {
            match dir {
                0 => {
                    for y in 0..pos.y {
                        to_check.insert(v(pos.x, y));
                    }
                }
                1 => {
                    for x in (pos.x + 1)..dim.x {
                        to_check.insert(v(x, pos.y));
                    }
                }
                2 => {
                    for y in (pos.y + 1)..dim.y {
                        to_check.insert(v(pos.x, y));
                    }
                }
                3 => {
                    for x in 0..pos.x {
                        to_check.insert(v(x, pos.y));
                    }
                }
                _ => unreachable!(),
            }
            break;
        }
        dir = (dir + 1) & 0b11;
    }

    to_check.remove(&start);

    let mut total = 0;

    for obstr in to_check {
        let mut remapped_up = HashMap::default();
        let dir = Vector::s();
        let mut pos = obstr + dir;
        let dest = pos;
        while map.get(pos) == Some(&b'.') {
            remapped_up.insert(pos, dest);
            pos += dir;
        }

        let mut remapped_right = HashMap::default();
        let dir = Vector::w();
        let mut pos = obstr + dir;
        let dest = pos;
        while map.get(pos) == Some(&b'.') {
            remapped_right.insert(pos, dest);
            pos += dir;
        }

        let mut remapped_down = HashMap::default();
        let dir = Vector::n();
        let mut pos = obstr + dir;
        let dest = pos;
        while map.get(pos) == Some(&b'.') {
            remapped_down.insert(pos, dest);
            pos += dir;
        }

        let mut remapped_left = HashMap::default();
        let dir = Vector::e();
        let mut pos = obstr + dir;
        let dest = pos;
        while map.get(pos) == Some(&b'.') {
            remapped_left.insert(pos, dest);
            pos += dir;
        }

        let remapped_dests = [remapped_up, remapped_right, remapped_down, remapped_left];
        let mut visited = [
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
        ];

        let mut dir = 0;
        let mut pos = start;
        loop {
            if let Some(dest) = remapped_dests[dir].get(&pos) {
                pos = *dest;
            } else if let Some(dest) = dests[dir][pos] {
                pos = dest;
            } else {
                break;
            }
            dir = (dir + 1) & 0b11;
            if !visited[dir].insert(pos) {
                total += 1;
                break;
            }
        }
    }

    total
}
