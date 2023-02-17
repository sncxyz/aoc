aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> String {
    let mut list = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    for _ in 0..100 {
        list = phase_full(list);
    }
    list[..8].iter().map(|d| d.to_string()).collect()
}

fn part_2(input: &[&str]) -> String {
    let mut list: Vec<u8> = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let offset = input[0][..7].parse().unwrap();
    let len = list.len() * 10000;
    list = list
        .into_iter()
        .cycle()
        .take(len)
        .skip(offset)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    for _ in 0..100 {
        list = phase(list);
    }
    list.iter().rev().take(8).map(|d| d.to_string()).collect()
}

fn phase_full(list: Vec<u8>) -> Vec<u8> {
    let mut new = Vec::new();
    const PATTERN: [i32; 4] = [0, 1, 0, -1];
    for i in 0..list.len() {
        let mut total = 0;
        for j in 0..list.len() {
            total += list[j] as i32 * PATTERN[((j + 1) / (i + 1)) % 4];
        }
        new.push((total.abs() % 10) as u8);
    }
    new
}

fn phase(list: Vec<u8>) -> Vec<u8> {
    let mut new = Vec::new();
    let mut total = 0;
    for value in list.iter() {
        total = (total + value) % 10;
        new.push(total);
    }
    new
}
