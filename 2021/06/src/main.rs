aoc::parts!(1, 2);

use fxhash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    simulate(input, 80)
}

fn part_2(input: &[&str]) -> impl ToString {
    simulate(input, 256)
}

fn parse(input: &[&str]) -> [u64; 6] {
    let mut frequencies = [0; 6];
    for age in input[0].split(',') {
        frequencies[age.parse::<usize>().unwrap()] += 1;
    }
    frequencies
}

fn simulate(input: &[&str], days: u16) -> u64 {
    let frequencies = parse(input);
    (0..6)
        .map(|age| frequencies[age as usize] * fish(days + 6 - age, &mut HashMap::default()))
        .sum()
}

fn fish(days: u16, cache: &mut HashMap<u16, u64>) -> u64 {
    if days < 9 {
        return (days / 7 + 1) as u64;
    }
    if !cache.contains_key(&days) {
        let population = fish(days - 7, cache) + fish(days - 9, cache);
        cache.insert(days, population);
    }
    *cache.get(&days).unwrap()
}
