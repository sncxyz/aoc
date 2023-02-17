aoc::parts!(1, 2);

use std::collections::{HashMap, HashSet};

fn part_1(input: &[&str]) -> impl ToString {
    collect(correlate_scanners(input).0).len()
}

fn part_2(input: &[&str]) -> impl ToString {
    let positions = correlate_scanners(input).1;
    let mut largest = 0;
    for &(x1, y1, z1) in &positions {
        for &(x2, y2, z2) in &positions {
            largest = largest.max((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs());
        }
    }
    largest
}

fn correlate_scanners(input: &[&str]) -> (Vec<Beacons>, Vec<(i32, i32, i32)>) {
    let mut relative = parse_relative(input);
    let mut scanners = Vec::with_capacity(relative.len());
    let [first, ..] = relative.remove(0).orientations;
    scanners.push(first);
    let mut positions = vec![(0, 0, 0)];

    let mut i = 0;
    while i < scanners.len() {
        let mut j = 0;
        'outer: while j < relative.len() {
            for k in 0..24 {
                if let Some((beacons, pos)) = scanners[i].overlap(&relative[j].orientations[k]) {
                    scanners.push(beacons);
                    relative.remove(j);
                    positions.push(pos);
                    continue 'outer;
                }
            }
            j += 1;
        }
        i += 1;
    }

    (scanners, positions)
}

fn parse_relative(input: &[&str]) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut i = 1;
    while i < input.len() {
        scanners.push(Scanner::new(input, &mut i));
        i += 2;
    }
    scanners
}

fn collect(scanners: Vec<Beacons>) -> HashSet<Beacon> {
    let mut beacons = HashSet::new();
    for scanner in scanners {
        for i in 0..scanner.x.len() {
            beacons.insert(Beacon {
                x: scanner.x[i],
                y: scanner.y[i],
                z: scanner.z[i],
            });
        }
    }
    beacons
}

struct Scanner {
    orientations: [Beacons; 24],
}

impl Scanner {
    fn new(input: &[&str], i: &mut usize) -> Scanner {
        let base = Beacons::new(input, i);
        let mut orientations = [(); 24].map(|_| base.clone());
        let mut i = 0;
        for y in 0..4 {
            for z in 0..4 {
                orientations[i].rotate(0, y, z);
                i += 1;
            }
        }
        for z in 0..4 {
            orientations[i].rotate(3, 0, z);
            orientations[i + 1].rotate(1, 0, z);
            i += 2;
        }
        Scanner { orientations }
    }
}

#[derive(Clone)]
struct Beacons {
    x: Vec<i32>,
    y: Vec<i32>,
    z: Vec<i32>,
}

impl Beacons {
    fn new(input: &[&str], i: &mut usize) -> Beacons {
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut z = Vec::new();
        while *i < input.len() && !input[*i].is_empty() {
            let mut parts = input[*i].split(',');
            x.push(parts.next().unwrap().parse().unwrap());
            y.push(parts.next().unwrap().parse().unwrap());
            z.push(parts.next().unwrap().parse().unwrap());
            *i += 1;
        }
        Beacons { x, y, z }
    }

    fn overlap(&self, other: &Self) -> Option<(Beacons, (i32, i32, i32))> {
        let x_overlaps = Overlap::all(&self.x, &other.x);
        let y_overlaps = Overlap::all(&self.y, &other.y);
        let z_overlaps = Overlap::all(&self.z, &other.z);

        for (&z, z_overlap) in z_overlaps.iter() {
            for (&y, y_overlap) in y_overlaps.iter() {
                for (&x, x_overlap) in x_overlaps.iter() {
                    if x_overlap.intersect(y_overlap).intersect(z_overlap).count() >= 12 {
                        return Some((other.offset(x, y, z), (x, y, z)));
                    }
                }
            }
        }

        None
    }

    fn offset(&self, x: i32, y: i32, z: i32) -> Beacons {
        Beacons {
            x: self.x.iter().map(|&v| v + x).collect(),
            y: self.y.iter().map(|&v| v + y).collect(),
            z: self.z.iter().map(|&v| v + z).collect(),
        }
    }

    fn rotate(&mut self, x: u8, y: u8, z: u8) {
        for i in 0..self.x.len() {
            for _ in 0..x {
                rotate(&mut self.y[i], &mut self.z[i]);
            }
            for _ in 0..y {
                rotate(&mut self.x[i], &mut self.z[i]);
            }
            for _ in 0..z {
                rotate(&mut self.x[i], &mut self.y[i]);
            }
        }
    }
}

struct Overlap {
    values: Vec<HashSet<usize>>,
}

impl Overlap {
    fn all(a: &[i32], b: &[i32]) -> HashMap<i32, Overlap> {
        let mut overlaps = HashMap::new();
        let min = a.iter().min().unwrap() - b.iter().max().unwrap();
        let max = a.iter().max().unwrap() - b.iter().min().unwrap();
        for offset in min..=max {
            let overlap = Overlap::new(a, b, offset);
            if overlap.count() >= 12 {
                overlaps.insert(offset, overlap);
            }
        }
        overlaps
    }

    fn new(a: &[i32], b: &[i32], offset: i32) -> Overlap {
        let mut values = Vec::with_capacity(a.len());
        for value in a {
            let mut set = HashSet::new();
            for (i, x) in b.iter().enumerate() {
                if *value == *x + offset {
                    set.insert(i);
                }
            }
            values.push(set);
        }
        Overlap { values }
    }

    fn intersect(&self, other: &Self) -> Overlap {
        let mut values = Vec::with_capacity(self.values.len());
        for i in 0..self.values.len() {
            values.push(&self.values[i] & &other.values[i]);
        }
        Overlap { values }
    }

    fn count(&self) -> usize {
        self.values.iter().filter(|value| !value.is_empty()).count()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

fn rotate(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = -*b;
    *b = temp;
}
