use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> u32 {
    input[0].split(',').map(hash).sum()
}

fn part_2(input: aoc::Input) -> u32 {
    let mut boxes: [Vec<_>; 256] = [(); 256].map(|_| Vec::new());
    for string in input[0].split(',') {
        let i = string.find(['=', '-']).unwrap();
        let label = &string[..i];
        let h = hash(label) as usize;
        match string.idx(i) {
            b'=' => {
                let focal = string.idx(i + 1) - b'0';
                let mut added = false;
                for (l, f) in &mut boxes[h] {
                    if *l == label {
                        *f = focal;
                        added = true;
                        break;
                    }
                }
                if !added {
                    boxes[h].push((label, focal));
                }
            }
            b'-' => boxes[h].retain(|(l, _)| *l != label),
            _ => unreachable!(),
        }
    }
    let mut total = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (j, (_, f)) in b.into_iter().enumerate() {
            total += (i as u32 + 1) * (j as u32 + 1) * f as u32;
        }
    }
    total
}

fn hash(string: &str) -> u32 {
    let mut current: u8 = 0;
    for byte in string.bytes() {
        current = current.wrapping_add(byte);
        current = current.wrapping_mul(17);
    }
    current as u32
}
