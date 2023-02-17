aoc::parts!(1);

fn part_1(input: &[&str]) -> impl ToString {
    let subject: u64 = input[0].parse().unwrap();
    let mut n = 1;
    for _ in 0..loop_size(input[1].parse().unwrap()) {
        n = (n * subject) % 20201227;
    }
    n
}

fn loop_size(key: u64) -> u64 {
    let mut n = 1;
    let mut loop_size = 0;
    while n != key {
        n = (n * 7) % 20201227;
        loop_size += 1;
    }
    loop_size
}
