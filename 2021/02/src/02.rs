aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let (mut position, mut depth) = (0, 0);
    for (direction, distance) in parse(input).into_iter() {
        match direction {
            "forward" => position += distance,
            "up" => depth -= distance,
            "down" => depth += distance,
            _ => panic!(),
        }
    }
    position * depth
}

fn part_2(input: &[&str]) -> impl ToString {
    let (mut position, mut depth, mut aim) = (0, 0, 0);
    for (direction, distance) in parse(input).into_iter() {
        match direction {
            "forward" => {
                position += distance;
                depth += aim * distance;
            }
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => panic!(),
        }
    }
    position * depth
}

fn parse<'a>(input: &[&'a str]) -> Vec<(&'a str, i32)> {
    input
        .iter()
        .map(|step| {
            let mut parts = step.split(' ');
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}
