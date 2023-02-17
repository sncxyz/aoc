aoc::parts!(1, 2);

use grid::{constants::*, v, Grid, Vector};
use search::dijkstra;

fn part_1(input: &[&str]) -> impl ToString {
    lowest_total_risk(parse(input))
}

fn part_2(input: &[&str]) -> impl ToString {
    let levels = parse(input);
    lowest_total_risk(Grid::from_fn(
        levels.width() * 5,
        levels.height() * 5,
        |pos| {
            (levels[Vector::new(pos.x % levels.width(), pos.y % levels.height())] - 1
                + (pos.x / levels.width()) as u8
                + (pos.y / levels.height()) as u8)
                % 9
                + 1
        },
    ))
}

fn lowest_total_risk(levels: Grid<u8>) -> u16 {
    let end = levels.dim() - v!(1, 1);
    dijkstra(
        (ZERO, 0),
        |&(pos, cost)| {
            let mut adj = Vec::new();
            for offset in ORTHOGONAL {
                let p = pos + offset;
                if levels.in_bounds(p) {
                    adj.push((p, cost + levels[p] as u16));
                }
            }
            adj
        },
        |&(pos, _)| pos,
        |&(_, cost)| cost,
    )
    .find(|&(pos, ..)| pos == end)
    .unwrap()
    .1
}

fn parse(input: &[&str]) -> Grid<u8> {
    let parse = input.iter().flat_map(|line| line.bytes()).map(|b| b - b'0');
    Grid::from_iter(input[0].len() as i64, input.len() as i64, parse)
}
