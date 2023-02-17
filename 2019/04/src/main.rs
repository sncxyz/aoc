aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut total = 0;
    for number in range(&input[0]) {
        let string = number.to_string();
        let mut last = 0;
        let mut valid = false;
        for digit in string.chars().map(|c| c.to_digit(10).unwrap()) {
            if digit == last {
                valid = true;
            } else if digit < last {
                valid = false;
                break;
            }
            last = digit;
        }
        if valid {
            total += 1;
        }
    }
    total
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut total = 0;
    for number in range(&input[0]) {
        let string = number.to_string();
        let mut last = 0;
        let mut valid = false;
        let mut two = false;
        let mut three = false;
        for digit in string.chars().map(|c| c.to_digit(10).unwrap()) {
            if digit == last {
                if two {
                    three = true;
                    two = false;
                } else if !three {
                    two = true;
                }
            } else {
                three = false;
                if digit < last {
                    valid = false;
                    two = false;
                    break;
                } else if two {
                    valid = true;
                }
            }
            last = digit;
        }
        if valid || two {
            total += 1;
        }
    }
    total
}

fn range(input: &str) -> impl Iterator<Item = u32> {
    let mut parts = input.split('-');
    let lower = parts.next().unwrap().parse().unwrap();
    let upper = parts.next().unwrap().parse().unwrap();
    lower..=upper
}
