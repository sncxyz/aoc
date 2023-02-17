aoc::parts!(1, 2);

use fxhash::FxHashSet as HashSet;

fn part_1(input: &[&str]) -> impl ToString {
    let y = Area::new(input).y.0.abs();
    y * (y - 1) / 2
}

fn part_2(input: &[&str]) -> impl ToString {
    let area = Area::new(input);
    let mut velocities = HashSet::default();

    for steps in 1..=area.y.0.unsigned_abs() * 2 {
        let all_x: Vec<_> = (0..=area.x.1)
            .filter(|&x| valid_x(x, &area, steps))
            .collect();
        for y in (area.y.0..area.y.0.abs()).filter(|&y| valid_y(y, &area, steps)) {
            for &x in &all_x {
                velocities.insert((x, y));
            }
        }
    }

    velocities.len()
}

fn valid_x(mut vel: i32, area: &Area, steps: u32) -> bool {
    let mut pos = 0;
    for _ in 0..steps {
        pos += vel;
        if vel != 0 {
            vel -= vel / vel.abs();
        }
    }
    pos >= area.x.0 && pos <= area.x.1
}

fn valid_y(mut vel: i32, area: &Area, steps: u32) -> bool {
    let mut pos = 0;
    for _ in 0..steps {
        pos += vel;
        vel -= 1;
    }
    pos >= area.y.0 && pos <= area.y.1
}

struct Area {
    x: (i32, i32),
    y: (i32, i32),
}

impl Area {
    fn new(input: &[&str]) -> Area {
        let mut parts = input[0].split(&['=', ',']);
        Area {
            x: parse_pair(parts.nth(1).unwrap()),
            y: parse_pair(parts.nth(1).unwrap()),
        }
    }
}

fn parse_pair(s: &str) -> (i32, i32) {
    let mut parts = s.split("..");
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}
