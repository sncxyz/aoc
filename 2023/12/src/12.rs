use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> u64 {
    input.lines().map(Row::parse).map(Row::solve).sum()
}

fn part_2(input: aoc::Input) -> u64 {
    input
        .lines()
        .map(Row::parse)
        .map(Row::unfold)
        .map(Row::solve)
        .sum()
}

struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Row {
    fn parse(line: &str) -> Self {
        let (row, groups) = line.split_once(' ').unwrap();

        let mut springs = Vec::new();
        let mut operational = false;
        for byte in row.bytes() {
            let spring = Spring::parse(byte);
            if spring == Spring::Operational {
                if !operational {
                    springs.push(spring);
                    operational = true;
                }
            } else {
                operational = false;
                springs.push(spring);
            }
        }

        let groups = groups.split(',').map(Parse::parse_uw).collect();

        Self { springs, groups }
    }

    fn unfold(mut self) -> Self {
        self.springs = vec![self.springs; 5].as_slice().join(&Spring::Unknown);
        self.groups = self.groups.repeat(5);
        self
    }

    fn solve(self) -> u64 {
        let total_springs = self.springs.len();
        let mut total_damaged = 0;
        for &spring in &self.springs {
            if spring == Spring::Damaged {
                total_damaged += 1;
            }
        }

        let mut last = Vec::with_capacity(total_springs);
        let mut add = vec![0; total_damaged + 1];
        for i in 0..total_springs {
            if let Some(ds) = self.group_at(i, self.groups[0]) {
                add[ds] += 1;
            }
            last.push(add.clone());
        }

        let mut last_group = self.groups[0] + 1;

        for &group in &self.groups[1..] {
            let mut next = Vec::with_capacity(total_springs);
            let mut add = vec![0; total_damaged + 1];
            for i in 0..total_springs {
                if i >= last_group {
                    if let Some(ds) = self.group_at(i, group) {
                        for d in 0..total_damaged + 1 {
                            if last[i - last_group][d] > 0 {
                                add[d + ds] += last[i - last_group][d];
                            }
                        }
                    }
                }
                next.push(add.clone());
            }
            last = next;
            last_group = group + 1;
        }

        last[total_springs - 1][total_damaged]
    }

    fn group_at(&self, i: usize, g: usize) -> Option<usize> {
        if i > 0 && self.springs[i - 1] == Spring::Damaged {
            return None;
        }
        if i + g < self.springs.len() && self.springs[i + g] == Spring::Damaged {
            return None;
        }
        if i + g > self.springs.len() {
            return None;
        }
        let mut count = 0;
        for &spring in &self.springs[i..i + g] {
            if spring == Spring::Operational {
                return None;
            }
            if spring == Spring::Damaged {
                count += 1;
            }
        }
        Some(count)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn parse(byte: u8) -> Self {
        match byte {
            b'.' => Self::Operational,
            b'#' => Self::Damaged,
            b'?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

use std::fmt;

impl fmt::Debug for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for spring in &self.springs {
            write!(f, "{spring:?}")?;
        }
        write!(f, " ")?;
        write!(f, "{}", self.groups[0])?;
        for group in &self.groups[1..] {
            write!(f, ",{group}")?;
        }
        Ok(())
    }
}
