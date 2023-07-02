aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    let (mut twos, mut threes) = (0, 0);
    for line in input {
        let mut counts = [0; 26];
        for letter in line.bytes() {
            counts[(letter - b'a') as usize] += 1;
        }
        let (mut has_two, mut has_three) = (false, false);
        for count in counts {
            if count == 2 {
                has_two = true;
            } else if count == 3 {
                has_three = true;
            }
            if has_two && has_three {
                break;
            }
        }
        twos += has_two as u32;
        threes += has_three as u32;
    }
    twos * threes
}

fn part_2(input: &[&str]) -> impl ToString {
    find_similar(input).unwrap()
}

fn find_similar(ids: &[&str]) -> Option<String> {
    if ids.len() <= 8 {
        for (i, &a) in ids.iter().enumerate() {
            for &b in ids.iter().skip(i + 1) {
                let mut differs_at = None;
                for (i, letter) in a.bytes().enumerate() {
                    if letter != b.as_bytes()[i] {
                        if differs_at.is_some() {
                            differs_at = None;
                            break;
                        }
                        differs_at = Some(i);
                    }
                }
                if let Some(i) = differs_at {
                    return Some(String::from(&a[..i]) + &a[i + 1..]);
                }
            }
        }
        return None;
    }
    let prefix_len = (ids[0].len() + 1) / 2;
    let mut prefix_map = HashMap::default();
    for &id in ids {
        prefix_map
            .entry(&id[..prefix_len])
            .or_insert(Vec::new())
            .push(&id[prefix_len..]);
    }
    for (prefix, ids) in prefix_map {
        if ids.len() > 1 {
            if let Some(s) = find_similar(&ids) {
                return Some(String::from(prefix) + &s);
            }
        }
    }
    let mut postfix_map = HashMap::default();
    for &id in ids {
        postfix_map
            .entry(&id[prefix_len..])
            .or_insert(Vec::new())
            .push(&id[..prefix_len]);
    }
    for (postfix, ids) in postfix_map {
        if ids.len() > 1 {
            if let Some(s) = find_similar(&ids) {
                return Some(s + postfix);
            }
        }
    }
    None
}
