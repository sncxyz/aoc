aoc::parts!(1, 2);

use grid::prelude::*;

fn part_1(input: &[&str]) -> impl ToString {
    run(input, 4, |pos, offset, map| {
        let pos = pos + offset;
        (map.get(pos) == Some(&true)).then_some(pos)
    })
}

fn part_2(input: &[&str]) -> impl ToString {
    run(input, 5, |pos, offset, map| {
        let mut current = pos + offset;
        while let Some(v) = map.get(current) {
            if *v {
                return Some(current);
            }
            current += offset;
        }
        None
    })
}

fn run(
    input: &[&str],
    threshold: u8,
    next_seat: impl Fn(Vector, Vector, &Grid<bool>) -> Option<Vector>,
) -> usize {
    let parse = input.iter().flat_map(|line| line.chars()).map(|c| c == 'L');
    let map = Grid::from_iter(input[0].len() as i64, input.len() as i64, parse);
    let mut seats = Vec::new();
    for (pos, _) in map.iter_positions().filter(|(_, &s)| s) {
        let check = ADJACENT.into_iter().filter_map(|o| next_seat(pos, o, &map));
        let check = check.collect();
        seats.push(Seat { pos, check });
    }
    let mut occupied = Grid::default(map.width(), map.height());
    let mut toggle = vec![ZERO];
    while !toggle.is_empty() {
        toggle.clear();
        for seat in &seats {
            let mut count = 0;
            for &pos in &seat.check {
                if occupied[pos] {
                    count += 1;
                }
            }
            if (count == 0 && !occupied[seat.pos]) || (count >= threshold && occupied[seat.pos]) {
                toggle.push(seat.pos);
            }
        }
        for &pos in &toggle {
            occupied[pos] = !occupied[pos];
        }
    }
    occupied.into_iter().filter(|o| *o).count()
}

struct Seat {
    pos: Vector,
    check: Vec<Vector>,
}
