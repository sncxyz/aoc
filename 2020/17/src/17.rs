aoc::parts!(1, 2);

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn part_1(input: &[&str]) -> impl ToString {
    run(input, &[0])
}

fn part_2(input: &[&str]) -> impl ToString {
    run(input, &[-1, 0, 1])
}

fn run(input: &[&str], w_offsets: &[i32]) -> usize {
    let mut state = HashSet::default();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                state.insert((x as i32, y as i32, 0, 0));
            }
        }
    }
    for _ in 0..6 {
        let mut neighbours = HashMap::default();
        for &(x, y, z, w) in &state {
            for &dw in w_offsets {
                for dz in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        for dx in [-1, 0, 1] {
                            if (dx, dy, dz, dw) != (0, 0, 0, 0) {
                                *neighbours
                                    .entry((x + dx, y + dy, z + dz, w + dw))
                                    .or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
            neighbours.entry((x, y, z, w)).or_insert(0);
        }
        for (pos, count) in neighbours {
            if state.contains(&pos) {
                if count != 2 && count != 3 {
                    state.remove(&pos);
                }
            } else if count == 3 {
                state.insert(pos);
            }
        }
    }
    state.len()
}
