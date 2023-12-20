use std::collections::VecDeque;

use aoc::Parse;
use rustc_hash::FxHashMap as HashMap;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut modules = Modules::parse(input);
    for _ in 0..1000 {
        modules.push_button();
    }
    modules.pulses.low * modules.pulses.high
}

fn part_2(input: aoc::Input) -> u64 {
    let mut modules = Modules::parse(input);
    while modules.target_memory.len() < modules.target_len {
        modules.push_button();
    }
    modules.target_memory.values().product()
}

struct Modules {
    modules: Vec<Module>,
    broadcaster: usize,
    pulses: Pulses,

    // part 2
    count: u64,
    target: usize,
    target_memory: HashMap<usize, u64>,
    target_len: usize,
}

impl Modules {
    fn parse(input: aoc::Input) -> Self {
        let mut names = Names::new();
        let mut modules = vec![Module::default(); input.len() + 1];
        let mut target = 0;
        for line in input {
            let (name, dests) = line.split_once(" -> ").unwrap();
            let (kind, name) = match name.idx(0) {
                b'%' => (ModuleKind::FlipFlop(State::Off), &name[1..]),
                b'&' => (ModuleKind::Conjunction(HashMap::default()), &name[1..]),
                _ => {
                    debug_assert!(name == "broadcaster");
                    (ModuleKind::Broadcaster, name)
                }
            };
            let i = names.get(name);
            let dests: Vec<_> = dests.split(", ").map(|name| names.get(name)).collect();
            if dests.len() == 1 && dests[0] == names.get("rx") {
                target = i;
            }
            modules[i] = Module { kind, dests };
        }
        let broadcaster = names.get("broadcaster");

        modules.resize(names.names.len(), Module::default());

        for i in 0..modules.len() {
            for j in 0..modules[i].dests.len() {
                let dest = modules[i].dests[j];
                if let ModuleKind::Conjunction(memory) = &mut modules[dest].kind {
                    memory.insert(i, Strength::Low);
                }
            }
        }

        let target_len = if let ModuleKind::Conjunction(memory) = &modules[target].kind {
            memory.len()
        } else {
            unreachable!()
        };

        Self {
            modules,
            broadcaster,
            pulses: Pulses::new(),
            count: 0,
            target,
            target_memory: HashMap::default(),
            target_len,
        }
    }

    fn push_button(&mut self) {
        self.count += 1;
        self.pulses.send(Pulse::low(0, self.broadcaster));

        while let Some(pulse) = self.pulses.queue.pop_front() {
            let module = &mut self.modules[pulse.receiver];
            let sender = pulse.receiver;
            match &mut module.kind {
                ModuleKind::Broadcaster => {
                    for &dest in &module.dests {
                        self.pulses.send(pulse.relay(sender, dest));
                    }
                }
                ModuleKind::FlipFlop(state) => {
                    if let Strength::Low = pulse.strength {
                        match state {
                            State::Off => {
                                *state = State::On;
                                for &dest in &module.dests {
                                    self.pulses.send(Pulse::high(sender, dest));
                                }
                            }
                            State::On => {
                                *state = State::Off;
                                for &dest in &module.dests {
                                    self.pulses.send(Pulse::low(sender, dest));
                                }
                            }
                        }
                    }
                }
                ModuleKind::Conjunction(memory) => {
                    if pulse.receiver == self.target && pulse.strength == Strength::High {
                        self.target_memory.entry(pulse.sender).or_insert(self.count);
                    }
                    memory.insert(pulse.sender, pulse.strength);
                    let strength = if memory.values().all(|&s| s == Strength::High) {
                        Strength::Low
                    } else {
                        Strength::High
                    };
                    for &dest in &module.dests {
                        self.pulses.send(Pulse::new(sender, dest, strength));
                    }
                }
            }
        }
    }
}

struct Pulses {
    queue: VecDeque<Pulse>,
    low: u64,
    high: u64,
}

impl Pulses {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            low: 0,
            high: 0,
        }
    }

    fn send(&mut self, pulse: Pulse) {
        match pulse.strength {
            Strength::Low => self.low += 1,
            Strength::High => self.high += 1,
        }
        self.queue.push_back(pulse);
    }
}

#[derive(Default, Clone)]
struct Module {
    kind: ModuleKind,
    dests: Vec<usize>,
}

#[derive(Default, Clone)]
enum ModuleKind {
    #[default]
    Broadcaster,
    FlipFlop(State),
    Conjunction(HashMap<usize, Strength>),
}

#[derive(Clone, Copy)]
struct Pulse {
    sender: usize,
    receiver: usize,
    strength: Strength,
}

impl Pulse {
    fn new(sender: usize, receiver: usize, strength: Strength) -> Self {
        Self {
            sender,
            receiver,
            strength,
        }
    }

    fn relay(self, sender: usize, receiver: usize) -> Self {
        Self {
            sender,
            receiver,
            strength: self.strength,
        }
    }

    fn low(sender: usize, receiver: usize) -> Self {
        Self {
            sender,
            receiver,
            strength: Strength::Low,
        }
    }

    fn high(sender: usize, receiver: usize) -> Self {
        Self {
            sender,
            receiver,
            strength: Strength::High,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Strength {
    Low,
    High,
}

#[derive(Default, Clone, Copy)]
enum State {
    #[default]
    Off,
    On,
}

struct Names<'a> {
    names: HashMap<&'a str, usize>,
}

impl<'a> Names<'a> {
    fn new() -> Self {
        Self {
            names: HashMap::default(),
        }
    }

    fn get(&mut self, name: &'a str) -> usize {
        let len = self.names.len();
        *self.names.entry(name).or_insert(len)
    }
}
