aoc::parts!(1, 2);

use fxhash::FxHashSet as HashSet;

fn part_1(input: &[&str]) -> impl ToString {
    let values: Vec<_> = input.iter().map(|v| v.parse().unwrap()).collect();
    if let Some(product) = find_pair(&values, 2020) {
        return product;
    }
    0
}

fn part_2(input: &[&str]) -> impl ToString {
    let values: Vec<_> = input.iter().map(|v| v.parse().unwrap()).collect();
    for (i, &value) in values.iter().enumerate() {
        if let Some(product) = find_pair(&values[i + 1..], 2020 - value) {
            return product * value;
        }
    }
    0
}

fn find_pair(input: &[i32], sum: i32) -> Option<i32> {
    let mut values = HashSet::default();
    for value in input {
        if values.contains(value) {
            return Some(value * (sum - value));
        } else {
            values.insert(sum - value);
        }
    }
    None
}
