aoc::parts!(1, 2);

use grid::{constants::*, v, Grid};

fn part_1(input: &[&str]) -> impl ToString {
    let view = parse(&input[0]);
    let mut total = 0;
    for y in 1..view.height() - 1 {
        for x in 1..view.width() - 1 {
            let v = v!(x, y);
            if ORTHOGONAL_ZERO.into_iter().all(|off| view[v + off] == '#') {
                total += x * y;
            }
        }
    }
    total
}

fn part_2(input: &[&str]) -> i64 {
    let mut main_routine = get_instructions(parse(&input[0]));

    let functions = vec![
        extract_function(&mut main_routine, 'A'),
        extract_function(&mut main_routine, 'B'),
        extract_function(&mut main_routine, 'C'),
    ];

    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.set_direct(0, 2);
    computer.run();

    while computer.output().is_some() {}

    for (i, part) in main_routine.iter().enumerate() {
        if let Part::Function(letter) = part {
            computer.input((*letter as u8) as i64);
            computer.input(if i == main_routine.len() - 1 { 10 } else { 44 });
        }
    }

    for function in functions.iter() {
        while computer.output().is_some() {}
        for (i, part) in function.iter().enumerate() {
            if let Part::Instruction(instruction) = part {
                computer.input(
                    (format!("{:?}", instruction.turn).chars().next().unwrap() as u8) as i64,
                );
                computer.input(44);
                for digit in instruction.distance.to_string().chars() {
                    computer.input((digit as u8) as i64);
                }
                computer.input(if i == function.len() - 1 { 10 } else { 44 });
            }
        }
    }

    while computer.output().is_some() {}

    computer.input(b'n' as i64);
    computer.input(10);

    let mut dust = 0;
    while let Some(output) = computer.output() {
        dust = output;
    }
    dust
}

fn parse(input: &str) -> Grid<char> {
    let mut computer = intcode::Computer::new(input).unwrap();
    computer.run();
    let mut view = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut x = 0;
    while let Some(value) = computer.output() {
        match value {
            10 => {
                if x > 1 {
                    height += 1;
                }
                if width == 0 {
                    width = x;
                }
                x = 0;
            }
            c => view.push((c as u8) as char),
        }
        x += 1;
    }
    Grid::from_iter(width, height, view.into_iter())
}

fn get_instructions(view: Grid<char>) -> Vec<Part> {
    let mut position = ZERO;
    let mut direction = ZERO;
    for (pos, &value) in view.iter_positions() {
        if value != '#' && value != '.' {
            position = pos;
            direction = match value {
                '>' => EAST,
                '^' => NORTH,
                '<' => WEST,
                'v' => SOUTH,
                _ => panic!(),
            };
            break;
        }
    }

    let mut instructions = Vec::new();
    loop {
        let turn;
        let rotated = direction.perp();
        if view.get(position + rotated) == Some(&'#') {
            turn = Turn::R;
            direction = rotated;
        } else if view.get(position - rotated) == Some(&'#') {
            turn = Turn::L;
            direction = -rotated;
        } else {
            break;
        }

        let mut distance = 0;
        while view.get(position + direction) == Some(&'#') {
            position += direction;
            distance += 1;
        }

        instructions.push(Part::Instruction(Instruction { turn, distance }));
    }

    instructions
}

fn extract_function(main_routine: &mut Vec<Part>, letter: char) -> Vec<Part> {
    let mut i = 0;
    while let Part::Function(_) = main_routine[i] {
        i += 1;
    }
    let mut size = 2;
    let mut highest_coverage = 0;
    let mut indices = Vec::new();
    while let Some(Part::Instruction(_)) = main_routine.get(i + size) {
        size += 1;
        let current_indices: Vec<usize> = main_routine
            .windows(size)
            .enumerate()
            .filter(|&(_, f)| f == &main_routine[i..i + size])
            .map(|(i, _)| i)
            .rev()
            .collect();
        let current_coverage = current_indices.len() * size;
        if current_coverage > highest_coverage {
            highest_coverage = current_coverage;
            indices = current_indices;
        } else {
            size -= 1;
            break;
        }
    }

    let function = main_routine[i..i + size].to_vec();

    for i in indices.into_iter() {
        main_routine.drain(i + 1..i + size);
        main_routine[i] = Part::Function(letter);
    }

    function
}

#[derive(PartialEq, Eq, Clone)]
enum Part {
    Function(char),
    Instruction(Instruction),
}

#[derive(PartialEq, Eq, Clone)]
struct Instruction {
    turn: Turn,
    distance: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Turn {
    R,
    L,
}
