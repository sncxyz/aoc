aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut fewest = usize::MAX;
    let mut result = 0;
    for i in 0..input[0].len() / (25 * 6) {
        let (mut zeros, mut ones, mut twos) = (0, 0, 0);
        for digit in input[0][i * (25 * 6)..(i + 1) * (25 * 6)].chars() {
            match digit {
                '0' => zeros += 1,
                '1' => ones += 1,
                '2' => twos += 1,
                _ => panic!(),
            }
        }
        if zeros < fewest {
            fewest = zeros;
            result = ones * twos;
        }
    }
    result
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut image = Vec::new();
    image.resize(25 * 6, '2');
    for i in 0..input[0].len() / (25 * 6) {
        for (j, c) in image.iter_mut().enumerate() {
            if *c == '2' {
                *c = input[0].chars().nth(i * (25 * 6) + j).unwrap();
            }
        }
    }
    let mut result = String::new();
    for y in 0..6 {
        for x in 0..25 {
            result.push(if image[x + y * 25] == '1' { '#' } else { '.' });
        }
        if y < 5 {
            result.push('\n');
        }
    }
    result
}
