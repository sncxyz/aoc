aoc::parts!(1, 2);

use fxhash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .map(|d| d.len())
                .filter(|&c| c != 5 && c != 6)
                .count()
        })
        .sum()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut lookup: HashMap<u8, &str> = HashMap::default();
    lookup.insert(0b1110111, "0");
    lookup.insert(0b0010010, "1");
    lookup.insert(0b1011101, "2");
    lookup.insert(0b1011011, "3");
    lookup.insert(0b0111010, "4");
    lookup.insert(0b1101011, "5");
    lookup.insert(0b1101111, "6");
    lookup.insert(0b1010010, "7");
    lookup.insert(0b1111111, "8");
    lookup.insert(0b1111011, "9");

    let mut total: u32 = 0;
    for line in input {
        let mut parts = line.split(" | ");
        let mut frequencies = HashMap::default();
        for digit in parts.next().unwrap().split(' ') {
            for segment in digit.chars() {
                let count = frequencies.entry(segment).or_insert(0u8);
                *count += match digit.len() {
                    2 => 2,
                    4 => 4,
                    _ => 1,
                };
            }
        }

        let mut segments = HashMap::default();
        for (&letter, &count) in frequencies.iter() {
            segments.insert(
                letter,
                1 << match count {
                    8 => 6,
                    9 => 5,
                    12 => 4,
                    10 => 3,
                    4 => 2,
                    13 => 1,
                    7 => 0,
                    _ => panic!(),
                },
            );
        }

        let mut value = String::new();
        for digit in parts.next().unwrap().split(' ') {
            value += lookup
                .get(
                    &digit
                        .chars()
                        .map(|letter| segments.get(&letter).unwrap())
                        .sum::<u8>(),
                )
                .unwrap();
        }
        total += &value.parse().unwrap();
    }
    total
}
