aoc::parts!(1, 2);

use Bit::*;

fn part_1(input: &[&str]) -> impl ToString {
    Packet::new(input).total_version()
}

fn part_2(input: &[&str]) -> impl ToString {
    Packet::new(input).evaluate()
}

struct Packet {
    version: u64,
    contents: Contents,
}

impl Packet {
    fn new(input: &[&str]) -> Packet {
        Packet::parse(
            &input[0].chars().flat_map(parse_hex).collect::<Vec<_>>(),
            &mut 0,
        )
    }

    fn parse(bits: &[Bit], i: &mut usize) -> Packet {
        let version = parse_value(bits, i, 3);
        let type_id = parse_value(bits, i, 3);

        if type_id == 4 {
            Packet {
                version,
                contents: Contents::Literal(parse_literal(bits, i)),
            }
        } else {
            let mut sub_packets = Vec::new();
            if let Zero = get_next(bits, i) {
                let length = parse_value(bits, i, 15);
                let end = *i + length as usize;
                while *i < end {
                    sub_packets.push(Packet::parse(bits, i));
                }
            } else {
                let number = parse_value(bits, i, 11);
                for _ in 0..number {
                    sub_packets.push(Packet::parse(bits, i));
                }
            }
            Packet {
                version,
                contents: Contents::Operator(Operator {
                    type_id,
                    sub_packets,
                }),
            }
        }
    }

    fn total_version(&self) -> u64 {
        self.version
            + if let Contents::Operator(op) = &self.contents {
                op.sub_packets
                    .iter()
                    .map(|packet| packet.total_version())
                    .sum()
            } else {
                0
            }
    }

    fn evaluate(&self) -> u64 {
        match &self.contents {
            Contents::Literal(value) => *value,
            Contents::Operator(op) => match op.type_id {
                id @ 0..=3 => {
                    let packets = op.sub_packets.iter().map(|packet| packet.evaluate());
                    match id {
                        0 => packets.sum(),
                        1 => packets.product(),
                        2 => packets.min().unwrap(),
                        3 => packets.max().unwrap(),
                        _ => unreachable!(),
                    }
                }
                id @ 5..=7 => {
                    let (one, two) = (op.sub_packets[0].evaluate(), op.sub_packets[1].evaluate());
                    (match id {
                        5 => one > two,
                        6 => one < two,
                        7 => one == two,
                        _ => unreachable!(),
                    }) as u64
                }
                _ => panic!(),
            },
        }
    }
}

enum Contents {
    Literal(u64),
    Operator(Operator),
}

struct Operator {
    type_id: u64,
    sub_packets: Vec<Packet>,
}

enum Bit {
    Zero,
    One,
}

fn parse_literal(bits: &[Bit], i: &mut usize) -> u64 {
    let mut value = 0;
    loop {
        let first = get_next(bits, i);
        value <<= 4;
        value += parse_value(bits, i, 4);
        if let Zero = first {
            break value;
        }
    }
}

fn parse_value(bits: &[Bit], i: &mut usize, length: u64) -> u64 {
    let mut value = 0;
    for _ in 0..length {
        value <<= 1;
        if let One = bits[*i] {
            value += 1;
        }
        *i += 1;
    }
    value
}

fn get_next<'a>(bits: &'a [Bit], i: &mut usize) -> &'a Bit {
    *i += 1;
    &bits[*i - 1]
}

fn parse_hex(hex: char) -> [Bit; 4] {
    match hex {
        '0' => [Zero, Zero, Zero, Zero],
        '1' => [Zero, Zero, Zero, One],
        '2' => [Zero, Zero, One, Zero],
        '3' => [Zero, Zero, One, One],
        '4' => [Zero, One, Zero, Zero],
        '5' => [Zero, One, Zero, One],
        '6' => [Zero, One, One, Zero],
        '7' => [Zero, One, One, One],
        '8' => [One, Zero, Zero, Zero],
        '9' => [One, Zero, Zero, One],
        'A' => [One, Zero, One, Zero],
        'B' => [One, Zero, One, One],
        'C' => [One, One, Zero, Zero],
        'D' => [One, One, Zero, One],
        'E' => [One, One, One, Zero],
        'F' => [One, One, One, One],
        _ => panic!(),
    }
}
