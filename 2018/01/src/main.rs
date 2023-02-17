aoc::parts!(1, 2);

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp::Reverse;

fn part_1(input: &[&str]) -> i32 {
    input.iter().map(|line| line.parse::<i32>().unwrap()).sum()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut set = HashSet::from_iter([0]);
    let mut frequencies = Vec::with_capacity(input.len());
    frequencies.push(0);
    let mut current = 0;
    for line in input {
        current += line.parse::<i32>().unwrap();
        if !set.insert(current) {
            return current;
        }
        frequencies.push(current);
    }
    frequencies.pop();
    let modulus = current;
    let mut residue_map = HashMap::default();
    for (i, &freq) in frequencies.iter().enumerate() {
        residue_map
            .entry(freq.rem_euclid(modulus))
            .or_insert(Vec::new())
            .push((freq, i));
    }
    residue_map
        .values_mut()
        .flat_map(|freqs| {
            if modulus > 0 {
                freqs.sort_unstable_by_key(|f| f.0);
            } else {
                freqs.sort_unstable_by_key(|f| Reverse(f.0));
            }
            freqs
                .windows(2)
                .map(|pair| (pair[1].0, ((pair[1].0 - pair[0].0) / modulus, pair[0].1)))
        })
        .min_by_key(|(_, x)| *x)
        .unwrap()
        .0
}
