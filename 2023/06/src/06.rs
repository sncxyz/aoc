use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> u64 {
    input[0][11..]
        .ints_iter()
        .zip(input[1][11..].ints_iter())
        .map(range)
        .product()
}

fn part_2(input: aoc::Input) -> u64 {
    range((parse_2(input[0]), parse_2(input[1])))
}

#[inline(always)]
fn parse_2(line: &str) -> u64 {
    line[11..]
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0, |x, d| x * 10 + d as u64)
}

fn range((time, dist): (u64, u64)) -> u64 {
    let mut min = 1;
    let mut max = time / 2;
    while min < max {
        let mid = (min + max) / 2;
        if mid * (time - mid) <= dist {
            min = mid + 1;
        } else {
            max = mid;
        }
    }
    time - 2 * min + 1
}
