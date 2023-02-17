aoc::parts!(1, 2);

use grid::{constants::*, v, Grid, Vector};
use std::ops::{Index, IndexMut};

fn part_1(input: &[&str]) -> impl ToString {
    let mut elves = Elves::new(input);
    for _ in 0..10 {
        elves.update();
    }
    let (min, max, len) = (elves.min, elves.max, elves.elves.len());
    (max.x - min.x + 1) * (max.y - min.y + 1) - len as i64
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut elves = Elves::new(input);
    while elves.update() {}
    elves.count
}

struct Elves {
    elves: Vec<Vector>,
    candidates: Vec<Vector>,
    count: usize,
    min: Vector,
    max: Vector,
}

impl Elves {
    fn new(input: &[&str]) -> Self {
        let mut elves = Vec::new();
        let mut min = v!(i64::MAX, i64::MAX);
        let mut max = v!(i64::MIN, i64::MIN);
        for (y, line) in input.iter().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                if b == b'#' {
                    let pos = v!(x as i64, y as i64);
                    min = min.min(pos);
                    max = max.max(pos);
                    elves.push(pos);
                }
            }
        }
        elves.shrink_to_fit();
        Self {
            candidates: Vec::with_capacity(elves.len()),
            elves,
            count: 0,
            min,
            max,
        }
    }

    fn update(&mut self) -> bool {
        const CHECKS: [(u8, Vector); 7] = [
            (0b00000111, NORTH),
            (0b01110000, SOUTH),
            (0b11000001, WEST),
            (0b00011100, EAST),
            (0b00000111, NORTH),
            (0b01110000, SOUTH),
            (0b11000001, WEST),
        ];
        let mut moved = false;

        let mut adjacent = VectorMap::new(self.min, self.max, 0);
        for &elf in &self.elves {
            adjacent.insert(elf, (0u8, false));
        }
        for &elf in &self.elves {
            for (i, dir) in [(0, NW), (1, NORTH), (2, NE), (3, EAST)] {
                if let Some(adj) = adjacent.get_mut(elf + dir) {
                    adj.0 |= 1 << (i + 4);
                    adj.1 = true;
                    adjacent[elf].0 |= 1 << i;
                    adjacent[elf].1 = true;
                }
            }
        }

        let mut proposals = VectorMap::new(self.min, self.max, 1);
        for (i, &elf) in self.elves.iter().enumerate() {
            let adj = adjacent[elf];
            if adj.1 {
                for check in CHECKS.into_iter().skip(self.count % 4).take(4) {
                    if adj.0 & check.0 == 0 {
                        let dest = elf + check.1;
                        if let Some(proposal) = proposals.get_mut(dest) {
                            *proposal = State::Blocked;
                        } else {
                            proposals.insert(dest, State::Proposed(i));
                            self.candidates.push(dest);
                        }
                        break;
                    }
                }
            }
        }

        for candidate in self.candidates.drain(..) {
            if let State::Proposed(from) = proposals[candidate] {
                self.elves[from] = candidate;
                self.min = self.min.min(candidate);
                self.max = self.max.max(candidate);
                moved = true;
            }
        }

        self.count += 1;
        moved
    }
}

#[derive(Clone, Copy)]
enum State {
    Proposed(usize),
    Blocked,
}

struct VectorMap<T> {
    grid: Grid<Option<T>>,
    min: Vector,
}

impl<T> VectorMap<T> {
    fn new(min: Vector, max: Vector, padding: i64) -> Self
    where
        T: Copy,
    {
        Self {
            grid: Grid::new(
                max.x - min.x + 1 + padding * 2,
                max.y - min.y + 1 + padding * 2,
                None,
            ),
            min: min - v!(padding, padding),
        }
    }

    fn insert(&mut self, vector: Vector, value: T) {
        self.grid[vector - self.min] = Some(value)
    }

    fn get(&self, vector: Vector) -> Option<&T> {
        self.grid.get(vector - self.min)?.as_ref()
    }

    fn get_mut(&mut self, vector: Vector) -> Option<&mut T> {
        self.grid.get_mut(vector - self.min)?.as_mut()
    }
}

impl<T> Index<Vector> for VectorMap<T> {
    type Output = T;

    fn index(&self, index: Vector) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<Vector> for VectorMap<T> {
    fn index_mut(&mut self, index: Vector) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
