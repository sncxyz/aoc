aoc::parts!(1, 2);

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use grid::{constants::*, v, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    let map = parse_input(input);
    best(&map).0
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut map = parse_input(input);
    let station = best(&map).1;
    map.remove(map.iter().position(|&pos| pos == station).unwrap());
    let result = asteroids_ordered(&map, station)[199].position;
    result.x * 100 + result.y
}

fn parse_input(input: &[&str]) -> Vec<Vector> {
    let mut map = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y].as_bytes()[x] == b'#' {
                map.push(v!(x as i64, y as i64));
            }
        }
    }
    map
}

fn best(map: &[Vector]) -> (usize, Vector) {
    let mut largest = 0;
    let mut best = ZERO;
    for asteroid in map {
        let offsets: HashSet<Vector> = map.iter().map(|&v| simplify(v - *asteroid).0).collect();
        if offsets.len() > largest {
            largest = offsets.len();
            best = *asteroid;
        }
    }
    (largest - 1, best)
}

fn asteroids_ordered(map: &[Vector], station: Vector) -> Vec<AsteroidOrd> {
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut lines: HashMap<_, Vec<usize>> = HashMap::default();
    for (i, &position) in map.iter().enumerate() {
        let mut asteroid = Asteroid::new(position, station);
        let indices = lines.entry(asteroid.offset).or_default();
        for &j in &*indices {
            if asteroid.gcd > asteroids[j].gcd {
                let new = asteroids[j].pass + 1;
                if new > asteroid.pass {
                    asteroid.pass = new;
                }
            } else {
                asteroids[j].pass += 1;
            }
        }
        indices.push(i);
        asteroids.push(asteroid);
    }
    let mut ordered: Vec<_> = asteroids.iter().map(AsteroidOrd::new).collect();
    ordered.sort_unstable();
    ordered
}

fn simplify(v: Vector) -> (Vector, i64) {
    let gcd = gcd(v.x.abs(), v.y.abs());
    if gcd == 0 {
        return (ZERO, 0);
    }
    (v / gcd, gcd)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

struct Asteroid {
    position: Vector,
    offset: Vector,
    gcd: i64,
    pub pass: u32,
}

impl Asteroid {
    fn new(position: Vector, station: Vector) -> Asteroid {
        let (offset, gcd) = simplify(position - station);
        Asteroid {
            position,
            offset,
            gcd,
            pass: 0,
        }
    }
}

struct AsteroidOrd {
    position: Vector,
    pass: u32,
    quadrant: u8,
    i: i64,
    j: i64,
}

impl AsteroidOrd {
    fn new(asteroid: &Asteroid) -> AsteroidOrd {
        let (quadrant, i, j) = match asteroid.offset {
            Vector { x, y } if y < 0 && x >= 0 => (0, x, -y),
            Vector { x, y } if y >= 0 && x > 0 => (1, y, x),
            Vector { x, y } if y > 0 && x <= 0 => (2, -x, y),
            Vector { x, y } if y <= 0 && x < 0 => (3, -y, -x),
            _ => panic!(),
        };
        AsteroidOrd {
            position: asteroid.position,
            pass: asteroid.pass,
            quadrant,
            i,
            j,
        }
    }
}

impl PartialEq for AsteroidOrd {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for AsteroidOrd {}

impl PartialOrd for AsteroidOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AsteroidOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pass
            .cmp(&other.pass)
            .then(self.quadrant.cmp(&other.quadrant))
            .then((self.i * other.j).cmp(&(other.i * self.j)))
    }
}
