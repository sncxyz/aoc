aoc::parts!(1, 2);

fn part_1(input: &[&str]) -> impl ToString {
    let mut game = Game::new(
        input[0]
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect(),
    );
    for _ in 0..100 {
        game.update(3);
    }
    game.result_one()
}

fn part_2(input: &[&str]) -> impl ToString {
    let mut values: Vec<_> = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    for x in values.len() + 1..=1000000 {
        values.push(x);
    }
    let mut game = Game::new(values);
    for _ in 0..10000000 {
        game.update(3);
    }
    game.result_two()
}

struct Game {
    cups: Vec<usize>,
    current: usize,
}

impl Game {
    fn new(values: Vec<usize>) -> Game {
        let mut cups = Vec::new();
        cups.resize_with(values.len() + 1, || 0);
        for i in 0..values.len() - 1 {
            cups[values[i]] = values[i + 1];
        }
        cups[values[values.len() - 1]] = values[0];
        Game {
            cups,
            current: values[0],
        }
    }

    fn update(&mut self, n: usize) {
        let mut to_move = vec![self.cups[self.current]];
        for i in 0..n - 1 {
            to_move.push(self.cups[to_move[i]]);
        }
        let mut destination = if self.current == 1 {
            self.cups.len() - 1
        } else {
            self.current - 1
        };
        while to_move.contains(&destination) {
            destination = if destination == 1 {
                self.cups.len() - 1
            } else {
                destination - 1
            };
        }
        self.cups[self.current] = self.cups[to_move[n - 1]];
        self.cups[to_move[n - 1]] = self.cups[destination];
        self.cups[destination] = to_move[0];
        self.current = self.cups[self.current];
    }

    fn result_one(&self) -> String {
        let mut result = String::new();
        let mut current = 1;
        for _ in 0..self.cups.len() - 2 {
            current = self.cups[current];
            result += &current.to_string();
        }
        result
    }

    fn result_two(&self) -> usize {
        self.cups[1] * self.cups[self.cups[1]]
    }
}
