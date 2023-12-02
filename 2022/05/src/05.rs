use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    solve(input, |d| d.rev().collect())
}

fn part_2(input: Input) -> impl ToString {
    solve(input, |d| d.collect())
}

fn solve(input: Input, f: fn(std::vec::Drain<u8>) -> Vec<u8>) -> String {
    let mut stacks = vec![Vec::new(); (input[0].len() + 1) / 4];
    let cut = input.lines().position(str::is_empty).unwrap();
    for line in input.lines().take(cut - 1).map(str::bytes).rev() {
        for (i, b) in line.skip(1).step_by(4).enumerate().filter(|x| x.1 != 32) {
            stacks[i].push(b);
        }
    }
    for line in input.lines().skip(cut + 1).map(|l| l.split(' ').skip(1)) {
        let v: Vec<usize> = line.step_by(2).map(|x| x.parse().unwrap()).collect();
        let i = stacks[v[1] - 1].len() - v[0];
        let crates = f(stacks[v[1] - 1].drain(i..));
        stacks[v[2] - 1].extend(crates);
    }
    stacks.iter().map(|s| *s.last().unwrap() as char).collect()
}
