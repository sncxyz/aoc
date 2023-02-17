aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    parse(input).max().unwrap()
}

fn part_2(input: &[&str]) -> impl ToString {
    let [a, b, c] = parse(input).fold([0, 0, 0], |[a, b, c], x| {
        [x.max(a), x.clamp(b, a), x.clamp(c, b)]
    });
    a + b + c
}

fn parse<'a>(input: &'a [&str]) -> impl Iterator<Item = u32> + 'a {
    input
        .split(|line| line.is_empty())
        .map(|elf| elf.iter().map(|c| c.parse::<u32>().unwrap()).sum())
}
