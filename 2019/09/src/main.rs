aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(1);
    computer.output().unwrap()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(2);
    computer.output().unwrap()
}
