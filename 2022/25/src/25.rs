aoc::parts!(1);

fn part_1(input: &[&str]) -> impl ToString {
    let mut total: i64 = 0;
    for line in input {
        let mut n = 0;
        for b in line.bytes() {
            n *= 5;
            n += match b {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                _ => panic!(),
            };
        }
        total += n;
    }
    if total == 0 {
        return String::from("0");
    }
    let mut answer = Vec::new();
    while total != 0 {
        total += 2;
        let rem = total.rem_euclid(5);
        answer.push(match rem {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!(),
        });
        total = total.div_euclid(5);
    }
    answer.into_iter().rev().collect()
}
