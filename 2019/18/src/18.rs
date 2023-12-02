aoc::parts!(1, 2);

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use grid::{constants::*, Grid, Vector};
use search::{bft, dijkstra};

fn part_1(input: &[&str]) -> impl ToString {
    let (map, start, keys, ids) = parse(input);
    let goal = dijkstra(
        Vault::new(vec![start], &keys),
        |v| v.adjacent(&map, &keys, &ids),
        |v| (v.id, v.robots[0]),
        |v| v.steps,
    )
    .find(|v| v.complete(keys.len()))
    .unwrap();
    goal.steps
}

fn part_2(input: &[&str]) -> u16 {
    let (mut map, start, keys, ids) = parse(input);
    for offset in ORTHOGONAL_ZERO {
        map[start + offset] = true;
    }
    let mut robots = Vec::new();
    for offset in DIAGONAL {
        robots.push(start + offset);
    }
    let goal = dijkstra(
        Vault::new(robots, &keys),
        |v| v.adjacent(&map, &keys, &ids),
        |v| (v.id, v.robots[0], v.robots[1], v.robots[2], v.robots[3]),
        |v| v.steps,
    )
    .find(|v| v.complete(keys.len()))
    .unwrap();
    goal.steps
}

fn parse(
    input: &[&str],
) -> (
    Grid<bool>,
    Vector,
    HashMap<Vector, Option<Vector>>,
    HashMap<Vector, u8>,
) {
    let mut map = Grid::default(input[0].len() as i64, input.len() as i64);
    let mut start = ZERO;
    let mut pos_iter = map.positions();
    let mut count = 0;
    let mut keys = HashMap::default();
    let mut ids = HashMap::default();
    let mut chars = HashMap::default();
    for line in input {
        for c in line.chars() {
            let pos = pos_iter.next().unwrap();
            match c {
                '.' => (),
                '#' => map[pos] = true,
                '@' => start = pos,
                _ => {
                    if c.is_ascii_lowercase() {
                        count += 1;
                        ids.insert(pos, count - 1);
                        if let Some(door) = chars.get(&c) {
                            keys.insert(pos, Some(*door));
                        } else {
                            keys.insert(pos, None);
                            chars.insert(c, pos);
                        }
                    } else {
                        let lower = c.to_ascii_lowercase();
                        if let Some(key) = chars.get(&lower) {
                            *keys.get_mut(key).unwrap() = Some(pos);
                        } else {
                            chars.insert(lower, pos);
                        }
                    }
                }
            }
        }
    }
    (map, start, keys, ids)
}

#[derive(Clone)]
struct Vault {
    robots: Vec<Vector>,
    keys: HashSet<Vector>,
    doors: HashSet<Vector>,
    steps: u16,
    id: u32,
}

impl Vault {
    fn new(robots: Vec<Vector>, keys: &HashMap<Vector, Option<Vector>>) -> Vault {
        Vault {
            robots,
            keys: keys.keys().copied().collect(),
            doors: keys.values().copied().flatten().collect(),
            steps: 0,
            id: 0,
        }
    }

    fn step(&self, map: &Grid<bool>, robot: usize) -> Vec<Vault> {
        if self.at_key(robot) {
            return Vec::new();
        }
        let mut vaults = Vec::new();
        for offset in ORTHOGONAL {
            let adj = self.robots[robot] + offset;
            if !map[adj] && !self.doors.contains(&adj) {
                let mut new = self.clone();
                new.robots[robot] = adj;
                new.steps += 1;
                vaults.push(new);
            }
        }
        vaults
    }

    fn adjacent(
        &self,
        map: &Grid<bool>,
        keys: &HashMap<Vector, Option<Vector>>,
        ids: &HashMap<Vector, u8>,
    ) -> Vec<Vault> {
        let mut vaults = Vec::new();
        for robot in 0..self.robots.len() {
            for vault in bft(self.clone(), |v| v.step(map, robot), |v| v.robots[robot])
                .filter(|v| v.at_key(robot))
            {
                vaults.push(vault.unlock(keys, ids, robot));
            }
        }
        vaults
    }

    fn unlock(
        mut self,
        keys: &HashMap<Vector, Option<Vector>>,
        ids: &HashMap<Vector, u8>,
        robot: usize,
    ) -> Vault {
        self.keys.remove(&self.robots[robot]);
        if let Some(door) = keys[&self.robots[robot]] {
            self.doors.remove(&door);
        }
        self.id |= 1 << ids[&self.robots[robot]];
        self
    }

    fn at_key(&self, robot: usize) -> bool {
        self.keys.contains(&self.robots[robot])
    }

    fn complete(&self, keys: usize) -> bool {
        self.id == (1 << keys) - 1
    }
}
