aoc::parts!(1, 2);

use grid::{constants::*, v, Vector};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

const ROCKS: [&[Vector]; 5] = [
    &[v!(3, 0), v!(4, 0), v!(5, 0), v!(6, 0)],
    &[v!(4, 0), v!(3, 1), v!(4, 1), v!(5, 1), v!(4, 2)],
    &[v!(3, 0), v!(4, 0), v!(5, 0), v!(5, 1), v!(5, 2)],
    &[v!(3, 0), v!(3, 1), v!(3, 2), v!(3, 3)],
    &[v!(3, 0), v!(4, 0), v!(3, 1), v!(4, 1)],
];

pub fn part_1(input: &[&str]) -> impl ToString {
    let mut tower = Tower::new(input);
    for _ in 0..2022 {
        tower.update();
    }
    tower.height
}

pub fn part_2(input: &[&str]) -> impl ToString {
    let mut tower = Tower::new(input);
    let mut cache = HashMap::default();
    let mut heights = vec![0];
    loop {
        let x = tower.update();
        if tower.count % 5 == 1 {
            if let Some(&(count, height)) = cache.get(&(tower.direction, x)) {
                const N: usize = 1_000_000_000_000;
                let height_diff = tower.height - height;
                let count_diff = tower.count - count;
                let offset_count = N - count;
                return ((offset_count / count_diff) as i64 * height_diff)
                    + heights[count + (offset_count % count_diff)];
            } else {
                cache.insert((tower.direction, x), (tower.count, tower.height));
            }
        }
        heights.push(tower.height);
    }
}

struct Tower {
    rocks: HashSet<Vector>,
    height: i64,
    directions: Vec<Vector>,
    count: usize,
    direction: usize,
}

impl Tower {
    fn new(input: &[&str]) -> Self {
        Self {
            rocks: HashSet::default(),
            height: 0,
            directions: input[0]
                .bytes()
                .map(|b| match b {
                    b'>' => EAST,
                    b'<' => WEST,
                    _ => panic!(),
                })
                .collect(),
            count: 0,
            direction: 0,
        }
    }

    fn update(&mut self) -> i64 {
        let mut pos = v!(0, self.height + 4);
        let j = self.count % 5;
        loop {
            let dir = self.directions[self.direction];
            self.direction = (self.direction + 1) % self.directions.len();
            pos += dir;
            for &v in ROCKS[j] {
                let new = v + pos;
                if new.x == 0 || new.x == 8 || self.rocks.contains(&new) {
                    pos -= dir;
                    break;
                }
            }
            pos += NORTH;
            let mut rest = false;
            for &v in ROCKS[j] {
                let new = v + pos;
                if new.y == 0 || self.rocks.contains(&new) {
                    rest = true;
                    break;
                }
            }
            if rest {
                pos += SOUTH;
                self.count += 1;
                for &v in ROCKS[j] {
                    let new = v + pos;
                    self.height = self.height.max(new.y);
                    self.rocks.insert(new);
                }
                return pos.x;
            }
        }
    }
}
