aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    solve_n(input, 2)
}

fn part_2(input: aoc::Input) -> impl ToString {
    solve_n(input, 1_000_000)
}

fn solve_n(input: aoc::Input, m: i64) -> i64 {
    let (mut xs, mut ys) = parse(input);
    expand(&mut xs, m);
    expand(&mut ys, m);
    total_dist(&xs) + total_dist(&ys)
}

fn parse(input: aoc::Input) -> (Vec<i64>, Vec<i64>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let (mut x, mut y) = (0, 0);
    for line in input {
        for byte in line.bytes() {
            if byte == b'#' {
                xs.push(x);
                ys.push(y);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    xs.sort_unstable();
    ys.sort_unstable();
    (xs, ys)
}

fn expand(ps: &mut [i64], m: i64) {
    let mut p = 0;
    for i in 0..ps.len() {
        if ps[i] > p + 1 {
            let gap = (ps[i] - p - 1) * (m - 1);
            for p in &mut ps[i..] {
                *p += gap;
            }
        }
        p = ps[i];
    }
}

fn total_dist(ps: &[i64]) -> i64 {
    let mut m = 1 - ps.len() as i64;
    let mut dist = 0;
    for &p in ps {
        dist += p * m;
        m += 2;
    }
    dist
}
