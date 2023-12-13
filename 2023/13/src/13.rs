aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    part_n(input, solve_1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_n(input, solve_2)
}

fn part_n(input: aoc::Input, solve: fn(Vec<u32>) -> Option<u32>) -> u32 {
    input
        .as_lines()
        .split(|line| line.is_empty())
        .map(parse)
        .map(|(rows, cols)| {
            solve(rows)
                .map(|r| r * 100)
                .or_else(|| solve(cols))
                .unwrap()
        })
        .sum()
}

fn parse(lines: &[&str]) -> (Vec<u32>, Vec<u32>) {
    let mut rows = Vec::new();
    let mut cols = vec![0; lines[0].len()];
    for line in lines {
        let mut row = 0;
        for (i, byte) in line.bytes().enumerate() {
            row *= 2;
            cols[i] *= 2;
            if byte == b'#' {
                row += 1;
                cols[i] += 1;
            }
        }
        rows.push(row);
    }
    (rows, cols)
}

fn solve_1(lines: Vec<u32>) -> Option<u32> {
    'outer: for i in 1..lines.len() {
        let max = i.min(lines.len() - i);
        for j in 0..max {
            if lines[i + j] != lines[i - j - 1] {
                continue 'outer;
            }
        }
        return Some(i as u32);
    }
    None
}

fn solve_2(lines: Vec<u32>) -> Option<u32> {
    'outer: for i in 1..lines.len() {
        let max = i.min(lines.len() - i);
        let mut found = false;
        for j in 0..max {
            let (a, b) = (lines[i + j], lines[i - j - 1]);
            if (a ^ b).count_ones() == 1 {
                if found {
                    continue 'outer;
                } else {
                    found = true;
                }
            } else if a != b {
                continue 'outer;
            }
        }
        if found {
            return Some(i as u32);
        }
    }
    None
}
