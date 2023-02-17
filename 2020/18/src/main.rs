aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u64 {
    input.iter().map(eval_one).sum()
}

fn part_2(input: &[&str]) -> u64 {
    input.iter().map(eval_two).sum()
}

fn eval_one(expr: impl AsRef<str>) -> u64 {
    let mut acc = vec![(0, false)];
    let mut i = 0;
    for c in expr.as_ref().chars() {
        match c {
            '(' => {
                acc.push((0, false));
                i += 1;
            }
            ')' => {
                match acc[i - 1].1 {
                    false => acc[i - 1].0 += acc[i].0,
                    true => acc[i - 1].0 *= acc[i].0,
                }
                acc.pop();
                i -= 1;
            }
            '+' => acc[i].1 = false,
            '*' => acc[i].1 = true,
            ' ' => (),
            n => {
                let value = n.to_digit(10).unwrap() as u64;
                match acc[i].1 {
                    false => acc[i].0 += value,
                    true => acc[i].0 *= value,
                }
            }
        }
    }
    acc[0].0
}

fn eval_two(expr: impl AsRef<str>) -> u64 {
    let mut acc = vec![(0, false)];
    let mut i = 0;
    for c in expr.as_ref().chars() {
        match c {
            '(' => {
                acc.push((0, false));
                i += 1;
            }
            ')' => {
                if acc[i].1 {
                    acc[i - 1].0 *= acc[i].0;
                    acc.pop();
                    i -= 1;
                }
                acc[i - 1].0 += acc[i].0;
                acc.pop();
                i -= 1;
            }
            '*' => {
                if acc[i].1 {
                    acc[i - 1].0 *= acc[i].0;
                    acc[i].0 = 0;
                } else {
                    acc.push((0, true));
                    i += 1;
                }
            }
            '+' | ' ' => (),
            n => {
                acc[i].0 += n.to_digit(10).unwrap() as u64;
            }
        }
    }
    if acc[i].1 {
        acc[0].0 * acc[1].0
    } else {
        acc[0].0
    }
}
