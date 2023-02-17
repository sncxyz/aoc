aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    run(&parse(input), -1).1
}

fn part_2(input: &[&str]) -> impl ToString {
    let boot_code = parse(input);
    for i in 0..boot_code.len() {
        if boot_code[i].op != "acc" {
            let (end, acc) = run(&boot_code, i as i32);
            if end {
                return acc;
            }
        }
    }
    0
}

fn parse<'a>(input: &'a [&str]) -> Vec<Instr<'a>> {
    input.iter().map(Instr::new).collect()
}

fn run(boot_code: &[Instr], alter: i32) -> (bool, i32) {
    let mut i = 0;
    let mut acc = 0;
    let mut executed = vec![false; boot_code.len()];
    loop {
        if executed[i as usize] {
            break (false, acc);
        }
        executed[i as usize] = true;
        let instr = &boot_code[i as usize];
        if instr.op == "acc" {
            acc += instr.arg;
        } else if (instr.op == "jmp") ^ (alter == i) {
            i += instr.arg - 1;
        }
        i += 1;
        if i as usize == boot_code.len() {
            break (true, acc);
        }
    }
}

struct Instr<'a> {
    op: &'a str,
    arg: i32,
}

impl<'a> Instr<'a> {
    fn new<T: AsRef<str>>(line: &'a T) -> Self {
        let mut parts = line.as_ref().split(' ');
        Self {
            op: parts.next().unwrap(),
            arg: parts.next().unwrap().parse().unwrap(),
        }
    }
}
