use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    part_n(input, Trench::parse_1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_n(input, Trench::parse_2)
}

fn part_n(input: aoc::Input, parse: fn(&str) -> Trench) -> i64 {
    let mut clockwise_area = 0;
    let mut extra = 1;
    let mut x = 0;
    for trench in input.lines().map(parse) {
        match trench.dir {
            Dir::Down => {
                clockwise_area += x * trench.len; // add area
                extra += trench.len; // add extra
            }
            Dir::Up => {
                clockwise_area -= x * trench.len; // subtract area
            }
            Dir::Right => {
                extra += trench.len; // add extra
                x += trench.len; // add position
            }
            Dir::Left => {
                x -= trench.len; // subtract position
            }
        }
    }
    clockwise_area.abs() + extra
}

struct Trench {
    dir: Dir,
    len: i64,
}

impl Trench {
    fn parse_1(line: &str) -> Self {
        let dir = match line.idx(0) {
            b'U' => Dir::Up,
            b'D' => Dir::Down,
            b'L' => Dir::Left,
            b'R' => Dir::Right,
            _ => unreachable!(),
        };
        let len = line.as_parser().between(" ", " ").parse_uw();
        Self { dir, len }
    }

    fn parse_2(line: &str) -> Self {
        let hex = &line.as_parser().after("(")[1..7];
        let dir = match hex.idx(5) {
            b'3' => Dir::Up,
            b'1' => Dir::Down,
            b'2' => Dir::Left,
            b'0' => Dir::Right,
            _ => unreachable!(),
        };
        let len = i64::from_str_radix(&hex[..5], 16).unwrap();
        Self { dir, len }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
