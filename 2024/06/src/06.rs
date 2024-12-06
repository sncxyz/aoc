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
    let start = map.enumerate().find(|(_, x)| **x == b'^').unwrap().0;
    map[start] = b'.';

    let mut candidates = HashSet::default();
    let mut pos = start;
    let mut dir = Vector::n();
    while let Some(&x) = map.get(pos + dir) {
        if x == b'#' {
            dir = dir.perp();
        } else {
            pos += dir;
            candidates.insert(pos);
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

    for obstr in candidates {
        let remapped_dests = [Vector::s(), Vector::w(), Vector::n(), Vector::e()].map(|dir| {
            let mut remapped = HashMap::default();
            let mut pos = obstr + dir;
            let dest = pos;
            while map.get(pos) == Some(&b'.') {
                remapped.insert(pos, dest);
                pos += dir;
            }
            remapped
        });

        let mut visited: [HashSet<Vector>; 4] = Default::default();

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
