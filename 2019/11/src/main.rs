aoc::parts!(1, 2);

use fxhash::FxHashMap as HashMap;
use grid::{constants::*, v, Vector};

fn part_1(input: &[&str]) -> impl ToString {
    paint_hull(0, &input[0]).len()
}

fn part_2(input: &[&str]) -> impl ToString {
    let hull = paint_hull(1, &input[0]);
    let (mut min, mut max) = (ZERO, ZERO);
    for (&pos, &colour) in hull.iter() {
        if colour == 1 {
            min = min.min(pos);
            max = max.max(pos);
        }
    }
    let mut result = String::new();
    for y in (min.y..=max.y).rev() {
        for x in min.x..=max.x {
            result.push(if let Some(1) = hull.get(&v!(x, y)) {
                '█'
            } else {
                ' '
            });
        }
        if y > min.y {
            result.push('\n');
        }
    }
    result
}

fn paint_hull(starting_panel: u8, input: &str) -> HashMap<Vector, u8> {
    let mut computer = intcode::Computer::new(input).unwrap();
    computer.run();
    let mut hull = HashMap::default();
    hull.insert(ZERO, starting_panel);
    let mut current = ZERO;
    let mut mov = v!(0, 1);
    while computer.state() != intcode::State::Halted {
        let colour = hull.entry(current).or_insert(0);
        computer.input(*colour as i64);
        *colour = computer.output().unwrap() as u8;
        mov = match computer.output().unwrap() {
            0 => mov.perp(),
            1 => -mov.perp(),
            _ => panic!(),
        };
        current += mov;
    }
    hull
}
