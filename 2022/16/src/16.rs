aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;
use std::collections::BinaryHeap;

fn part_1(input: &[&str]) -> impl ToString {
    let mut max = 0;
    Valves::parse(input).max_pressure(0, 30, 0, 0, &mut max);
    max
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut cache = HashMap::default();
    Valves::parse(input).set_pressure(0, 26, 0, 0, &mut cache);
    let mut pressures: Vec<_> = cache.into_iter().collect();
    pressures.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let mut pairs = BinaryHeap::with_capacity(pressures.len() - 2);
    let mut current = Pair::new(0, 1, pressures[0].1 + pressures[1].1);
    let mut next = Pair::new(1, 2, pressures[1].1 + pressures[2].1);
    for i in 2..pressures.len() - 1 {
        pairs.push(Pair::new(i, i + 1, pressures[i].1 + pressures[i + 1].1));
    }
    loop {
        while current.pressure >= next.pressure {
            if pressures[current.i].0 & pressures[current.j].0 == 0 {
                return current.pressure;
            }
            current.pressure -= pressures[current.j].1 - pressures[current.j + 1].1;
            current.j += 1;
        }
        pairs.push(current);
        current = next;
        next = pairs.pop().unwrap();
    }
}

struct Valves {
    rates: [u32; 32],
    edges: [[u32; 32]; 32],
    len: usize,
}

impl Valves {
    fn parse(input: &[&str]) -> Self {
        let mut flow_rates = [0; 26 * 26];
        let mut adjacent = [(); 26 * 26].map(|_| Vec::new());
        let mut valves = Vec::new();
        for line in input {
            let bytes = line.as_bytes();
            let valve = to_index(bytes[6], bytes[7]);
            valves.push(valve);
            let i = bytes.iter().position(|b| *b == b';').unwrap();
            flow_rates[valve] = line[23..i].parse().unwrap();
            adjacent[valve] = if bytes[i + 8] == b' ' {
                &line[i + 24..]
            } else {
                &line[i + 25..]
            }
            .split(", ")
            .map(str::as_bytes)
            .map(|s| to_index(s[0], s[1]))
            .collect();
        }
        let mut working = vec![0];
        let mut working_edges = [(); 26 * 26].map(|_| Vec::new());
        for &valve in &valves {
            if valve == 0 || flow_rates[valve] > 0 {
                if valve != 0 {
                    working.push(valve);
                }
                let search = search::bft(
                    (valve, 0),
                    |&(v, c)| adjacent[v].iter().map(move |u| (*u, c + 1)),
                    |(v, _)| *v,
                );
                working_edges[valve] = search
                    .skip(1)
                    .filter_map(|(v, c)| (flow_rates[v] > 0).then_some((v, c + 1)))
                    .collect();
            }
        }
        let len = working.len();
        let mut rates = [0; 32];
        let mut index_map = [0; 26 * 26];
        for (i, &valve) in working.iter().enumerate() {
            index_map[valve] = i;
            rates[i] = flow_rates[valve];
        }
        let mut edges = [[0; 32]; 32];
        for (i, &valve) in working.iter().enumerate() {
            for &(other, cost) in &working_edges[valve] {
                edges[i][index_map[other]] = cost;
            }
        }
        Self { rates, edges, len }
    }

    fn max_pressure(&self, valve: usize, time: u32, pressure: u32, set: u32, max: &mut u32) {
        for u in 1..self.len {
            let cost = self.edges[valve][u];
            if cost < time && set & (1 << (u - 1)) == 0 {
                let time = time - cost;
                let pressure = pressure + time * self.rates[u];
                self.max_pressure(u, time, pressure, set | (1 << (u - 1)), max);
            }
        }
        *max = (*max).max(pressure);
    }

    fn set_pressure(
        &self,
        valve: usize,
        time: u32,
        pressure: u32,
        set: u32,
        cache: &mut HashMap<u32, u32>,
    ) {
        if let Some(highest) = cache.get_mut(&set) {
            *highest = pressure.max(*highest);
        } else {
            cache.insert(set, pressure);
        }
        for u in 1..self.len {
            let cost = self.edges[valve][u];
            if cost < time && set & (1 << (u - 1)) == 0 {
                let time = time - cost;
                let pressure = pressure + time * self.rates[u];
                self.set_pressure(u, time, pressure, set | (1 << (u - 1)), cache);
            }
        }
    }
}

fn to_index(a: u8, b: u8) -> usize {
    (a - b'A') as usize + 26 * (b - b'A') as usize
}

struct Pair {
    i: usize,
    j: usize,
    pressure: u32,
}

impl Pair {
    fn new(i: usize, j: usize, pressure: u32) -> Self {
        Self { i, j, pressure }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pressure.cmp(&other.pressure)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Pair {}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.pressure == other.pressure
    }
}
