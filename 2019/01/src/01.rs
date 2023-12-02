aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|mass| mass.parse::<i32>().unwrap() / 3 - 2)
        .sum()
}

fn part_2(input: &[&str]) -> i32 {
    let mut total = 0;
    for mut mass in input.iter().map(|m| m.parse::<i32>().unwrap()) {
        while mass > 0 {
            mass = mass / 3 - 2;
            total += mass.max(0);
        }
    }
    total
}
