aoc::parts!(1, 2);

use grid::prelude::*;
use std::ops::{Index, IndexMut};

fn part_1(input: &[&str]) -> impl ToString {
    let mut elves = Elves::new(input);
    for _ in 0..10 {
        elves.update();
    }
    let (min, max) = get_bounds(&elves.elves);
    (max.x - min.x + 1) * (max.y - min.y + 1) - elves.elves.len() as i64
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut elves = Elves::new(input);
    while elves.update() {}
    elves.rounds
}

struct Elves {
    elves: Vec<Vector>,
    states: States,
    proposed: Vec<Option<Vector>>,
    rounds: u32,
}

impl Elves {
    fn new(input: &[&str]) -> Self {
        let mut elves = Vec::new();
        for (y, line) in input.iter().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                if b == b'#' {
                    let pos = v(x as i64, y as i64);
                    elves.push(pos);
                }
            }
        }
        elves.shrink_to_fit();
        let proposed = vec![None; elves.len()];
        Self {
            elves,
            states: States::new(),
            proposed,
            rounds: 0,
        }
    }

    fn update(&mut self) -> bool {
        const CHECKS: [(u8, Vector); 7] = [
            (0b00001110, NORTH),
            (0b11100000, SOUTH),
            (0b00111000, WEST),
            (0b10000011, EAST),
            (0b00001110, NORTH),
            (0b11100000, SOUTH),
            (0b00111000, WEST),
        ];

        self.states.resize(&self.elves);

        let mut moved = false;

        for (i, &elf) in self.elves.iter().enumerate() {
            let mut adj = 0u8;
            for (j, dir) in ADJACENT.into_iter().enumerate() {
                if self.states[elf + dir] == State::Elf {
                    adj |= 1 << j;
                }
            }

            let mut dest = None;
            if adj != 0 {
                for (mask, dir) in CHECKS.into_iter().skip(self.rounds as usize % 4).take(4) {
                    if adj & mask == 0 {
                        dest = Some(elf + dir);
                        break;
                    }
                }
            }

            if let Some(d) = dest {
                self.states[d] = match self.states[d] {
                    State::Empty => State::Proposed,
                    State::Proposed | State::Blocked => {
                        dest = None;
                        State::Blocked
                    }
                    _ => unreachable!(),
                };
            }

            self.proposed[i] = dest;
        }

        for (i, elf) in self.elves.iter_mut().enumerate() {
            if let Some(dest) = self.proposed[i] {
                match self.states[dest] {
                    State::Proposed => {
                        self.states[*elf] = State::Empty;
                        self.states[dest] = State::Elf;
                        *elf = dest;
                        moved = true;
                    }
                    State::Blocked => {
                        self.states[dest] = State::Empty;
                    }
                    _ => unreachable!(),
                }
            }
        }

        self.rounds += 1;
        moved
    }
}

fn get_bounds(elves: &[Vector]) -> (Vector, Vector) {
    let mut min = Vector::MAX;
    let mut max = Vector::MIN;
    for &elf in elves {
        min = min.min(elf);
        max = max.max(elf);
    }
    (min, max)
}

struct States {
    grid: Grid<State>,
    min: Vector,
    max: Vector,
}

impl States {
    const PADDING: i64 = 16;

    fn new() -> Self {
        Self {
            grid: Default::default(),
            min: Vector::MAX,
            max: Vector::MIN,
        }
    }

    fn resize(&mut self, elves: &[Vector]) {
        let (elves_min, elves_max) = get_bounds(elves);
        if elves_min.x <= self.min.x
            || elves_min.y <= self.min.y
            || elves_max.x >= self.max.x
            || elves_max.y >= self.max.y
        {
            *self = Self::new();
            let padding = v(Self::PADDING, Self::PADDING);
            let (min, max) = (elves_min - padding, elves_max + padding);
            let mut grid = Grid::new(max.x - min.x + 1, max.y - min.y + 1, State::Empty);
            for &elf in elves {
                grid[elf - min] = State::Elf;
            }
            *self = Self { grid, min, max };
        }
    }
}

impl Index<Vector> for States {
    type Output = State;

    fn index(&self, index: Vector) -> &Self::Output {
        &self.grid[index - self.min]
    }
}

impl IndexMut<Vector> for States {
    fn index_mut(&mut self, index: Vector) -> &mut Self::Output {
        &mut self.grid[index - self.min]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum State {
    #[default]
    Empty,
    Elf,
    Proposed,
    Blocked,
}
