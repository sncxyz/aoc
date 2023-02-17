aoc::parts!(1, 2);

use fxhash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    let mut polymer = Polymer::new(input);
    for _ in 0..10 {
        polymer.step();
    }
    polymer.result()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut polymer = Polymer::new(input);
    for _ in 0..40 {
        polymer.step();
    }
    polymer.result()
}

struct Polymer {
    rules: Vec<(usize, usize, usize)>,
    pairs: Vec<u64>,
    frequencies: Vec<u64>,
    count: usize,
}

impl Polymer {
    fn new(input: &[&str]) -> Polymer {
        let mut ids = HashMap::default();
        let mut count = 0;
        for line in &input[2..] {
            for element in line.split(" -> ").next().unwrap().chars() {
                ids.entry(element).or_insert_with(|| {
                    count += 1;
                    count - 1
                });
            }
        }

        let mut frequencies = vec![0; count];
        for element in input[0].chars() {
            frequencies[ids[&element]] += 1;
        }

        let mut pairs = vec![0; count * count];
        for elements in input[0].chars().collect::<Vec<_>>().windows(2) {
            pairs[ids[&elements[0]] * count + ids[&elements[1]]] += 1;
        }

        let mut rules = vec![(0, 0, 0); count * count];
        for line in &input[2..] {
            let mut parts = line.split(" -> ");
            let elements: Vec<_> = parts.next().unwrap().chars().map(|e| ids[&e]).collect();
            let result = ids[&parts.next().unwrap().chars().next().unwrap()];
            rules[elements[0] * count + elements[1]] = (
                elements[0] * count + result,
                result * count + elements[1],
                result,
            );
        }

        Polymer {
            rules,
            pairs,
            frequencies,
            count: count * count,
        }
    }

    fn step(&mut self) {
        let mut new = vec![0; self.count];

        for pair in 0..self.count {
            let count = self.pairs[pair];
            if count > 0 {
                let rule = self.rules[pair];
                new[rule.0] += count;
                new[rule.1] += count;
                self.frequencies[rule.2] += count;
            }
        }

        self.pairs = new;
    }

    fn result(&self) -> u64 {
        *self.frequencies.iter().max().unwrap() - *self.frequencies.iter().min().unwrap()
    }
}
