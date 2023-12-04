aoc::parts!(1, 2);

use grid::prelude::*;

const Y: i64 = 2_000_000;

fn part_1(input: &[&str]) -> impl ToString {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for (sensor, beacon) in parse(input) {
        sensors.push(sensor);
        if beacon.y == Y && !beacons.contains(&beacon.x) {
            beacons.push(beacon.x);
        }
    }
    let beacons = beacons.len() as i64;
    let mut lines = Vec::new();
    for sensor in &sensors {
        let width = sensor.dist - (sensor.pos.y - Y).abs();
        if width >= 0 {
            lines.push(Line {
                start: sensor.pos.x - width,
                end: sensor.pos.x + width,
            });
        }
    }
    lines.sort_unstable_by_key(|line| line.start);
    let mut count = 0;
    let mut lines = lines.into_iter();
    let mut current = lines.next().unwrap();
    for line in lines {
        if line.start <= current.end + 1 {
            current.end = line.end.max(current.end);
        } else {
            count += current.end - current.start + 1;
            current = line;
        }
    }
    count + current.end - current.start + 1 - beacons
}

fn part_2(input: &[&str]) -> impl ToString {
    let sensors: Vec<_> = parse(input).map(|(s, _)| s).collect();
    for (i, a) in sensors.iter().enumerate() {
        for b in sensors.iter().skip(i + 1) {
            if a.dist + b.dist + 2 == a.pos.manhattan(b.pos) {
                let diff = b.pos - a.pos;
                let abs = diff.abs();
                let signum = diff.signum();
                let mut pos = if abs.x > a.dist {
                    v(a.pos.x + (a.dist + 1) * signum.x, a.pos.y)
                } else {
                    v(b.pos.x, b.pos.y - (b.dist + 1) * signum.y)
                };
                let end = if abs.y > a.dist {
                    v(a.pos.x, a.pos.y + (a.dist + 1) * signum.y)
                } else {
                    v(b.pos.x - (b.dist + 1) * signum.x, b.pos.y)
                };
                let diff = end - pos;
                let dir = diff.signum();
                for _ in 0..=diff.x.abs() {
                    if check(&sensors, pos) {
                        return pos.x * 4_000_000 + pos.y;
                    }
                    pos += dir;
                }
            }
        }
    }
    -1
}

fn check(sensors: &[Sensor], pos: Vector) -> bool {
    for sensor in sensors {
        if sensor.pos.manhattan(pos) <= sensor.dist {
            return false;
        }
    }
    true
}

fn parse<'a>(input: &'a [&str]) -> impl Iterator<Item = (Sensor, Vector)> + 'a {
    input.iter().map(|line| {
        let i = line.bytes().position(|b| b == b':').unwrap();
        let sensor = parse_pos(&line[12..i]);
        let beacon = parse_pos(&line[i + 25..]);
        (Sensor::new(sensor, beacon), beacon)
    })
}

fn parse_pos(s: &str) -> Vector {
    let i = s.bytes().position(|b| b == b',').unwrap();
    Vector::new(s[..i].parse().unwrap(), s[i + 4..].parse().unwrap())
}

struct Sensor {
    pos: Vector,
    dist: i64,
}

impl Sensor {
    fn new(pos: Vector, beacon: Vector) -> Self {
        Self {
            pos,
            dist: pos.manhattan(beacon),
        }
    }
}

struct Line {
    start: i64,
    end: i64,
}
