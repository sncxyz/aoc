use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> u32 {
    parse(input).map(|n| 2u32.pow(n as u32) / 2).sum()
}

fn part_2(input: aoc::Input) -> u32 {
    let mut copies = vec![1; input.len()];
    for (i, n) in parse(input).enumerate() {
        for j in i + 1..=i + n {
            copies[j] += copies[i];
        }
    }
    copies.into_iter().sum()
}

fn parse(input: aoc::Input) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .map(Parse::as_parser)
        .map(|mut p| {
            (
                p.between(":", "|").uints_iter().collect::<Vec<u32>>(),
                p.rest().uints_iter(),
            )
        })
        .map(|(winning, yours)| yours.filter(|n| winning.contains(n)).count())
}
