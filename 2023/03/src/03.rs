use grid::prelude::*;
use rustc_hash::FxHashSet as HashSet;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> u32 {
    let (cells, nums) = parse(input);
    cells
        .iter_positions()
        .filter_map(|(pos, &cell)| matches!(cell, Cell::Star | Cell::Symbol).then_some(pos))
        .flat_map(|pos| ADJACENT.map(|o| pos + o))
        .filter_map(|pos| cells.get(pos).copied().and_then(Cell::as_index))
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|i| nums[i])
        .sum()
}

fn part_2(input: aoc::Input) -> u32 {
    let (cells, nums) = parse(input);
    cells
        .iter_positions()
        .filter_map(|(pos, &cell)| (cell == Cell::Star).then_some(pos))
        .map(|pos| {
            ADJACENT
                .into_iter()
                .filter_map(|o| cells.get(pos + o).copied().and_then(Cell::as_index))
                .collect::<HashSet<_>>()
        })
        .filter_map(|ps| (ps.len() == 2).then(|| ps.into_iter().map(|i| nums[i]).product::<u32>()))
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Number(u32),
    Index(usize),
    Star,
    Empty,
    Symbol,
}

impl Cell {
    fn parse(byte: u8) -> Self {
        match byte {
            b'0'..=b'9' => Self::Number((byte - b'0') as u32),
            b'*' => Self::Star,
            b'.' => Self::Empty,
            _ => Self::Symbol,
        }
    }

    fn as_index(self) -> Option<usize> {
        if let Self::Index(i) = self {
            Some(i)
        } else {
            None
        }
    }
}

fn parse(input: aoc::Input) -> (Grid<Cell>, Vec<u32>) {
    let mut cells = Grid::from_nested_iter(input.lines().map(|line| line.bytes().map(Cell::parse)));
    let mut numbers = Vec::new();
    let mut i = 0;
    for y in 0..cells.height() {
        let mut init = false;
        for x in 0..cells.width() {
            if let Cell::Number(n) = cells[v(x, y)] {
                if init {
                    numbers[i] *= 10;
                    numbers[i] += n;
                } else {
                    numbers.push(n);
                    init = true;
                }
                cells[v(x, y)] = Cell::Index(i);
            } else if init {
                i += 1;
                init = false;
            }
        }
        i += init as usize;
    }
    (cells, numbers)
}
