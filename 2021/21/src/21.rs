aoc::parts!(1, 2);

use rustc_hash::FxHashMap as HashMap;

fn part_1(input: &[&str]) -> impl ToString {
    let mut one = DeterministicPlayer::new(&input[0], 6);
    let mut two = DeterministicPlayer::new(&input[1], 5);
    let mut rolls = 0;
    (loop {
        one.update();
        rolls += 3;
        if one.score >= 1000 {
            break two.score;
        }
        two.update();
        rolls += 3;
        if two.score >= 1000 {
            break one.score;
        }
    }) * rolls
}

fn part_2(input: &[&str]) -> impl ToString {
    let one = Player::new(&input[0]);
    let two = Player::new(&input[1]);
    let mut success_one = HashMap::default();
    let mut success_two = HashMap::default();
    let mut failure_one = HashMap::default();
    let mut failure_two = HashMap::default();
    populate(one, &mut success_one, &mut failure_one, 0, 1);
    populate(two, &mut success_two, &mut failure_two, 0, 1);

    let mut wins_one = 0;
    for (&turns, &universes) in success_one.iter() {
        wins_one += universes * failure_two[&(turns - 1)];
    }
    let mut wins_two = 0;
    for (turns, &universes) in success_two.iter() {
        wins_two += universes * failure_one.get(turns).unwrap_or(&0);
    }

    wins_one.max(wins_two)
}

struct DeterministicPlayer {
    space: u32,
    score: u32,
    offset: u32,
}

impl DeterministicPlayer {
    fn new(line: &str, offset: u32) -> DeterministicPlayer {
        DeterministicPlayer {
            space: line.chars().last().unwrap().to_string().parse().unwrap(),
            score: 0,
            offset,
        }
    }

    fn update(&mut self) {
        self.space = (self.space + self.offset - 1) % 10 + 1;
        self.score += self.space;
        self.offset = (self.offset + 8) % 10;
    }
}

fn populate(
    player: Player,
    success: &mut HashMap<u8, u64>,
    failure: &mut HashMap<u8, u64>,
    turns: u8,
    universes: u64,
) {
    if player.score >= 21 {
        *success.entry(turns).or_insert(0) += universes;
        return;
    } else {
        *failure.entry(turns).or_insert(0) += universes;
    }
    populate(player.update(3), success, failure, turns + 1, universes);
    populate(player.update(4), success, failure, turns + 1, universes * 3);
    populate(player.update(5), success, failure, turns + 1, universes * 6);
    populate(player.update(6), success, failure, turns + 1, universes * 7);
    populate(player.update(7), success, failure, turns + 1, universes * 6);
    populate(player.update(8), success, failure, turns + 1, universes * 3);
    populate(player.update(9), success, failure, turns + 1, universes);
}

struct Player {
    space: u8,
    score: u8,
}

impl Player {
    fn new(line: &str) -> Player {
        Player {
            space: line.chars().last().unwrap().to_string().parse().unwrap(),
            score: 0,
        }
    }

    fn update(&self, offset: u8) -> Player {
        let space = (self.space + offset - 1) % 10 + 1;
        Player {
            space,
            score: self.score + space,
        }
    }
}
