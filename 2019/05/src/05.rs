aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(1);
    let mut last = 0;
    while let Some(value) = computer.output() {
        last = value;
    }
    last
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(5);
    computer.output().unwrap()
}
