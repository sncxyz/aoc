use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    score(input, |y, o| (4 * y + 6 * o) % 9 + 1)
}

fn part_2(input: Input) -> impl ToString {
    score(input, |y, o| (y + o) % 3 + 3 * y + 1)
}

fn score(input: Input, f: fn(u32, u32) -> u32) -> u32 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|b| f(b[2] as u32 - 88, b[0] as u32 - 63))
        .sum()
}
