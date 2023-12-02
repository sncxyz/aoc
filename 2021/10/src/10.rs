aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut error_score = 0;

    for line in input {
        let mut stack = Vec::new();

        for c in line.chars() {
            let character = Character::from_char(c);
            match character.polarity {
                Polarity::Open => stack.push(character),
                Polarity::Close => {
                    if stack.pop().unwrap().variant != character.variant {
                        error_score += character.variant.error_points();
                        break;
                    }
                }
            }
        }
    }

    error_score
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut completion_scores = Vec::new();

    'outer: for line in input {
        let mut stack = Vec::new();

        for c in line.chars() {
            let character = Character::from_char(c);
            match character.polarity {
                Polarity::Open => stack.push(character),
                Polarity::Close => {
                    if stack.pop().unwrap().variant != character.variant {
                        continue 'outer;
                    }
                }
            }
        }

        let mut completion_score = 0;
        for character in stack.into_iter().rev() {
            completion_score *= 5;
            completion_score += character.variant.completion_points();
        }
        completion_scores.push(completion_score);
    }

    completion_scores.sort_unstable();
    completion_scores[completion_scores.len() / 2]
}

struct Character {
    polarity: Polarity,
    variant: Variant,
}

impl Character {
    fn from_char(c: char) -> Character {
        Character {
            polarity: Polarity::from_char(c),
            variant: Variant::from_char(c),
        }
    }
}

enum Polarity {
    Open,
    Close,
}

impl Polarity {
    fn from_char(c: char) -> Polarity {
        match c {
            '(' | '[' | '{' | '<' => Polarity::Open,
            _ => Polarity::Close,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Variant {
    Round,
    Square,
    Curly,
    Angle,
}

impl Variant {
    fn from_char(c: char) -> Variant {
        match c {
            '(' | ')' => Variant::Round,
            '[' | ']' => Variant::Square,
            '{' | '}' => Variant::Curly,
            '<' | '>' => Variant::Angle,
            _ => panic!(),
        }
    }

    fn error_points(&self) -> u64 {
        match self {
            Variant::Round => 3,
            Variant::Square => 57,
            Variant::Curly => 1197,
            Variant::Angle => 25137,
        }
    }

    fn completion_points(&self) -> u64 {
        match self {
            Variant::Round => 1,
            Variant::Square => 2,
            Variant::Curly => 3,
            Variant::Angle => 4,
        }
    }
}
