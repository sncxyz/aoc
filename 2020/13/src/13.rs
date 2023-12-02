aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let timestamp: u32 = input[0].parse().unwrap();
    let mut earliest = (0, u32::MAX);
    for id in input[1].split(',').filter(|id| *id != "x") {
        let id = id.parse().unwrap();
        let wait = id - (timestamp % id);
        if wait < earliest.1 {
            earliest = (id, wait);
        }
    }
    earliest.0 * earliest.1
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut timestamp = 0;
    let mut step = 1;
    for (offset, id) in input[1].split(',').enumerate() {
        if let Ok(id) = id.parse::<usize>() {
            while (timestamp + offset) % id != 0 {
                timestamp += step;
            }
            step *= id;
        }
    }
    timestamp
}
