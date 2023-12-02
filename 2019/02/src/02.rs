aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.set_direct(1, 12);
    computer.set_direct(2, 2);
    computer.run();
    computer.get_direct(0)
}

fn part_2(input: &[&str]) -> impl ToString {
    let start = intcode::Computer::new(&input[0]).unwrap();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = start.clone();
            computer.set_direct(1, noun);
            computer.set_direct(2, verb);
            computer.run();
            if computer.get_direct(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
