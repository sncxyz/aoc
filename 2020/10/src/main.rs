aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let numbers = parse(input);
    let mut ones = 0;
    let mut threes = 0;
    for i in 0..numbers.len() - 1 {
        match numbers[i + 1] - numbers[i] {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}

fn part_2(input: &[&str]) -> impl ToString {
    const LOOKUP: [u64; 8] = [1, 1, 1, 2, 4, 7, 13, 24]; // f(n) = f(n-1) + f(n-2) + f(n-3)
    let numbers = parse(input);
    let mut total = 1;
    let mut count = 1;
    for i in 0..numbers.len() - 1 {
        if numbers[i + 1] - numbers[i] == 3 {
            total *= LOOKUP[count];
            count = 1;
        } else {
            count += 1;
        }
    }
    total
}

fn parse(input: &[&str]) -> Vec<u8> {
    let mut numbers: Vec<_> = input.iter().map(|n| n.parse().unwrap()).collect();
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers[numbers.len() - 1] + 3);
    numbers
}
