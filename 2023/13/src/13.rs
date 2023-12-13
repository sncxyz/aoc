aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    part_n(input, solve_1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    part_n(input, solve_2)
}

fn part_n(input: aoc::Input, solve: fn(u32, (Vec<u32>, Vec<u32>)) -> u32) -> u32 {
    input
        .as_lines()
        .split(|line| line.is_empty())
        .map(parse)
        .fold(0, solve)
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

fn solve_1(total: u32, (rows, cols): (Vec<u32>, Vec<u32>)) -> u32 {
    total
        + get_reflection(&rows, None)
            .map(|r| r * 100)
            .or_else(|| get_reflection(&cols, None))
            .unwrap()
}

fn solve_2(total: u32, (mut rows, mut cols): (Vec<u32>, Vec<u32>)) -> u32 {
    let prev_row = get_reflection(&rows, None);
    let prev_col = get_reflection(&cols, None);

    for row in 0..rows.len() {
        for col in 0..cols.len() {
            rows[row] ^= 1 << col;
            cols[col] ^= 1 << row;
            if let Some(r) = get_reflection(&rows, prev_row) {
                return total + 100 * r;
            } else if let Some(r) = get_reflection(&cols, prev_col) {
                return total + r;
            }
            rows[row] ^= 1 << col;
            cols[col] ^= 1 << row;
        }
    }

    unreachable!()
}

fn get_reflection(lines: &[u32], ignore: Option<u32>) -> Option<u32> {
    for i in 1..lines.len() {
        let max = i.min(lines.len() - i);
        let mut is_reflection = true;
        for j in 0..max {
            if lines[i + j] != lines[i - j - 1] {
                is_reflection = false;
            }
        }
        if is_reflection {
            let r = Some(i as u32);
            if r != ignore {
                return r;
            }
        }
    }
    None
}
