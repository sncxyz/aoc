use aoc::{Input, Parse, Parser};

aoc::parts!(1, 2);

fn part_1(input: Input) -> u32 {
    part_n(input, |(id, (r, g, b))| {
        (r <= 12 && g <= 13 && b <= 14) as u32 * id
    })
}

fn part_2(input: Input) -> u32 {
    part_n(input, |(_, (r, g, b))| r * g * b)
}

fn part_n(input: Input, f: fn((u32, (u32, u32, u32))) -> u32) -> u32 {
    input.lines().map(Parse::as_parser).map(parse).map(f).sum()
}

fn parse(mut parser: Parser) -> (u32, (u32, u32, u32)) {
    (
        parser.between("Game ", ": ").parse_uw(),
        parser
            .rest()
            .split("; ")
            .flat_map(|p| p.split(", "))
            .map(Parse::as_parser)
            .map(|mut p| (p.before(" ").parse_uw(), p.rest()))
            .fold((0, 0, 0), |(r, g, b), (count, colour)| match colour {
                "red" => (r.max(count), g, b),
                "green" => (r, g.max(count), b),
                "blue" => (r, g, b.max(count)),
                _ => unreachable!(),
            }),
    )
}
