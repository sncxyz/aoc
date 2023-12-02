aoc::parts!(1, 2);

use rustc_hash::FxHashSet as HashSet;

pub fn part_1(input: &[&str]) -> impl ToString {
    let mut frequencies = Vec::new();
    frequencies.resize(input[0].len(), 0);
    for number in input {
        for (i, digit) in number.chars().enumerate() {
            frequencies[i] += if digit == '1' { 1 } else { -1 };
        }
    }
    let gamma = frequencies
        .iter()
        .fold(0, |acc, &freq| (acc << 1) + (freq > 0) as i32);
    gamma * ((1 << frequencies.len()) - gamma - 1)
}

pub fn part_2(input: &[&str]) -> impl ToString {
    let len = input[0].len() as u16;
    let numbers: HashSet<u16> = input
        .iter()
        .map(|number| {
            number.chars().fold(0, |acc, digit| {
                (acc << 1) + digit.to_digit(10).unwrap() as u16
            })
        })
        .collect();
    cull(numbers.clone(), len, false) as u32 * cull(numbers, len, true) as u32
}

fn cull(mut numbers: HashSet<u16>, len: u16, mode: bool) -> u16 {
    let mut i = len;
    while numbers.len() > 1 {
        i -= 1;
        let mut freq = 0;
        for &number in numbers.iter() {
            freq += if (number >> i) & 1 == 1 { 1 } else { -1 };
        }
        let value = !((freq < 0) ^ mode) as u16;
        numbers.retain(|number| (*number >> i) & 1 == value);
    }
    numbers.into_iter().next().unwrap()
}
