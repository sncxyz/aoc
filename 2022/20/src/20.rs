aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    mix(input, 1, 1)
}

fn part_2(input: &[&str]) -> impl ToString {
    mix(input, 811_589_153, 10)
}

fn mix(input: &[&str], multiplier: i64, repetitions: u8) -> i64 {
    let numbers: Vec<_> = input
        .iter()
        .map(|line| line.parse::<i64>().unwrap() * multiplier)
        .collect();
    let x = numbers.len() as i64 - 1;
    let movements: Vec<_> = numbers
        .iter()
        .map(|&n| {
            let n = (n + x / 2).rem_euclid(x) - x / 2;
            if n >= 0 {
                (true, n as usize)
            } else {
                (false, n.unsigned_abs() as usize)
            }
        })
        .collect();

    let mut file = File::new(numbers.len());
    for _ in 0..repetitions {
        file.mix(&movements);
    }

    let zero = file.positions[numbers.iter().position(|&n| n == 0).unwrap()];
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| numbers[file.ids[(zero + i) % file.ids.len()]])
        .sum()
}

struct File {
    ids: Vec<usize>,
    positions: Vec<usize>,
}

impl File {
    fn new(len: usize) -> Self {
        Self {
            ids: (0..len).collect(),
            positions: (0..len).collect(),
        }
    }

    fn mix(&mut self, movements: &[(bool, usize)]) {
        let len = self.ids.len();
        let last = len - 1;
        for (id, &(positive, movement)) in movements.iter().enumerate() {
            let pos = self.positions[id];
            if positive {
                let mut max = pos + movement;
                for i in pos..max.min(last) {
                    self.slide(i + 1, i);
                }
                if max >= len {
                    self.slide(0, last);
                    max -= len;
                    for i in 0..max {
                        self.slide(i + 1, i);
                    }
                }
                self.ids[max] = id;
                self.positions[id] = max;
            } else {
                let mut min = len + pos - movement;
                if pos >= movement {
                    min -= len;
                    for i in (min..pos).rev() {
                        self.slide(i, i + 1);
                    }
                } else {
                    for i in (0..pos).rev() {
                        self.slide(i, i + 1);
                    }
                    self.slide(last, 0);
                    for i in (min..last).rev() {
                        self.slide(i, i + 1);
                    }
                }
                self.ids[min] = id;
                self.positions[id] = min;
            }
        }
    }

    fn slide(&mut self, from: usize, to: usize) {
        let id = self.ids[from];
        self.ids[to] = id;
        self.positions[id] = to;
    }
}
