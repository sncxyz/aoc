use std::collections::BTreeMap;

use aoc::{Input, IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let guard = parse(input).max_by_key(|x| x.1.total_mins).unwrap();
    guard.0 * guard.1.modal_min().0
}

fn part_2(input: Input) -> impl ToString {
    let guard = parse(input)
        .map(|(i, g)| (i, g.modal_min()))
        .max_by_key(|x| x.1 .1)
        .unwrap();
    guard.0 * guard.1 .0
}

fn parse(input: Input) -> impl Iterator<Item = (u32, Guard)> {
    let mut records: Vec<_> = input.lines().map(Record::new).collect();
    records.sort_unstable_by_key(|r| r.date_time);

    let mut guards = BTreeMap::default();
    let mut guard_id = 0;
    let mut from = 0;

    for record in records {
        match record.event {
            Event::Asleep => from = record.date_time.minute,
            Event::Awake => guards
                .entry(guard_id)
                .or_insert_with(|| Guard::new())
                .sleeps(from, record.date_time.minute),
            Event::BeginShift(id) => guard_id = id,
        }
    }

    guards.into_iter()
}

struct Guard {
    min_counts: [u16; 60],
    total_mins: u32,
}

impl Guard {
    #[inline]
    fn new() -> Self {
        Self {
            min_counts: [0; 60],
            total_mins: 0,
        }
    }

    fn sleeps(&mut self, from: u8, to: u8) {
        self.total_mins += (to - from) as u32;
        for minute in from..to {
            self.min_counts[minute as usize] += 1;
        }
    }

    fn modal_min(self) -> (u32, u16) {
        self.min_counts
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i as u32, c))
            .max_by_key(|x| x.1)
            .unwrap()
    }
}

struct Record {
    date_time: DateTime,
    event: Event,
}

impl Record {
    fn new(line: &str) -> Self {
        let mut parser = line.as_parser();
        let date_time = DateTime::new(&mut parser);
        let event = Event::new(parser);
        Self { date_time, event }
    }
}

enum Event {
    Asleep,
    Awake,
    BeginShift(u32),
}

impl Event {
    fn new(parser: aoc::Parser) -> Self {
        match parser.after(" ") {
            "falls asleep" => Self::Asleep,
            "wakes up" => Self::Awake,
            s => Self::BeginShift(s.uints_iter().next_uw()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl DateTime {
    fn new(parser: &mut aoc::Parser) -> Self {
        Self {
            year: parser.between("[", "-").parse_uw(),
            month: parser.before("-").parse_uw(),
            day: parser.before(" ").parse_uw(),
            hour: parser.before(":").parse_uw(),
            minute: parser.before("]").parse_uw(),
        }
    }
}
