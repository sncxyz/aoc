aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    total(input.iter().map(|n| n.parse().unwrap()).collect())
}

fn part_2(input: &[&str]) -> impl ToString {
    total(
        input
            .iter()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>()
            .windows(3)
            .map(|depths| depths.iter().sum())
            .collect(),
    )
}

fn total(values: Vec<u16>) -> usize {
    values.windows(2).filter(|p| p[1] > p[0]).count()
}
