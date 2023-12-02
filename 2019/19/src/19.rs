aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    let mut total = 0;
    for y in 0..50 {
        for x in 0..50 {
            total += affected(&computer, x, y) as u32;
        }
    }
    total
}

fn part_2(input: &[&str]) -> i64 {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    let mut start_x = 0;
    let mut y = 0;
    loop {
        while !affected(&computer, start_x, y) {
            start_x += 1;
        }
        let mut x = start_x;
        while affected(&computer, x + 99, y) {
            if affected(&computer, x, y + 99) {
                return x * 10_000 + y;
            }
            x += 1;
        }
        y += 1;
    }
}

fn affected(computer: &intcode::Computer, x: i64, y: i64) -> bool {
    let mut computer = computer.clone();
    computer.input(x);
    computer.input(y);
    computer.output().unwrap() == 1
}
