aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> u32 {
    parse(input)
        .map(|(id, costs)| id * blueprint_geodes(costs, 24))
        .sum()
}

fn part_2(input: &[&str]) -> u32 {
    parse(input)
        .take(3)
        .map(|b| blueprint_geodes(b.1, 32))
        .product()
}

fn blueprint_geodes(costs: [[u32; 4]; 4], time: u32) -> u32 {
    let mut max = 0;
    most_geodes(costs, [0; 4], [1, 0, 0, 0], time, &mut max);
    max
}

fn most_geodes(costs: [[u32; 4]; 4], amounts: [u32; 4], rates: [u32; 4], time: u32, max: &mut u32) {
    let mut geode_time = None;
    'outer: for i in (0..4).rev() {
        let mut t = 0;
        for j in 0..4 {
            if costs[i][j] <= amounts[j] {
                continue;
            }
            let cost = costs[i][j] - amounts[j];
            if rates[j] == 0 {
                continue 'outer;
            }
            t = t.max((cost - 1) / rates[j] + 1);
        }
        if t + 1 < time {
            if let Some(g) = geode_time {
                if t >= g {
                    continue;
                }
            } else if i == 3 {
                geode_time = Some(t);
            }
            let mut amounts = amounts;
            let mut rates = rates;
            for j in 0..4 {
                amounts[j] += rates[j] * (t + 1);
                amounts[j] -= costs[i][j];
            }
            rates[i] += 1;
            let time = time - t - 1;
            let mut t = time;
            if t > 1 {
                let mut afford_geode = true;
                for j in 0..4 {
                    if amounts[j] < costs[3][j] {
                        afford_geode = false;
                        break;
                    }
                }
                if !afford_geode {
                    t -= 1;
                    if t > 1 {
                        afford_geode = true;
                        for j in 0..4 {
                            if amounts[j] + rates[j] < costs[3][j] {
                                afford_geode = false;
                                break;
                            }
                        }
                        if !afford_geode {
                            t -= 1;
                        }
                    }
                }
            }
            if amounts[3] + rates[3] * time + ((t * (t - 1)) / 2) > *max {
                most_geodes(costs, amounts, rates, time, max);
            }
        }
    }
    *max = (*max).max(amounts[3] + rates[3] * time);
}

fn parse<'a>(input: &'a [&str]) -> impl Iterator<Item = (u32, [[u32; 4]; 4])> + 'a {
    input.iter().map(|line| {
        let id = line[10..].split_once(':').unwrap().0.parse().unwrap();
        let mut parts = line[10..].split(' ');
        let ore_ore = parts.nth(5).unwrap().parse().unwrap();
        let clay_ore = parts.nth(5).unwrap().parse().unwrap();
        let obsidian_ore = parts.nth(5).unwrap().parse().unwrap();
        let obsidian_clay = parts.nth(2).unwrap().parse().unwrap();
        let geode_ore = parts.nth(5).unwrap().parse().unwrap();
        let geode_obsidian = parts.nth(2).unwrap().parse().unwrap();
        (
            id,
            [
                [ore_ore, 0, 0, 0],
                [clay_ore, 0, 0, 0],
                [obsidian_ore, obsidian_clay, 0, 0],
                [geode_ore, 0, geode_obsidian, 0],
            ],
        )
    })
}
