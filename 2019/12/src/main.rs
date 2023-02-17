aoc::parts!(1, 2);

use fxhash::FxHashSet as HashSet;
use itertools::Itertools;

fn part_1(input: &[&str]) -> impl ToString {
    let mut moons = Moons::new(input);
    moons.step_all(1000);
    moons.total_energy()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut moons = Moons::new(input);
    let x = moons.step_until_repeat(0);
    let y = moons.step_until_repeat(1);
    let z = moons.step_until_repeat(2);
    lcm(x, lcm(y, z))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

struct Moons {
    moons: [[(i32, i32); 4]; 3],
}

impl Moons {
    fn new(input: &[&str]) -> Moons {
        let mut moons = [[(0, 0); 4]; 3];
        for (moon, line) in input.iter().enumerate() {
            let mut parts = line[1..line.len() - 1].split(", ");
            for axis in &mut moons {
                axis[moon] = (parts.next().unwrap()[2..].parse().unwrap(), 0);
            }
        }
        Moons { moons }
    }

    fn step_all(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.step(0);
            self.step(1);
            self.step(2);
        }
    }

    fn step_until_repeat(&mut self, axis: usize) -> usize {
        let mut previous = HashSet::default();
        previous.insert(self.moons[axis]);
        let mut i = 0;
        loop {
            self.step(axis);
            i += 1;
            if previous.contains(&self.moons[axis]) {
                return i;
            }
            previous.insert(self.moons[axis]);
        }
    }

    fn step(&mut self, axis: usize) {
        self.apply_gravity(axis);
        self.apply_velocity(axis);
    }

    fn apply_velocity(&mut self, axis: usize) {
        for moon in 0..4 {
            self.moons[axis][moon].0 += self.moons[axis][moon].1;
        }
    }

    fn apply_gravity(&mut self, axis: usize) {
        for pair in (0..4).permutations(2) {
            self.moons[axis][pair[0]].1 +=
                match self.moons[axis][pair[1]].0 - self.moons[axis][pair[0]].0 {
                    h if h > 0 => 1,
                    h if h < 0 => -1,
                    _ => 0,
                };
        }
    }

    fn total_energy(&self) -> i32 {
        let mut total = 0;
        for moon in 0..4 {
            let mut sum_pos = 0;
            let mut sum_vel = 0;
            for axis in 0..3 {
                sum_pos += self.moons[axis][moon].0.abs();
                sum_vel += self.moons[axis][moon].1.abs();
            }
            total += sum_pos * sum_vel;
        }
        total
    }
}
