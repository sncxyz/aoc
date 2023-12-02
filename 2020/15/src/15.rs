aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    run(input, 2020)
}

fn part_2(input: &[&str]) -> impl ToString {
    run(input, 30_000_000)
}

fn run(input: &[&str], n: usize) -> usize {
    let mut memory = HashMap::default();
    let starting: Vec<_> = input[0].split(',').map(|n| n.parse().unwrap()).collect();
    let mut count = starting.len();
    for i in 1..count {
        memory.insert(starting[i - 1], i);
    }
    let mut last = starting[count - 1];
    while count < n {
        let new = memory.get(&last).map_or(0, |&x| count - x);
        memory.insert(last, count);
        last = new;
        count += 1;
    }
    last
}
