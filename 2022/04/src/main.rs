aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    count(input, |(a, b, c, d)| !((a > c || b < d) & (a < c || b > d)))
}

fn part_2(input: &[&str]) -> impl ToString {
    count(input, |(a, b, c, d)| a <= d && b >= c)
}

fn count(input: &[&str], pred: fn(&(u8, u8, u8, u8)) -> bool) -> usize {
    input.iter().map(parse).filter(pred).count()
}

fn parse(line: impl AsRef<str>) -> (u8, u8, u8, u8) {
    let mut values = line.as_ref().split(&[',', '-']).map(|s| s.parse().unwrap());
    (
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
    )
}
