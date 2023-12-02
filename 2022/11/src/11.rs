aoc::parts!(1, 2);

use std::collections::VecDeque;

fn part_1(input: &[&str]) -> impl ToString {
    monkey_business(input, 20, true)
}

fn part_2(input: &[&str]) -> impl ToString {
    monkey_business(input, 10_000, false)
}

fn monkey_business(input: &[&str], rounds: u64, div_three: bool) -> u64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for i in 0..(input.len() + 1) / 7 {
        monkeys.push(Monkey::parse(&input[i * 7 + 1..i * 7 + 6]));
    }
    let modulus = monkeys.iter().map(|m| m.test).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((j, level)) = monkeys[i].inspect(modulus, div_three) {
                monkeys[j].items.push_back(level);
            }
        }
    }
    let (mut a, mut b) = (0, 0);
    for monkey in monkeys {
        (a, b) = (monkey.count.max(a), monkey.count.clamp(b, a));
    }
    a * b
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    pass: usize,
    fail: usize,
    count: u64,
}

impl Monkey {
    fn parse(input: &[&str]) -> Self {
        Self {
            items: input[0][18..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            operation: Operation::parse(&input[1][23..]),
            test: input[2][21..].parse().unwrap(),
            pass: input[3][29..].parse().unwrap(),
            fail: input[4][30..].parse().unwrap(),
            count: 0,
        }
    }

    fn inspect(&mut self, modulus: u64, div_three: bool) -> Option<(usize, u64)> {
        let mut level = self.operation.apply(self.items.pop_front()?);
        if div_three {
            level /= 3;
        }
        level %= modulus;
        self.count += 1;
        if level % self.test == 0 {
            Some((self.pass, level))
        } else {
            Some((self.fail, level))
        }
    }
}

enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn parse(s: &str) -> Self {
        if s == "* old" {
            Self::Square
        } else {
            let value = s[2..].parse().unwrap();
            if &s[..1] == "+" {
                Self::Add(value)
            } else {
                Self::Mul(value)
            }
        }
    }

    fn apply(&self, old: u64) -> u64 {
        match self {
            Self::Add(value) => old + value,
            Self::Mul(value) => old * value,
            Self::Square => old * old,
        }
    }
}
