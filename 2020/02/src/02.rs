aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut total = 0;
    for line in input {
        let password = Password::from_str(line);
        let count = password
            .password
            .chars()
            .filter(|a| *a == password.letter)
            .count();
        if count >= password.min && count <= password.max {
            total += 1;
        }
    }
    total
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut total = 0;
    for line in input {
        let password = Password::from_str(line);
        if (password.password.chars().nth(password.min - 1).unwrap() == password.letter)
            ^ (password.password.chars().nth(password.max - 1).unwrap() == password.letter)
        {
            total += 1;
        }
    }
    total
}

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn from_str(line: &str) -> Password {
        let mut parts = line.split(' ');
        let mut values = parts.next().unwrap().split('-');
        let min = values.next().unwrap().parse().unwrap();
        let max = values.next().unwrap().parse().unwrap();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap().to_string();
        Password {
            min,
            max,
            letter,
            password,
        }
    }
}
