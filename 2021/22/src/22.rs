aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let inner = Cuboid {
        bounds: [(-50, 50); 3],
    };
    let mut grid = Grid::new();
    for line in input {
        let (is_on, cuboid) = parse(line);
        if cuboid.overlap(&inner).is_some() {
            grid.update(cuboid, is_on);
        }
    }
    grid.volume()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut grid = Grid::new();
    for line in input {
        let (is_on, cuboid) = parse(line);
        grid.update(cuboid, is_on);
    }
    grid.volume()
}

fn parse(line: &str) -> (bool, Cuboid) {
    let mut parts = line.split(' ');
    (
        parts.next().unwrap() == "on",
        Cuboid::new(parts.next().unwrap()),
    )
}
struct Grid {
    on: Vec<Cuboid>,
    off: Vec<Cuboid>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            on: Vec::new(),
            off: Vec::new(),
        }
    }

    fn update(&mut self, cuboid: Cuboid, is_on: bool) {
        let mut new_on = Vec::new();
        let mut new_off = Vec::new();
        for on in &self.on {
            if let Some(off) = cuboid.overlap(on) {
                new_off.push(off);
            }
        }
        for off in &self.off {
            if let Some(on) = cuboid.overlap(off) {
                new_on.push(on);
            }
        }
        if is_on {
            new_on.push(cuboid);
        }
        self.on.extend(new_on);
        self.off.extend(new_off);
    }

    fn volume(&self) -> i64 {
        self.on.iter().map(|c| c.volume()).sum::<i64>()
            - self.off.iter().map(|c| c.volume()).sum::<i64>()
    }
}

struct Cuboid {
    bounds: [(i64, i64); 3],
}

impl Cuboid {
    fn new(s: &str) -> Cuboid {
        let mut parts = s.split(',');
        Cuboid {
            bounds: [(); 3].map(|_| parse_range(parts.next().unwrap())),
        }
    }

    fn overlap(&self, other: &Cuboid) -> Option<Cuboid> {
        let mut bounds = [(0, 0); 3];
        for (i, bound) in bounds.iter_mut().enumerate() {
            *bound = overlap(self.bounds[i], other.bounds[i])?;
        }
        Some(Cuboid { bounds })
    }

    fn volume(&self) -> i64 {
        self.bounds.map(|(min, max)| max - min + 1).iter().product()
    }
}

fn parse_range(s: &str) -> (i64, i64) {
    let mut parts = s[2..].split("..");
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn overlap(a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    let min = a.0.max(b.0);
    let max = a.1.min(b.1);
    if min > max {
        return None;
    }
    Some((min, max))
}
