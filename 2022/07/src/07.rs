use aoc::{Input, Lines};

aoc::parts!(1, 2);

fn part_1(input: Input) -> u64 {
    Sizes::new(input.lines()).filter(|s| *s <= 100_000).sum()
}

fn part_2(input: Input) -> impl ToString {
    let sizes: Vec<_> = Sizes::new(input.lines()).collect();
    let threshold = sizes.last().unwrap() - 40_000_000;
    sizes.into_iter().filter(|s| *s >= threshold).min().unwrap()
}

struct Sizes<'a> {
    lines: Lines<'a>,
    stack: Vec<u64>,
}

impl<'a> Sizes<'a> {
    fn new(lines: Lines<'a>) -> Self {
        Self {
            lines,
            stack: Vec::new(),
        }
    }

    fn pop(&mut self) -> u64 {
        let size = self.stack.pop().unwrap();
        if let Some(last) = self.stack.last_mut() {
            *last += size;
        }
        size
    }
}

impl<'a> Iterator for Sizes<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line) = self.lines.next() {
                if line == "$ cd .." {
                    return Some(self.pop());
                } else if &line[..3] == "dir" || &line[..3] == "$ l" {
                    continue;
                } else if &line[..3] == "$ c" {
                    self.stack.push(0);
                } else {
                    *self.stack.last_mut().unwrap() +=
                        line.split(' ').next().unwrap().parse::<u64>().unwrap();
                }
            } else if !self.stack.is_empty() {
                return Some(self.pop());
            } else {
                return None;
            }
        }
    }
}
