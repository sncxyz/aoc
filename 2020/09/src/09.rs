aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    first_invalid(&input.iter().map(|n| n.parse().unwrap()).collect::<Vec<_>>())
}

fn part_2(input: &[&str]) -> impl ToString {
    let numbers: Vec<_> = input.iter().map(|n| n.parse().unwrap()).collect();
    let target = first_invalid(&numbers);
    let (mut i, mut j) = (0, 1);
    let mut sum = numbers[i] + numbers[j];
    loop {
        while sum < target {
            j += 1;
            sum += numbers[j];
        }
        if sum == target {
            break;
        }
        sum -= numbers[i];
        i += 1;
        while sum > target {
            sum -= numbers[j];
            j -= 1;
        }
    }
    numbers[i..=j].iter().min().unwrap() + numbers[i..=j].iter().max().unwrap()
}

fn first_invalid(numbers: &[u64]) -> u64 {
    for i in 25..numbers.len() {
        if !is_valid(&numbers[i - 25..i], numbers[i]) {
            return numbers[i];
        }
    }
    0
}

fn is_valid(prev: &[u64], next: u64) -> bool {
    for i in 0..24 {
        for j in i + 1..25 {
            if prev[i] + prev[j] == next {
                return true;
            }
        }
    }
    false
}
