aoc::parts!(1);

use std::io;

fn part_1(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    while computer.state() != intcode::State::Halted {
        while let Some(value) = computer.output() {
            print!("{}", (value as u8) as char);
        }
        if computer.state() == intcode::State::Input {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            for c in line.trim().chars() {
                computer.input((c as u8) as i64);
            }
            computer.input(10);
        }
    }
    "Finished."
}
