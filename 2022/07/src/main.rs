aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u64 {
    Sizes::new(input.iter()).filter(|s| *s <= 100_000).sum()
}

fn part_2(input: &[&str]) -> impl ToString {
    let sizes: Vec<_> = Sizes::new(input.iter()).collect();
    let threshold = sizes.last().unwrap() - 40_000_000;
    sizes.into_iter().filter(|s| *s >= threshold).min().unwrap()
}

struct Sizes<I> {
    lines: I,
    stack: Vec<u64>,
}

impl<I> Sizes<I> {
    fn new(lines: I) -> Self {
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

impl<T: AsRef<str>, I: Iterator<Item = T>> Iterator for Sizes<I> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line) = self.lines.next() {
                let line = line.as_ref();
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
