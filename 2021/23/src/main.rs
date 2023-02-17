aoc::parts!(1, 2);

use search::dijkstra;
use Colour::*;

fn part_1(input: &[&str]) -> impl ToString {
    let (one, two) = parse(input);
    lowest_energy([
        [A, A, two[0], one[0]],
        [B, B, two[1], one[1]],
        [C, C, two[2], one[2]],
        [D, D, two[3], one[3]],
    ])
}

fn part_2(input: &[&str]) -> impl ToString {
    let (one, four) = parse(input);
    lowest_energy([
        [four[0], D, D, one[0]],
        [four[1], B, C, one[1]],
        [four[2], A, B, one[2]],
        [four[3], C, A, one[3]],
    ])
}

fn parse(input: &[&str]) -> ([Colour; 4], [Colour; 4]) {
    (parse_line(&input[2]), parse_line(&input[3]))
}

fn parse_line(line: &str) -> [Colour; 4] {
    let mut chars = line.chars();
    [
        chars.nth(3).unwrap(),
        chars.nth(1).unwrap(),
        chars.nth(1).unwrap(),
        chars.nth(1).unwrap(),
    ]
    .map(Colour::new)
}

fn lowest_energy(rooms: [[Colour; 4]; 4]) -> u32 {
    dijkstra(
        Burrow::new(rooms),
        Burrow::legal_moves,
        Burrow::normalise,
        |b| b.energy,
    )
    .find(Burrow::complete)
    .unwrap()
    .energy
}

#[derive(Clone)]
struct Burrow {
    hallway: [Option<Colour>; 11],
    rooms: [Room; 4],
    energy: u32,
}

impl Burrow {
    fn new(rooms: [[Colour; 4]; 4]) -> Burrow {
        let mut colours = [A, B, C, D].into_iter();
        Burrow {
            hallway: [None; 11],
            rooms: rooms.map(|r| Room::new(colours.next().unwrap(), r)),
            energy: 0,
        }
    }

    fn legal_moves(&self) -> Vec<Burrow> {
        let mut moves = Vec::new();

        for pos in 0..11 {
            if let Some(colour) = self.hallway[pos] {
                if self.rooms[colour.room_index()].valid()
                    && self.clear_path(pos, colour.room_position())
                {
                    moves.push(self.hallway_move(pos));
                }
            }
        }

        for i in 0..4 {
            if self.rooms[i].count > 0 && !self.rooms[i].valid() {
                for pos in 0..11 {
                    if is_room_pos(pos) {
                        continue;
                    }
                    let j = (i + 1) * 2;
                    if self.hallway[j].is_none() && self.clear_path(j, pos) {
                        moves.push(self.room_move(i, pos));
                    }
                }
            }
        }

        moves
    }

    fn hallway_move(&self, pos: usize) -> Burrow {
        let mut new = self.clone();
        let colour = new.hallway[pos].unwrap();
        let room = &mut new.rooms[colour.room_index()];
        new.hallway[pos] = None;
        new.energy +=
            (colour.room_position().abs_diff(pos) + 4 - room.count) as u32 * colour.energy_cost();
        room.push();
        new
    }

    fn room_move(&self, i: usize, pos: usize) -> Burrow {
        let mut new = self.clone();
        let colour = new.rooms[i].pop();
        new.hallway[pos] = Some(colour);
        new.energy +=
            (pos.abs_diff((i + 1) * 2) + 4 - new.rooms[i].count) as u32 * colour.energy_cost();
        new
    }

    fn clear_path(&self, from: usize, to: usize) -> bool {
        let (start, end) = if to > from {
            (from + 1, to)
        } else {
            (to, from - 1)
        };
        for pos in start..=end {
            if self.hallway[pos].is_some() {
                return false;
            }
        }
        true
    }

    fn normalise(&self) -> u64 {
        let mut value = 0;
        for pos in 0..11 {
            if !is_room_pos(pos) {
                value *= 5;
                if let Some(colour) = self.hallway[pos] {
                    value += colour.room_index() as u64 + 1;
                }
            }
        }
        for room in self.rooms.iter() {
            value *= 625;
            value += room.id();
        }
        value
    }

    fn complete(&self) -> bool {
        for room in &self.rooms {
            if room.count < 4 || !room.valid() {
                return false;
            }
        }
        true
    }
}

fn is_room_pos(pos: usize) -> bool {
    pos == 2 || pos == 4 || pos == 6 || pos == 8
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Colour {
    A,
    B,
    C,
    D,
}

impl Colour {
    fn new(c: char) -> Colour {
        match c {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            _ => panic!(),
        }
    }

    fn energy_cost(&self) -> u32 {
        match &self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn room_position(&self) -> usize {
        match &self {
            A => 2,
            B => 4,
            C => 6,
            D => 8,
        }
    }

    fn room_index(&self) -> usize {
        match &self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }
}

#[derive(Clone)]
struct Room {
    colour: Colour,
    amphipods: [Colour; 4],
    count: usize,
}

impl Room {
    fn new(colour: Colour, amphipods: [Colour; 4]) -> Room {
        Room {
            colour,
            amphipods,
            count: 4,
        }
    }

    fn push(&mut self) {
        self.amphipods[self.count] = self.colour;
        self.count += 1;
    }

    fn pop(&mut self) -> Colour {
        self.count -= 1;
        self.amphipods[self.count]
    }

    fn valid(&self) -> bool {
        for i in 0..self.count {
            if self.amphipods[i] != self.colour {
                return false;
            }
        }
        true
    }

    fn id(&self) -> u64 {
        let mut value = 0;
        for i in 0..self.count {
            value *= 5;
            value += self.amphipods[i].room_index() as u64 + 1;
        }
        value
    }
}
