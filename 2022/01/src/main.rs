use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    parse(input).max().unwrap()
}

fn part_2(input: Input) -> impl ToString {
    let [a, b, c] = parse(input).fold([0, 0, 0], |[a, b, c], x| {
        [x.max(a), x.clamp(b, a), x.clamp(c, b)]
    });
    a + b + c
}

fn parse<'a>(input: Input<'a>) -> impl Iterator<Item = u32> + 'a {
    input
        .as_lines()
        .split(|line| line.is_empty())
        .map(|elf| elf.iter().map(Parse::parse_uw::<u32>).sum())
}
