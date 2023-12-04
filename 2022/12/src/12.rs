aoc::parts!(1, 2);

use grid::prelude::*;

fn part_1(input: &[&str]) -> impl ToString {
    let (start, end, heightmap) = parse(input);
    let goal = search(&heightmap, end).find(|s| s.pos == start).unwrap();
    goal.steps
}

fn part_2(input: &[&str]) -> impl ToString {
    let (_, end, heightmap) = parse(input);
    let goal = search(&heightmap, end).find(|s| s.height == b'a').unwrap();
    goal.steps
}

fn parse(input: &[&str]) -> (Vector, Vector, Grid<u8>) {
    let (mut start, mut end) = (ZERO, ZERO);
    let mut heights = input.iter().flat_map(|line| line.bytes());
    let heightmap = Grid::from_fn(
        input[0].len() as i64,
        input.len() as i64,
        |pos| match heights.next().unwrap() {
            b'S' => {
                start = pos;
                b'a'
            }
            b'E' => {
                end = pos;
                b'z'
            }
            height => height,
        },
    );
    (start, end, heightmap)
}

fn search(heightmap: &Grid<u8>, pos: Vector) -> impl Iterator<Item = State> + '_ {
    search::bft(
        State::new(pos, 0, b'z'),
        move |&s| {
            ORTHOGONAL.into_iter().filter_map(move |dir| {
                let pos = s.pos + dir;
                let height = *heightmap.get(pos)?;
                (height + 1 >= s.height).then(|| State::new(pos, s.steps + 1, height))
            })
        },
        |s| s.pos,
    )
}

#[derive(Clone, Copy)]
struct State {
    pos: Vector,
    steps: u32,
    height: u8,
}

impl State {
    fn new(pos: Vector, steps: u32, height: u8) -> Self {
        Self { pos, steps, height }
    }
}
