aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    score(input, |y, o| (4 * y + 6 * o) % 9 + 1)
}

fn part_2(input: &[&str]) -> impl ToString {
    score(input, |y, o| (y + o) % 3 + 3 * y + 1)
}

fn score(input: &[&str], f: fn(u32, u32) -> u32) -> u32 {
    input
        .iter()
        .map(|line| line.as_bytes())
        .map(|b| f(b[2] as u32 - 88, b[0] as u32 - 63))
        .sum()
}
