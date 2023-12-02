aoc::parts!(1, 2);

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use grid::{constants::*, Vector};
use std::collections::VecDeque;

fn part_1(input: &[&str]) -> impl ToString {
    let (oxygen, distances) = map_area(&input[0]);
    distances[&oxygen]
}

fn part_2(input: &[&str]) -> impl ToString {
    let (oxygen, distances) = map_area(&input[0]);
    let area: HashSet<Vector> = distances.keys().cloned().collect();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::default();
    queue.push_back(oxygen);
    visited.insert(oxygen);
    let mut time = 0;

    while !queue.is_empty() {
        let length = queue.len();
        for _ in 0..length {
            let current = queue.pop_front().unwrap();
            for offset in ORTHOGONAL {
                let new = current + offset;
                if !visited.contains(&new) && area.contains(&new) {
                    visited.insert(new);
                    queue.push_back(new);
                }
            }
        }
        if !queue.is_empty() {
            time += 1;
        }
    }

    time
}

fn map_area(input: &str) -> (Vector, HashMap<Vector, usize>) {
    let mut computer = intcode::Computer::new(input).unwrap();
    computer.run();
    let mut distances = HashMap::default();
    let mut unchecked = HashMap::default();
    let mut walls = HashSet::default();
    let start = ZERO;
    let mut stack = vec![start];
    let mut oxygen = start;
    distances.insert(start, 0);
    unchecked.insert(start, Vec::from(ORTHOGONAL));

    'outer: while !stack.is_empty() {
        let current_pos = *stack.last().unwrap();
        let current_dist = distances[&current_pos];
        while let Some(movement) = unchecked.get_mut(&current_pos).unwrap().pop() {
            let new_pos = current_pos + movement;
            if let Some(distance) = distances.get_mut(&new_pos) {
                if *distance > current_dist + 1 {
                    unchecked.insert(new_pos, Vec::from(ORTHOGONAL));
                    *distance = current_dist + 1;
                    stack.push(new_pos);
                    computer.input(to_input(movement));
                    computer.output().unwrap();
                    continue 'outer;
                }
            } else if !walls.contains(&new_pos) {
                computer.input(to_input(movement));
                let status = computer.output().unwrap();
                if status == 2 {
                    oxygen = new_pos;
                }
                if status > 0 {
                    unchecked.insert(new_pos, Vec::from(ORTHOGONAL));
                    distances.insert(new_pos, current_dist + 1);
                    stack.push(new_pos);
                    continue 'outer;
                } else {
                    walls.insert(new_pos);
                }
            }
        }
        let current_pos = stack.pop().unwrap();
        if let Some(&new_pos) = stack.last() {
            computer.input(to_input(new_pos - current_pos));
            computer.output().unwrap();
        }
    }

    (oxygen, distances)
}

fn to_input(pos: Vector) -> i64 {
    match pos {
        SOUTH => 1,
        NORTH => 2,
        WEST => 3,
        EAST => 4,
        _ => panic!(),
    }
}
