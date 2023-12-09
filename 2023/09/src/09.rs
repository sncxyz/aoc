use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> i32 {
    input.lines().map(forwards).sum()
}

fn part_2(input: aoc::Input) -> i32 {
    input.lines().map(backwards).sum()
}

fn forwards(line: &str) -> i32 {
    let mut values: Vec<i32> = line.ints_iter().collect();
    for limit in (0..values.len()).rev() {
        for i in 0..limit {
            values[i] = values[i + 1] - values[i];
        }
    }
    values.into_iter().sum()
}

fn backwards(line: &str) -> i32 {
    let mut values: Vec<i32> = line.ints_iter().collect();
    for limit in 1..values.len() {
        for i in (limit..values.len()).rev() {
            values[i] -= values[i - 1];
        }
    }
    values.into_iter().rev().fold(0, |sum, x| x - sum)
}
