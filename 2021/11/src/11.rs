aoc::parts!(1, 2);

use grid::{constants::*, Grid, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    let mut cave = Cave::new(input);
    for _ in 0..100 {
        cave.step();
    }
    cave.total_flashes
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut cave = Cave::new(input);
    while cave.step_flashes != 100 {
        cave.step();
    }
    cave.step
}

struct Cave {
    grid: Grid<u8>,
    step: u16,
    total_flashes: u16,
    step_flashes: u16,
}

impl Cave {
    fn new(input: &[&str]) -> Cave {
        let parse = input.iter().flat_map(|line| line.bytes()).map(|b| b - b'0');
        let grid = Grid::from_iter(10, 10, parse);
        Cave {
            grid,
            step: 0,
            total_flashes: 0,
            step_flashes: 0,
        }
    }

    fn step(&mut self) {
        self.step += 1;

        for pos in self.grid.positions() {
            self.grid[pos] += 1;
            if self.grid[pos] == 10 {
                self.increment_adj(pos);
            }
        }

        self.step_flashes = 0;
        for level in &mut self.grid {
            if *level > 9 {
                self.step_flashes += 1;
                *level = 0;
            }
        }

        self.total_flashes += self.step_flashes;
    }

    fn increment_adj(&mut self, pos: Vector) {
        for offset in ADJACENT {
            let adj = pos + offset;
            if let Some(v) = self.grid.get_mut(adj) {
                *v += 1;
                if *v == 10 {
                    self.increment_adj(adj)
                }
            }
        }
    }
}
