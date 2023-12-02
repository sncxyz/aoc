aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    let (numbers, mut boards) = parse(input);
    for &number in &numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(number) {
                return score;
            }
        }
    }
    0
}

fn part_2(input: &[&str]) -> impl ToString {
    let (numbers, mut boards) = parse(input);
    let mut last = 0;
    for &number in &numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(number) {
                last = score;
            }
        }
    }
    last
}

fn parse(input: &[&str]) -> (Vec<u32>, Vec<Board>) {
    let mut boards = Vec::new();
    let mut i = 2;
    while i < input.len() {
        boards.push(Board::new(&input[i..i + 5]));
        i += 6;
    }
    (
        input[0].split(',').map(|n| n.parse().unwrap()).collect(),
        boards,
    )
}

struct Board {
    numbers: HashMap<u32, (usize, usize)>,
    rows: [u8; 5],
    columns: [u8; 5],
    score: u32,
    completed: bool,
}

impl Board {
    fn new(input: &[&str]) -> Board {
        let mut numbers = HashMap::default();
        let mut score = 0;
        for (row, line) in input.iter().enumerate() {
            for (column, number) in line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .enumerate()
            {
                numbers.insert(number, (row, column));
                score += number;
            }
        }
        Board {
            numbers,
            rows: [0; 5],
            columns: [0; 5],
            score,
            completed: false,
        }
    }

    pub fn mark(&mut self, number: u32) -> Option<u32> {
        if self.completed {
            return None;
        }
        if let Some(&(row, column)) = self.numbers.get(&number) {
            self.rows[row] += 1;
            self.columns[column] += 1;
            self.score -= number;
            if self.rows[row] == 5 || self.columns[column] == 5 {
                self.completed = true;
                return Some(self.score * number);
            }
        }
        None
    }
}
