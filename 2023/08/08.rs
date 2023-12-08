use std::collections::{hash_map::Entry, HashMap};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let elements = parse(input);
    let mut current = [b'A', b'A', b'A'];
    for (i, instr) in input[0].bytes().cycle().enumerate() {
        match instr {
            b'L' => current = elements[&current].0,
            b'R' => current = elements[&current].1,
            _ => unreachable!(),
        }
        if current == [b'Z', b'Z', b'Z'] {
            return i as u64 + 1;
        }
    }
    0
}

fn part_2(input: aoc::Input) -> impl ToString {
    let elements = parse(input);
    println!("{}", input[0].len());
    let mut info = Vec::new();
    for element in elements.keys().copied() {
        if element[2] == b'A' {
            info.push(run_2(input[0].as_bytes(), &elements, element));
        }
    }
    for info in &info {
        println!("{info:?}");
    }
    for info in &info {
        debug_assert!(info.z_offsets.len() == 1);
        debug_assert!(info.z_offsets[0] == info.cycle_len);
    }
    let info: Vec<_> = info
        .into_iter()
        .map(|info| (info.offset, info.cycle_len / input[0].len() as u64))
        .collect();
    println!("{info:?}");
    // let x = crt(&info);
    // x * input[0].len() as u64
    info.into_iter().map(|x| x.1).product::<u64>() * input[0].len() as u64
}

fn run_2(
    instrs: &[u8],
    elements: &HashMap<[u8; 3], ([u8; 3], [u8; 3])>,
    mut element: [u8; 3],
) -> Info {
    let mut steps = 0;
    let mut instr = 0;
    let mut map = HashMap::from([((element, instr as u64), steps)]);
    let mut z_offsets = Vec::new();
    loop {
        steps += 1;
        match instrs[instr] {
            b'L' => element = elements[&element].0,
            b'R' => element = elements[&element].1,
            _ => unreachable!(),
        }
        instr = (instr + 1) % instrs.len();
        if element[2] == b'Z' {
            println!("{instr}");
            z_offsets.push(steps);
        }
        match map.entry((element, instr as u64)) {
            Entry::Vacant(x) => {
                x.insert(steps);
            }
            Entry::Occupied(x) => {
                let offset = *x.get();
                let cycle_len = steps - offset;
                return Info {
                    offset,
                    cycle_len,
                    z_offsets,
                };
            }
        }
    }
}

/// pairs = &[(offset, cycle)]
fn crt(pairs: &[(u64, u64)]) -> u64 {
    let mut increment = pairs[0].1;
    let mut x = pairs[0].0;
    for &(offset, cycle) in &pairs[1..] {
        while x % cycle != offset {
            x += increment;
        }
        increment *= cycle;
    }
    x
}

fn parse(input: aoc::Input) -> HashMap<[u8; 3], ([u8; 3], [u8; 3])> {
    let mut elements = HashMap::new();
    for line in input[2..].iter().copied().map(str::as_bytes) {
        let x = [line[0], line[1], line[2]];
        let l = [line[7], line[8], line[9]];
        let r = [line[12], line[13], line[14]];
        elements.insert(x, (l, r));
    }
    elements
}

#[derive(Debug)]
struct Info {
    offset: u64,
    cycle_len: u64,
    z_offsets: Vec<u64>,
}
