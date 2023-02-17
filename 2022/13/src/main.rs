aoc::parts!(1, 2);

use std::cmp::Ordering;

fn part_1(input: &[&str]) -> usize {
    input
        .split(|line| line.is_empty())
        .enumerate()
        .filter(|(_, pair)| Packet::parse(pair[0]) < Packet::parse(pair[1]))
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_2(input: &[&str]) -> usize {
    let mut packets: Vec<_> = input
        .iter()
        .filter(|line| !line.is_empty())
        .map(Packet::parse)
        .collect();
    let dividers = ["[[2]]", "[[6]]"].map(Packet::parse);
    packets.extend(dividers.clone());
    packets.sort_unstable();
    dividers
        .into_iter()
        .map(|d| packets.binary_search(&d).unwrap() + 1)
        .product()
}

#[derive(Clone)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl Packet {
    fn parse(line: impl AsRef<str>) -> Self {
        Self::parse_list(line.as_ref(), &mut 1)
    }

    fn parse_list(line: &str, i: &mut usize) -> Self {
        let bytes = line.as_bytes();
        let mut list = Vec::new();
        while bytes[*i] != b']' {
            match bytes[*i] {
                b'[' => {
                    *i += 1;
                    list.push(Self::parse_list(line, i));
                    *i += 1;
                }
                b',' => *i += 1,
                _ => {
                    let j = *i;
                    while bytes[*i] != b',' && bytes[*i] != b']' {
                        *i += 1;
                    }
                    list.push(Self::Value(line[j..*i].parse().unwrap()));
                }
            }
        }
        Self::List(list)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use Packet::*;
        match (self, other) {
            (Value(l), Value(r)) => l.cmp(r),
            (Value(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(_), Value(_)) => self.cmp(&List(vec![other.clone()])),
            (List(l), List(r)) => {
                let mut i = 0;
                loop {
                    match (l.len() == i, r.len() == i) {
                        (true, true) => break Equal,
                        (true, false) => break Less,
                        (false, true) => break Greater,
                        _ => match l[i].cmp(&r[i]) {
                            Equal => i += 1,
                            ord => break ord,
                        },
                    }
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}
