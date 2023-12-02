use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    count(input, |[a, b, c, d]| !((a > c || b < d) & (a < c || b > d)))
}

fn part_2(input: Input) -> impl ToString {
    count(input, |[a, b, c, d]| a <= d && b >= c)
}

fn count(input: Input, pred: fn(&[u8; 4]) -> bool) -> usize {
    input.lines().map(Parse::uints).filter(pred).count()
}
