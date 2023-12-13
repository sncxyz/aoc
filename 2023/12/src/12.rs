use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> u64 {
    input.lines().map(Row::parse).map(Row::solve).sum()
}

fn part_2(input: aoc::Input) -> u64 {
    input.lines().map(Row::parse_unfold).map(Row::solve).sum()
}

struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
    damaged: usize,
}

impl Row {
    fn parse(line: &str) -> Self {
        let (row, groups) = line.split_once(' ').unwrap();

        let mut damaged = 0;
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
                if spring == Spring::Damaged {
                    damaged += 1;
                }
                springs.push(spring);
                operational = false;
            }
        }

        let groups = groups.split(',').map(Parse::parse_uw).collect();

        Self {
            springs,
            groups,
            damaged,
        }
    }

    fn parse_unfold(line: &str) -> Self {
        let mut row = Self::parse(line);
        row.springs = vec![row.springs; 5].as_slice().join(&Spring::Unknown);
        row.groups = row.groups.repeat(5);
        row.damaged *= 5;
        row
    }

    fn solve(self) -> u64 {
        let num_springs = self.springs.len();

        let width = self.groups.iter().sum::<usize>() + self.groups.len() - 1;
        let len = num_springs + 1 - width;

        let mut min = 0;
        let mut first = vec![0; self.damaged + 1];
        first[0] = 1;
        let mut last = vec![first; len];

        for &group in &self.groups {
            let mut next = Vec::with_capacity(len);
            let mut add = vec![0; self.damaged + 1];
            for i in 0..len {
                if let Some(ds) = self.group_at(min + i, group) {
                    for d in 0..=self.damaged {
                        if last[i][d] > 0 {
                            add[d + ds] += last[i][d];
                        }
                    }
                }
                next.push(add.clone());
            }
            last = next;
            min += group + 1;
        }

        *last.last().unwrap().last().unwrap()
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
            match spring {
                Spring::Operational => return None,
                Spring::Damaged => count += 1,
                _ => (),
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
