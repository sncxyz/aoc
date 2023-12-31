use aoc::Input;

aoc::parts!(1, 2);

const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part_1(input: Input) -> u32 {
    input
        .lines()
        .map(|line| find_1(line.chars()) * 10 + find_1(line.chars().rev()))
        .sum()
}

fn find_1(mut iter: impl Iterator<Item = char>) -> u32 {
    iter.find_map(|c| c.to_digit(10)).unwrap()
}

fn part_2(input: Input) -> u32 {
    input
        .lines()
        .map(|line| {
            find_2(line, line.char_indices()) * 10 + find_2(line, line.char_indices().rev())
        })
        .sum()
}

fn find_2(line: &str, mut iter: impl Iterator<Item = (usize, char)>) -> u32 {
    iter.find_map(|(i, c)| {
        c.to_digit(10).or_else(|| {
            DIGITS
                .iter()
                .enumerate()
                .find_map(|(j, d)| line[i..].starts_with(d).then_some(j as u32 + 1))
        })
    })
    .unwrap()
}
