aoc::parts!(1, 2);

use grid::prelude::*;
use std::collections::{BTreeSet, VecDeque};

fn part_1(input: &[&str]) -> impl ToString {
    let mut network = Network::new(input);
    while network.nat == v(-1, 0) {
        network.update();
    }
    network.nat.y
}

fn part_2(input: &[&str]) -> i64 {
    let mut network = Network::new(input);
    let mut history = BTreeSet::new();
    loop {
        if let Some(nat) = network.update() {
            if history.contains(&nat.y) {
                return nat.y;
            }
            history.insert(nat.y);
        }
    }
}

struct Network {
    computers: [intcode::Computer; 50],
    packets: [VecDeque<Vector>; 50],
    nat: Vector,
}

impl Network {
    fn new(input: &[&str]) -> Network {
        let mut computer = intcode::Computer::new(&input[0]).unwrap();
        computer.run();
        let mut computers = [(); 50].map(|_| computer.clone());
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.input(i as i64);
        }
        Network {
            computers,
            packets: [(); 50].map(|_| VecDeque::new()),
            nat: v(-1, 0),
        }
    }

    fn update(&mut self) -> Option<Vector> {
        let mut updated = false;
        for (i, computer) in self.computers.iter_mut().enumerate() {
            if computer.state() == intcode::State::Input {
                if let Some(pos) = self.packets[i].pop_front() {
                    updated = true;
                    computer.input(pos.x);
                    computer.input(pos.y);
                } else {
                    computer.input(-1);
                }
            } else if computer.state() == intcode::State::Output {
                let j = computer.output().unwrap() as usize;
                let packet = Vector::new(computer.output().unwrap(), computer.output().unwrap());
                if j == 255 {
                    self.nat = packet;
                } else {
                    updated = true;
                    self.packets[j].push_back(packet);
                }
            }
        }
        if updated {
            return None;
        }
        self.packets[0].push_back(self.nat);
        Some(self.nat)
    }
}
