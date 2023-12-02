aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    let mut total = 0;
    while computer.state() != intcode::State::Halted {
        computer.output().unwrap();
        computer.output().unwrap();
        if computer.output().unwrap() == 2 {
            total += 1;
        }
    }
    total
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.set_direct(0, 2);
    computer.run();
    let mut ball = 0;
    let mut paddle = 0;
    let mut score = 0;
    while computer.state() != intcode::State::Halted {
        while computer.state() == intcode::State::Output {
            let x = computer.output().unwrap();
            computer.output().unwrap();
            match computer.output().unwrap() {
                s if x == -1 => score = s,
                3 => paddle = x,
                4 => ball = x,
                _ => (),
            }
        }
        computer.input(if ball - paddle > 0 {
            1
        } else if ball == paddle {
            0
        } else {
            -1
        });
    }
    score
}
