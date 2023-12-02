aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::default();
    let mut and: u64 = 0;
    let mut or: u64 = 0;
    for line in input {
        if &line[0..4] == "mask" {
            and = 0;
            or = 0;
            for (i, digit) in line[7..].chars().enumerate() {
                match digit {
                    'X' => and |= 1 << (35 - i),
                    '1' => or |= 1 << (35 - i),
                    _ => (),
                }
            }
        } else {
            let (address, value) = parse(line);
            memory.insert(address, (value & and) | or);
        }
    }
    memory.values().sum()
}

fn part_2(input: &[&str]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::default();
    let mut mask = "";
    for line in input {
        if &line[0..4] == "mask" {
            mask = &line[7..];
        } else {
            let (address, value) = parse(line);
            set_memory(&mut memory, mask, address, value);
        }
    }
    memory.values().sum()
}

fn parse(line: &str) -> (u64, u64) {
    (
        line[line.find('[').unwrap() + 1..line.find(']').unwrap()]
            .parse()
            .unwrap(),
        line[line.find('=').unwrap() + 2..].parse().unwrap(),
    )
}

fn set_memory(memory: &mut HashMap<u64, u64>, mask: &str, address: u64, value: u64) {
    if mask.is_empty() {
        memory.insert(address, value);
    } else {
        let len = mask.len() - 1;
        match &mask[0..1] {
            "1" => set_memory(memory, &mask[1..], address | 1 << len, value),
            "0" => set_memory(memory, &mask[1..], address, value),
            "X" => {
                set_memory(memory, &mask[1..], address | 1 << len, value);
                set_memory(memory, &mask[1..], address & !(1 << len), value);
            }
            _ => panic!(),
        }
    }
}
