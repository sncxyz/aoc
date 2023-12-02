aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut cpu = Cpu::new(input.iter()).skip(18);
    let mut total = cpu.next().unwrap() * 20;
    for i in 1..6 {
        total += cpu.nth(39).unwrap() * (20 + i * 40);
    }
    total
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut crt = String::from("#");
    for (c, x) in (1..240).zip(Cpu::new(input.iter())) {
        if c % 40 == 0 {
            crt.push('\n');
        }
        crt.push(if x.abs_diff(c % 40) <= 1 { '#' } else { '.' });
    }
    crt
}

struct Cpu<I> {
    ins: I,
    x: i32,
    v: Option<i32>,
}

impl<I> Cpu<I> {
    fn new(ins: I) -> Self {
        Self { ins, x: 1, v: None }
    }
}

impl<T: AsRef<str>, I: Iterator<Item = T>> Iterator for Cpu<I> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.v {
            self.x += value;
            self.v = None;
        } else {
            match self.ins.next()?.as_ref() {
                "noop" => (),
                ins => self.v = Some(ins[5..].parse().unwrap()),
            }
        }
        Some(self.x)
    }
}
