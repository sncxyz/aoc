aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;
use grid::{constants::*, Grid, Vector};
use search::bft;

fn part_1(input: &[&str]) -> impl ToString {
    let (maze, portals, start, end) = parse(input);
    let goal = bft(
        State::start(start),
        |s| {
            let mut states = Vec::new();
            let steps = s.steps + 1;
            for offset in ORTHOGONAL {
                let adj = s.pos + offset;
                if maze[adj] == '.' {
                    states.push(State::new(adj, steps, 0));
                }
            }
            if let Some(&pos) = portals.get(&s.pos) {
                states.push(State::new(pos, steps, 0));
            }
            states
        },
        |s| s.pos,
    )
    .find(|s| s.pos == end)
    .unwrap();
    goal.steps
}

fn part_2(input: &[&str]) -> u16 {
    let (maze, portals, start, end) = parse(input);
    let goal = bft(
        State::start(start),
        |s| {
            let mut states = Vec::new();
            let steps = s.steps + 1;
            for offset in ORTHOGONAL {
                let adj = s.pos + offset;
                if maze[adj] == '.' {
                    states.push(State::new(adj, steps, s.layer));
                }
            }
            if let Some(&pos) = portals.get(&s.pos) {
                if outer_portal(s.pos, &maze) {
                    if s.layer > 0 {
                        states.push(State::new(pos, steps, s.layer - 1));
                    }
                } else {
                    states.push(State::new(pos, steps, s.layer + 1));
                }
            }
            states
        },
        |s| (s.pos, s.layer),
    )
    .find(|s| s.pos == end && s.layer == 0)
    .unwrap();
    goal.steps
}

struct State {
    pos: Vector,
    steps: u16,
    layer: u8,
}

impl State {
    fn start(start: Vector) -> State {
        State {
            pos: start,
            steps: 0,
            layer: 0,
        }
    }

    fn new(pos: Vector, steps: u16, layer: u8) -> State {
        State { pos, steps, layer }
    }
}

fn outer_portal(pos: Vector, maze: &Grid<char>) -> bool {
    pos.x == 2 || pos.y == 2 || pos.x == maze.width() - 3 || pos.y == maze.height() - 3
}

fn parse(input: &[&str]) -> (Grid<char>, HashMap<Vector, Vector>, Vector, Vector) {
    let mut maze = Grid::new(input[0].len() as i64, input.len() as i64, ' ');
    let mut pos_iter = maze.positions();
    let mut names = HashMap::default();
    let mut portals = HashMap::default();
    for line in input {
        for c in line.chars() {
            let pos = pos_iter.next().unwrap();
            maze[pos] = c;
            if c.is_ascii_alphabetic() {
                if pos.y > 0 && maze[pos + NORTH].is_ascii_alphabetic() {
                    if pos.y > 1 && maze[pos + NORTH * 2] == '.' {
                        add_portal(
                            pos + NORTH * 2,
                            (maze[pos + NORTH], c),
                            &mut portals,
                            &mut names,
                        );
                    } else {
                        add_portal(
                            pos + SOUTH,
                            (maze[pos + NORTH], c),
                            &mut portals,
                            &mut names,
                        );
                    }
                } else if pos.x > 0 && maze[pos + WEST].is_ascii_alphabetic() {
                    if pos.x > 1 && maze[pos + WEST * 2] == '.' {
                        add_portal(
                            pos + WEST * 2,
                            (maze[pos + WEST], c),
                            &mut portals,
                            &mut names,
                        );
                    } else {
                        add_portal(pos + EAST, (maze[pos + WEST], c), &mut portals, &mut names);
                    }
                }
            }
        }
    }
    (maze, portals, names[&('A', 'A')], names[&('Z', 'Z')])
}

fn add_portal(
    pos: Vector,
    name: (char, char),
    portals: &mut HashMap<Vector, Vector>,
    names: &mut HashMap<(char, char), Vector>,
) {
    if let Some(&dest) = names.get(&name) {
        portals.insert(pos, dest);
        portals.insert(dest, pos);
    } else {
        names.insert(name, pos);
    }
}
