use std::collections::VecDeque;

use grid::prelude::*;

aoc::parts!(1, 2);

const STEPS_1: i64 = 64;
const STEPS_2: i64 = 26501365;

fn part_1(input: aoc::Input) -> impl ToString {
    let farm = parse(input);
    // assume odd square grid with S in the centre
    count(&farm, farm.dim() / 2, STEPS_1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let farm = parse(input);

    let mut total = 0;

    // assume odd square grid with S in the centre
    let width = farm.width();
    let half = width / 2;

    // four corner cells
    let steps = STEPS_2 - half - 1;
    let corner_steps = steps % width;
    for m in [v(1, 0), v(2, 1), v(1, 2), v(0, 1)] {
        total += count(&farm, m * half, corner_steps);
    }

    // edge cells
    let steps = STEPS_2 - width - 1;
    let dist = steps / width;
    let outer_steps = steps % width;
    let inner_steps = outer_steps + width;

    let outers = dist + 1;
    let inners = dist;

    let (mut outer, mut inner) = (0, 0);
    for m in [v(0, 0), v(1, 0), v(1, 1), v(0, 1)] {
        outer += count(&farm, m * (width - 1), outer_steps);
        inner += count(&farm, m * (width - 1), inner_steps);
    }

    total += outer * outers;
    total += inner * inners;

    // interior cells
    let (filled_even, filled_odd) = count_filled(&farm);
    let x = 2 * (dist / 2) + 1;
    let inc = x * x;
    let x = 2 * ((dist + 1) / 2);
    let exc = x * x;
    let (evens, odds) = if STEPS_2 % 2 == 0 {
        (inc, exc)
    } else {
        (exc, inc)
    };
    total += filled_even * evens;
    total += filled_odd * odds;

    total
}

fn parse(input: aoc::Input) -> Grid<u8> {
    Grid::from_nested_iter(input.lines().map(|line| line.bytes()))
}

fn count(farm: &Grid<u8>, start: Vector, steps: i64) -> i64 {
    let mut visited = farm.map(|&tile| tile == b'#');
    visited[start] = true;
    let mut add = steps % 2 == 0;
    let mut count = 0;
    let mut queue = VecDeque::from([start]);
    let mut len = 1;

    for _ in 0..steps {
        count += add as i64 * len;
        for _ in 0..len {
            let pos = queue.pop_front().unwrap();
            for offset in ORTHOGONAL {
                let pos = pos + offset;
                if let Some(v @ false) = visited.get_mut(pos) {
                    *v = true;
                    queue.push_back(pos);
                }
            }
        }
        len = queue.len() as i64;
        add = !add;
    }

    count + add as i64 * len
}

fn count_filled(farm: &Grid<u8>) -> (i64, i64) {
    let mut visited = farm.map(|&tile| tile == b'#');
    let start = farm.dim() / 2;
    visited[start] = true;
    let mut add_even = 1;
    let (mut even, mut odd) = (0, 0);
    let mut queue = VecDeque::from([start]);
    let mut len = 1;

    while len > 0 {
        even += add_even * len;
        odd += (1 - add_even) * len;
        for _ in 0..len {
            let pos = queue.pop_front().unwrap();
            for offset in ORTHOGONAL {
                let pos = pos + offset;
                if let Some(v @ false) = visited.get_mut(pos) {
                    *v = true;
                    queue.push_back(pos);
                }
            }
        }
        len = queue.len() as i64;
        add_even = 1 - add_even;
    }

    (even, odd)
}
