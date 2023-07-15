use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> u32 {
    input
        .lines()
        .map(|r| (r.bytes().take(r.len() / 2), r.bytes().skip(r.len() / 2)))
        .map(|(f, s)| intersect(unique(f), s))
        .sum()
}

fn part_2(input: Input) -> u32 {
    input
        .as_lines()
        .chunks(3)
        .map(|g| intersect(unique(g[0].bytes()) & unique(g[1].bytes()), g[2].bytes()))
        .sum()
}

fn unique(items: impl Iterator<Item = u8>) -> u64 {
    items.fold(0, |u, i| u | (1 << prio(i)))
}

fn intersect(u: u64, items: impl Iterator<Item = u8>) -> u32 {
    items.map(prio).find(|p| u & (1 << p) != 0).unwrap() + 1
}

#[inline(never)]
fn prio(item: u8) -> u32 {
    (item - if item < 95 { 39 } else { 97 }) as u32
}
