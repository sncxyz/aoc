aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut crabs: Vec<_> = input[0].split(',').map(|n| n.parse().unwrap()).collect();
    crabs.sort_unstable();
    fuel_one(&crabs, crabs[(crabs.len() - 1) / 2])
}

fn part_2(input: &[&str]) -> impl ToString {
    let crabs: Vec<_> = input[0].split(',').map(|n| n.parse().unwrap()).collect();
    let (mut low, mut high) = (*crabs.iter().min().unwrap(), *crabs.iter().max().unwrap());
    loop {
        let position = (low + high) / 2;
        let fuel = fuel_two(&crabs, position);
        if fuel > fuel_two(&crabs, position - 1) {
            high = position;
        } else if fuel > fuel_two(&crabs, position + 1) {
            low = position;
        } else {
            return fuel;
        }
    }
}

fn fuel_one(crabs: &[u32], position: u32) -> u32 {
    crabs
        .iter()
        .map(|&x| {
            if x > position {
                x - position
            } else {
                position - x
            }
        })
        .sum::<u32>()
}

fn fuel_two(crabs: &[u32], position: u32) -> u32 {
    crabs
        .iter()
        .map(|&x| {
            let diff = if x > position {
                x - position
            } else {
                position - x
            };
            (diff * (diff + 1)) / 2
        })
        .sum::<u32>()
}
