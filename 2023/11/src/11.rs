aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    part_n(input, 2)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_n(input, 1_000_000)
}

fn part_n(input: aoc::Input, e: i64) -> i64 {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                xs.push(x as i64);
                ys.push(y as i64);
            }
        }
    }
    xs.sort_unstable();
    expand_dist(xs, e) + expand_dist(ys, e)
}

fn expand_dist(points: Vec<i64>, e: i64) -> i64 {
    let mut multiplier = 1 - points.len() as i64;
    let mut expansion = 0;
    let mut last = points[0];
    let mut dist = last * multiplier;
    for mut point in points.into_iter().skip(1) {
        point += expansion;
        if point > last + 1 {
            let increase = (point - last - 1) * (e - 1);
            point += increase;
            expansion += increase;
        }
        multiplier += 2;
        dist += point * multiplier;
        last = point;
    }
    dist
}
